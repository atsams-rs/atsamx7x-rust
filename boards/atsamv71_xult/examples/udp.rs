#![no_main]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};



use panic_rtt_target as _;


#[rtic::app(device = atsamx7x_hal::pac, dispatchers = [PIOA, PIOB])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::{Tokens, PllaConfig, Pck, Pck0, Pck4, HostClockController, HostClockConfig, HccPrescaler, MckDivider, PeripheralIdentifier};
    use hal::pio::bank::{BankA, BankB, BankD, BankConfiguration, PA25};
    use hal::pio::{Pin,A};
    use hal::pio::pin::Peripheral;
    use hal::serial::ExtBpsU32;
    use hal::generics::events::EventHandler;
    use hal::fugit::{ExtU32, RateExtU32};
    use hal::rtt::*;
    use heapless::String;
    use core::str;
    use rtt_target::{rprintln, rtt_init_print};
    use hal::gmac::*;
    use cortex_m::singleton;
    use smoltcp::iface::{Neighbor, InterfaceBuilder, SocketStorage, NeighborCache, Interface, Route, Routes, SocketHandle};
    use smoltcp::phy::{Device, RxToken, TxToken};
    use smoltcp::socket::{TcpSocketBuffer, TcpSocket, Dhcpv4Event, Dhcpv4Socket, UdpSocketBuffer, UdpSocket, UdpPacketMetadata};
    use smoltcp::time::Instant;
    use smoltcp::wire::{IpCidr, IpAddress, EthernetAddress, HardwareAddress, Ipv4Address, Ipv4Cidr, IpEndpoint};
    // TODO: Add a monotonic if scheduling will be used
    use dwt_systick_monotonic::*;
    #[monotonic(binds = SysTick, default = true)]
    type DwtMono = DwtSystick<150_000_000>;

    // Shared resources go here
    #[shared]
    struct Shared {
    }

    // Local resources go here
    #[local]
    struct Local {
        iface: Interface<'static, Gmac>,
        udp_handle: SocketHandle,
        dhcp_handle: SocketHandle,
    }
    

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let device = cx.device;
        let pioa = device.PIOA;
        let piob = device.PIOB;
        let piod = device.PIOD;

        rprintln!("Init");
        // unsafe {
        //     (*cortex_m::peripheral::SCB::ptr()).vtor.write(0x400000);
        // }
        // cortex_m::asm::dsb();
        let mut efc = hal::efc::Efc::new(device.EFC, hal::efc::VddioLevel::V3);
        // efc.set_wait_states(96).unwrap();
        // let mut watchdog = hal::watchdog::Watchdog::new(device.WDT);
        // watchdog.disable();
        //
        // Clock Configuration
        let clocks = Tokens::new((device.PMC, device.SUPC, device.UTMI), &device.WDT.into());
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();
        let slck = clocks.slck.configure_internal();
        let pllack = clocks.pllack.configure(&mainck, PllaConfig{ div: 2, mult: 25}).unwrap();
        // let _pck0: Pck<Pck0> = clocks.pcks.pck0.configure(&pllack, 2);
        // let _pck4: Pck<Pck4> = clocks.pcks.pck4.configure(&pllack, 2);
        let (mclk,mut hclk) = HostClockController::new(clocks.hclk, clocks.mck).
            configure(
                &pllack,
                &mut efc,
                HostClockConfig {
                    pres: HccPrescaler::Div1,
                    div: MckDivider::Div1,
                },
            )
            .unwrap();

        let mono = DwtSystick::new(&mut cx.core.DCB, cx.core.DWT, cx.core.SYST, 150_000_000);

        let banka = BankA::new(pioa, &mut hclk, &slck, BankConfiguration::default());
        let bankb = BankB::new(piob, &mut hclk, &slck, BankConfiguration::default());
        let bankd = BankD::new(piod, &mut hclk, &slck, BankConfiguration::default());


        // Configure PD[1,2,3,4,5,6,8,9,11,12,14,15,16] for ethernet
        let pd0:  Pin<_,Peripheral<A>> = bankd.pd0.into_peripheral();
        let pd1:  Pin<_,Peripheral<A>> = bankd.pd1.into_peripheral();
        let pd2:  Pin<_,Peripheral<A>> = bankd.pd2.into_peripheral();
        let pd3:  Pin<_,Peripheral<A>> = bankd.pd3.into_peripheral();
        let pd4:  Pin<_,Peripheral<A>> = bankd.pd4.into_peripheral();
        let pd5:  Pin<_,Peripheral<A>> = bankd.pd5.into_peripheral();
        let pd6:  Pin<_,Peripheral<A>> = bankd.pd6.into_peripheral();
        let pd7:  Pin<_,Peripheral<A>> = bankd.pd7.into_peripheral();
        let pd8:  Pin<_,Peripheral<A>> = bankd.pd8.into_peripheral();
        let pd9:  Pin<_,Peripheral<A>> = bankd.pd9.into_peripheral();
        // let _pd11: Pin<_,Peripheral<A>> = bankd.pd11.into_peripheral();
        // let _pd12: Pin<_,Peripheral<A>> = bankd.pd12.into_peripheral();
        // let _pd14: Pin<_,Peripheral<A>> = bankd.pd14.into_peripheral();
        // let _pd15: Pin<_,Peripheral<A>> = bankd.pd15.into_peripheral();
        // let _pd16: Pin<_,Peripheral<A>> = bankd.pd16.into_peripheral();

        let gmac = device.GMAC;

         let mut gmac = Gmac::new_gmac(
             gmac,
             (pd0, pd1, pd3, pd2, pd4, pd6, pd5, pd7, pd8, pd9),
             GmacConfiguration {speed: GmacSpeed::_100Mbit, mii: GmacMii::Rmii, duplex: GmacDuplex::HalfDuplex, mac: [0x04,0x91,0x62,0x01,0x02,0x03]},
             &mut hclk,
         )
         .unwrap();
        {
            // enables the peripheral clock
            // pmc.enable_periph_clk(39).unwrap();

            rprintln!("miim_post_setup might not return");
            gmac.miim_post_setup();
            rprintln!("miim_post_setup did return, all is good.");
        }

        // Configure TCP Stack
        let ip_addrs: &'static mut _ = if cfg!(feature = "static-ip") {
            singleton!(: [IpCidr; 1] = [IpCidr::new(IpAddress::v4(169, 254, 33, 1), 24)]).unwrap()
        } else {
            singleton!(: [IpCidr; 1] = [IpCidr::new(Ipv4Address::UNSPECIFIED.into(), 24)]).unwrap()
        };
        let neighbor_cache: &'static mut _ = singleton!(: [Option<(IpAddress, Neighbor)>; 8] = [None; 8]).unwrap();
        let sockets: &'static mut _ = singleton!(: [SocketStorage<'static>; 8] = [SocketStorage::EMPTY; 8]).unwrap();
        let routes_storage: &'static mut _ = singleton!(: [Option<(IpCidr, Route)>; 1] = [None; 1]).unwrap();
        let routes = Routes::new(routes_storage.as_mut_slice());

        let mut iface = InterfaceBuilder::new(gmac, sockets.as_mut_slice())
            .hardware_addr(EthernetAddress::from_bytes(&[0x04, 0x91, 0x62, 0x01, 0x02, 0x03]).into())
            .neighbor_cache(NeighborCache::new(neighbor_cache.as_mut_slice()))
            .routes(routes)
            .ip_addrs(ip_addrs.as_mut_slice())
            .finalize();

        let udp_socket = {
            let rx_data: &'static mut [u8] = singleton!(: [u8; 1024] = [0; 1024]).unwrap();
            let rx_metadata: &'static mut [UdpPacketMetadata] = singleton!(: [UdpPacketMetadata; 1024] = [UdpPacketMetadata::EMPTY; 1024]).unwrap();
            let tx_data: &'static mut [u8] = singleton!(: [u8; 1024] = [0; 1024]).unwrap();
            let tx_metadata: &'static mut [UdpPacketMetadata] = singleton!(: [UdpPacketMetadata; 1024] = [UdpPacketMetadata::EMPTY; 1024]).unwrap();
            let udp_rx_buffer = UdpSocketBuffer::new(rx_metadata, rx_data);
            let udp_tx_buffer = UdpSocketBuffer::new(tx_metadata, tx_data);
            UdpSocket::new(udp_rx_buffer,udp_tx_buffer)
        };

        let udp_handle = iface.add_socket(udp_socket);
        let dhcp_socket = smoltcp::socket::Dhcpv4Socket::new();
        let dhcp_handle = iface.add_socket(dhcp_socket);


        (
            Shared {
            },
            Local {
               iface,
               udp_handle,
               dhcp_handle,
            },
            init::Monotonics(
                // Initialization of optional monotonic timers go here
                mono
            ),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle(local=[iface, dhcp_handle, udp_handle])]
    fn idle(cx: idle::Context) -> ! {
        rprintln!("idle");
        let mut iface = cx.local.iface;
        let udp_handle = cx.local.udp_handle;
        let dhcp_handle = cx.local.dhcp_handle;
        loop {
                
            match iface.poll(Instant::from_micros(monotonics::now().ticks()/48)) {
                Ok(_) => {},
                Err(e) => {
                    rprintln!("Error: {:?}", e);
                }
            }


            let event = if cfg!(not(feature = "static-ip")) {
                iface.get_socket::<Dhcpv4Socket>(*dhcp_handle).poll()
            } else {
                None
            };
            match event {
                None => {}
                Some(Dhcpv4Event::Configured(config)) => {
                    rprintln!("DHCP config acquired!");

                    rprintln!("IP address:      {}", config.address);
                    // set_ipv4_addr(&mut iface, config.address);
                    iface.update_ip_addrs(|addrs| {
                        let dest = addrs.iter_mut().next().unwrap();
                        *dest = IpCidr::Ipv4(config.address);
                    });

                    if let Some(router) = config.router {
                        rprintln!("Default gateway: {}", router);
                        iface.routes_mut().add_default_ipv4_route(router).unwrap();
                    } else {
                        rprintln!("Default gateway: None");
                        iface.routes_mut().remove_default_ipv4_route();
                    }

                    for (i, s) in config.dns_servers.iter().enumerate() {
                        if let Some(s) = s {
                            rprintln!("DNS server {}:    {}", i, s);
                        }
                    }
                }
                Some(Dhcpv4Event::Deconfigured) => {
                    rprintln!("DHCP lost config!");
                    // set_ipv4_addr(&mut iface, Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0));
                    iface.update_ip_addrs(|addrs| {
                        let dest = addrs.iter_mut().next().unwrap();
                        *dest = IpCidr::Ipv4(Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0));
                    });
                    iface.routes_mut().remove_default_ipv4_route();
                }
            }

            let socket = iface.get_socket::<UdpSocket>(*udp_handle);


            if !socket.is_open() {
                socket.bind(1234).unwrap();
                rprintln!("Socket is listening on port 1234");
            }


            let client = match socket.recv() {
                Ok((data, endpoint)) => {
                    rprintln!("udp: 1234 recv data: {:?} from {}",
                                 str::from_utf8(data).unwrap(),
                                 endpoint
                    );
                    Some(endpoint)
                }
                Err(_) => None,
            };

            if let Some(endpoint) = client {
                socket.send_slice("hello\n".as_bytes(), endpoint);
            }

                


        }
    }

}
