//! GMAC Example
//! This example should echo messages received via TCP.
//! NOTE: This code has not been tested.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};
use panic_halt as _;
// use defmt_rtt as _;
// use defmt;
// 
// static COUNT: AtomicUsize = AtomicUsize::new(0);
// defmt::timestamp!("{=usize}", {
//     // NOTE(no-CAS) `timestamps` runs with interrupts disabled
//     let n = COUNT.load(Ordering::Relaxed);
//     COUNT.store(n + 1, Ordering::Relaxed);
//     n
// });
#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::gmac::*;
    use smoltcp::iface::{Neighbor, InterfaceBuilder, SocketStorage, NeighborCache, Interface, Route, Routes};
    use smoltcp::phy::{Device, RxToken, TxToken};
    use smoltcp::socket::{TcpSocketBuffer, TcpSocket, Dhcpv4Event, Dhcpv4Socket};
    use smoltcp::time::Instant;
    use smoltcp::wire::{IpCidr, IpAddress, EthernetAddress, HardwareAddress, Ipv4Address, Ipv4Cidr};
    use cortex_m::singleton;

    use rtt_target::rtt_init_print;
    use rtt_target::rprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        iface: Interface<'static, Gmac>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {

        rtt_init_print!();
        rprintln!("Init");

        let mut efc = hal::efc::Efc::new(ctx.device.EFC, hal::efc::VddioLevel::V3);
        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::InternalRC);
        pmc.enable_peripherals(&[PeripheralIdentifier::GMAC]);

        let bankd = hal::pio::BankD::new(ctx.device.PIOD, &mut pmc, BankConfiguration::default());
        // Configure PD[1,2,3,4,5,6,8,9,11,12,14,15,16] for ethernet
        let _pd1:  Pin<_,Peripheral<A>> = bankd.pd1.into_peripheral();
        let _pd2:  Pin<_,Peripheral<A>> = bankd.pd2.into_peripheral();
        let _pd3:  Pin<_,Peripheral<A>> = bankd.pd3.into_peripheral();
        let _pd4:  Pin<_,Peripheral<A>> = bankd.pd4.into_peripheral();
        let _pd5:  Pin<_,Peripheral<A>> = bankd.pd5.into_peripheral();
        let _pd6:  Pin<_,Peripheral<A>> = bankd.pd6.into_peripheral();
        let _pd8:  Pin<_,Peripheral<A>> = bankd.pd8.into_peripheral();
        let _pd9:  Pin<_,Peripheral<A>> = bankd.pd9.into_peripheral();
        let _pd11: Pin<_,Peripheral<A>> = bankd.pd11.into_peripheral();
        let _pd12: Pin<_,Peripheral<A>> = bankd.pd12.into_peripheral();
        let _pd14: Pin<_,Peripheral<A>> = bankd.pd14.into_peripheral();
        let _pd15: Pin<_,Peripheral<A>> = bankd.pd15.into_peripheral();
        let _pd16: Pin<_,Peripheral<A>> = bankd.pd16.into_peripheral();

        let gmac = ctx.device.GMAC;
        let mut gmac = unsafe { Gmac::new(gmac) };
        {
            // enables the peripheral clock
            // pmc.enable_periph_clk(39).unwrap();

            gmac.init();
            // defmt::debug!("miim_post_setup might not return");
            gmac.miim_post_setup();
            // defmt::debug!("miim_post_setup did return, all is good.");
        }

        
        let ip_addrs: &'static mut _ = singleton!(: [IpCidr; 1] = [IpCidr::new(IpAddress::v4(169, 254, 33, 1), 24)]).unwrap();
        let neighbor_cache: &'static mut _ = singleton!(: [Option<(IpAddress, Neighbor)>; 8] = [None; 8]).unwrap();
        let sockets: &'static mut _ = singleton!(: [SocketStorage<'static>; 8] = [SocketStorage::EMPTY; 8]).unwrap();
        let routes_storage: &'static mut _ = singleton!(: [Option<(IpCidr, Route)>; 1] = [None; 1]).unwrap();
        let routes = Routes::new(routes_storage.as_mut_slice());

        let iface = InterfaceBuilder::new(gmac, sockets.as_mut_slice())
            .hardware_addr(EthernetAddress::from_bytes(&[0x04, 0x91, 0x62, 0x01, 0x02, 0x03]).into())
            .neighbor_cache(NeighborCache::new(neighbor_cache.as_mut_slice()))
            .routes(routes)
            .ip_addrs(ip_addrs.as_mut_slice())
            .finalize();

        (Shared {}, Local {iface}, init::Monotonics())
    }

    #[idle(local = [iface])]
    fn idle(ctx: idle::Context) -> ! {
        let mut iface = ctx.local.iface;
        rprintln!("Idle");

        let server_socket = {
            let rx_data: &'static mut [u8] = singleton!(: [u8; 1024] = [0; 1024]).unwrap();
            let tx_data: &'static mut [u8] = singleton!(: [u8; 1024] = [0; 1024]).unwrap();
            let tcp_rx_buffer = TcpSocketBuffer::new(rx_data);
            let tcp_tx_buffer = TcpSocketBuffer::new(tx_data);
            TcpSocket::new(tcp_rx_buffer,tcp_tx_buffer)
        };

        let server_handle = iface.add_socket(server_socket);
        let mut last_state = smoltcp::socket::TcpState::Closed;
        loop {
            match iface.poll(Instant::from_micros(48)) {
                Ok(_) => {},
                Err(e) => {
                }
            }

            
            let mut buf = [0u8;1024];
            let socket = iface.get_socket::<TcpSocket>(server_handle);
            let state = socket.state();
            if state != last_state {
                // defmt::println!("STATE CHANGE: {=?} => {=?}", last_state, state);
                last_state = state;
            }

            if state == smoltcp::socket::TcpState::CloseWait {
                socket.close();
            }

            if !socket.is_active() && !socket.is_listening() {
                // defmt::info!("Listening...");
                socket.listen(4321).unwrap();
            }


            let mut to_send = None;
            if socket.can_recv() {
                socket.recv(|buffer| {
                    // defmt::info!("Receive!");
                    // defmt::info!("Len: {}", buffer.len());
                    // defmt::info!("dat {}", buffer);
                    // defmt::info!("{}", cmd_string.as_str());

                    buf[..buffer.len()].copy_from_slice(&buffer[..buffer.len()]);
                    to_send = Some(&buf[..buffer.len()]);

                    (buffer.len(), ())
                }).unwrap();
            }
            if socket.can_send() && to_send != None {
                let tx = to_send.unwrap();
                socket.send_slice(tx).unwrap();
            }
        }
    }
}
