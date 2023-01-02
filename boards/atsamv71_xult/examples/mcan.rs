//! MCAN example
//! Assumed bus:
//! - 375 kb/s nominal bitrate
//! - 750 kb/s data bitrate if sending/receiving CAN FD frames with bitrate
//!   switching
//!
//! 1. Sends a message over CAN on SW0 button press and prints out transmit
//! event queue content, protocol status register and error counter register in
//! RTT terminal.
//!
//! 2. Sets up two interrupt lines and message filters
//! - messages with standard IDs will end up in RxFifo0
//! - messages with extended IDs will end up in RxFifo1
//! - messages content will be printed out in RTT terminal upon arrival
//!
//! 3. LED0 will blink to indicate activity (sending & receiving)
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use atsamx7x_hal as hal;
use hal::clocks::*;
use hal::efc::*;
use hal::pio::*;
use mcan::prelude::*;
use typenum::consts::*;

use hal::ehal::digital::v2::OutputPin as _;
use hal::fugit::RateExtU32 as _;

use dwt_systick_monotonic::{DwtSystick, ExtU32};
use mcan::embedded_can as ecan;
use mcan::interrupt::{Interrupt, InterruptLine, OwnedInterruptSet};
use mcan::message::rx;
use mcan::message::tx;
use mcan::messageram::SharedMemory;
use mcan::rx_fifo::Fifo0;
use mcan::rx_fifo::Fifo1;
use mcan::rx_fifo::RxFifo;
use mcan::{
    config::{BitTiming, Mode},
    filter::{Action, ExtFilter, Filter},
};

pub struct Capacities;

impl mcan::messageram::Capacities for Capacities {
    type StandardFilters = U1;
    type ExtendedFilters = U1;
    type RxBufferMessage = rx::Message<64>;
    type DedicatedRxBuffers = U0;
    type RxFifo0Message = rx::Message<64>;
    type RxFifo0 = U64;
    type RxFifo1Message = rx::Message<64>;
    type RxFifo1 = U64;
    type TxMessage = tx::Message<64>;
    type TxBuffers = U32;
    type DedicatedTxBuffers = U0;
    type TxEventFifo = U32;
}

type RxFifo0 = RxFifo<
    'static,
    Fifo0,
    hal::can::Can1,
    <Capacities as mcan::messageram::Capacities>::RxFifo0Message,
>;
type RxFifo1 = RxFifo<
    'static,
    Fifo1,
    hal::can::Can1,
    <Capacities as mcan::messageram::Capacities>::RxFifo1Message,
>;
type Tx = mcan::tx_buffers::Tx<'static, hal::can::Can1, Capacities>;
type TxEventFifo = mcan::tx_event_fifo::TxEventFifo<'static, hal::can::Can1>;
type Aux = mcan::bus::Aux<
    'static,
    hal::can::Can1,
    hal::can::Dependencies<hal::can::Can1, Pin<PC14, Peripheral<C>>, Pin<PC12, Peripheral<C>>>,
>;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [PIOB])]
mod app {
    use super::*;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<12_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
        irq: BankInterrupts<A>,
        line_0_interrupts: OwnedInterruptSet<hal::can::Can1>,
        line_1_interrupts: OwnedInterruptSet<hal::can::Can1>,
        rx_fifo_0: RxFifo0,
        rx_fifo_1: RxFifo1,
        tx: Tx,
        tx_event_fifo: TxEventFifo,
        aux: Aux,
    }

    #[init(local = [
        #[link_section = ".can"]
        can_memory: SharedMemory<Capacities> = SharedMemory::new()
    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Application up!");

        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let slck = clocks.slck.configure_external_normal();
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();

        let (hclk, mut mck) = HostClockController::new(clocks.hclk, clocks.mck)
            .configure(
                &mainck,
                &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
                HostClockConfig {
                    pres: HccPrescaler::Div1,
                    div: MckDivider::Div1,
                },
            )
            .unwrap();

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            hclk.systick_freq().to_Hz(),
        );

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut mck,
            &slck,
            BankConfiguration {
                min_debounce_duration: hal::fugit::MillisDurationU32::from_ticks(50).convert(),
            },
        );

        let bankc = hal::pio::BankC::new(
            ctx.device.PIOC,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );

        let mut button = banka.pa9.into_input(PullDir::PullUp);
        button.set_interrupt(Some(InterruptType::FallingEdge));
        button.set_filter(Some(InputFilter::Debounce));
        let led = banka.pa23.into_output(true);

        let pck5 = clocks.pcks.pck5.configure(&mck, 1).unwrap();

        // Safety: CAN{0,1}DMABASEADDRESS not touched
        let dependencies = unsafe {
            hal::can::Dependencies::<hal::can::Can1, _, _>::new(
                ctx.device.MCAN1,
                &ctx.device.MATRIX,
                (bankc.pc14.into_mode(), bankc.pc12.into_mode()),
                &pck5,
                &mut mck,
            )
        };

        let mut can =
            mcan::bus::CanConfigurable::new(375.kHz(), dependencies, ctx.local.can_memory).unwrap();

        can.config().mode = Mode::Fd {
            allow_bit_rate_switching: true,
            data_phase_timing: BitTiming::new(750.kHz()),
        };

        let line_0_interrupts = can
            .interrupts()
            .enable(
                [
                    Interrupt::RxFifo0NewMessage,
                    Interrupt::RxFifo0Full,
                    Interrupt::RxFifo0MessageLost,
                ]
                .into_iter()
                .collect(),
                InterruptLine::Line0,
            )
            .unwrap();

        let line_1_interrupts = can
            .interrupts()
            .enable(
                [
                    Interrupt::RxFifo1NewMessage,
                    Interrupt::RxFifo1Full,
                    Interrupt::RxFifo1MessageLost,
                ]
                .into_iter()
                .collect(),
                InterruptLine::Line1,
            )
            .unwrap();

        can.filters_standard()
            .push(Filter::Classic {
                action: Action::StoreFifo0,
                filter: ecan::StandardId::MAX,
                mask: ecan::StandardId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Standard filter application failed"));

        can.filters_extended()
            .push(ExtFilter::Classic {
                action: Action::StoreFifo1,
                filter: ecan::ExtendedId::MAX,
                mask: ecan::ExtendedId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Extended filter application failed"));

        let can = can.finalize().unwrap();

        let rx_fifo_0 = can.rx_fifo_0;
        let rx_fifo_1 = can.rx_fifo_1;
        let tx = can.tx;
        let tx_event_fifo = can.tx_event_fifo;
        let aux = can.aux;

        (
            Shared {},
            Local {
                led,
                irq: banka.interrupts,
                line_0_interrupts,
                line_1_interrupts,
                rx_fifo_0,
                rx_fifo_1,
                tx,
                tx_event_fifo,
                aux,
            },
            init::Monotonics(mono),
        )
    }

    #[task(binds = PIOA, local = [counter: u16 = 0, irq, tx_event_fifo, aux, tx])]
    fn pioa(ctx: pioa::Context) {
        for pin in ctx.local.irq.iter() {
            if pin == 9 {
                bump_activity_led();
                rprintln!("Button pressed! Status:");
                while let Some(e) = ctx.local.tx_event_fifo.pop() {
                    rprintln!("TxEvent: {:0X?}", e);
                }
                rprintln!("{:?}", ctx.local.aux.protocol_status());
                rprintln!("{:?}", ctx.local.aux.error_counters());

                let counter = *ctx.local.counter;
                let wrapped_counter = (counter % u8::MAX as u16) as u8;
                let mut payload = [0_u8; 64];
                payload.fill(wrapped_counter);

                ctx.local
                    .tx
                    .transmit_queued(
                        tx::MessageBuilder {
                            id: ecan::Id::Extended(ecan::ExtendedId::new(counter as _).unwrap()),
                            frame_type: tx::FrameType::FlexibleDatarate {
                                payload: &payload,
                                bit_rate_switching: true,
                                force_error_state_indicator: false,
                            },
                            store_tx_event: Some(wrapped_counter),
                        }
                        .build()
                        .unwrap(),
                    )
                    .unwrap();
                rprintln!("Message {:0X} sent!", counter);
                *ctx.local.counter += 1;
            }
        }
    }

    #[task(priority = 2, binds = MCAN1_INT0, local = [line_0_interrupts, rx_fifo_0])]
    fn mcan1_int0(mut ctx: mcan1_int0::Context) {
        bump_activity_led();
        let line_0_interrupts = ctx.local.line_0_interrupts;
        for interrupt in line_0_interrupts.iter_flagged() {
            match interrupt {
                Interrupt::RxFifo0NewMessage => {
                    for message in &mut ctx.local.rx_fifo_0 {
                        log("RxFifo0", &message);
                    }
                }
                i => rprintln!("{:?} interrupt triggered", i),
            }
        }
    }

    #[task(priority = 2, binds = MCAN1_INT1, local = [line_1_interrupts, rx_fifo_1])]
    fn mcan1_int1(mut ctx: mcan1_int1::Context) {
        bump_activity_led();
        let line_1_interrupts = ctx.local.line_1_interrupts;
        for interrupt in line_1_interrupts.iter_flagged() {
            match interrupt {
                Interrupt::RxFifo1NewMessage => {
                    for message in &mut ctx.local.rx_fifo_1 {
                        log("RxFifo1", &message);
                    }
                }
                i => rprintln!("{:?} interrupt triggered", i),
            }
        }
    }

    #[task(local = [led])]
    fn activity_led(ctx: activity_led::Context, led_on: bool) {
        let _ = ctx.local.led.set_state((!led_on).into());
        if led_on {
            let _ = activity_led::spawn_after(100.millis(), false);
        }
    }

    fn bump_activity_led() {
        let _ = activity_led::spawn(true);
    }

    fn log(fifo: &str, message: &impl mcan::message::Raw) {
        rprintln!("New message received ({})", fifo);
        rprintln!("id:              {:0X?}", message.id());
        rprintln!("decoded_dlc:     {:?}", message.decoded_dlc());
        rprintln!("fd_format:       {:?}", message.fd_format());
        rprintln!("is_remote_frame: {:?}", message.is_remote_frame());
        rprintln!("data:            {:0X?}", message.data());
    }
}
