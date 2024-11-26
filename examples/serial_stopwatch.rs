#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac::{interrupt, Interrupt, Peripherals, TIM16},
    prelude::*,
    timers::{Event, Timer},
};
use core::cell::RefCell;
use core::fmt::Write as _;
use core::ops::DerefMut;
use cortex_m::{interrupt::Mutex, peripheral::Peripherals as c_m_Peripherals};
use cortex_m_rt::entry;
use embedded_hal_02::timer::CountDown;

// Make timer interrupt registers globally available
static GINT: Mutex<RefCell<Option<Timer<TIM16>>>> = Mutex::new(RefCell::new(None));

#[derive(Copy, Clone)]
struct Time {
    seconds: u32,
    millis: u16,
}

static TIME: Mutex<RefCell<Time>> = Mutex::new(RefCell::new(Time {
    seconds: 0,
    millis: 0,
}));

// Define an interupt handler, i.e. function to call when interrupt occurs. Here if our external
// interrupt trips when the timer timed out
#[interrupt]
fn TIM16() {
    cortex_m::interrupt::free(|cs| {
        // Move LED pin here, leaving a None in its place
        GINT.borrow(cs)
            .borrow_mut()
            .deref_mut()
            .as_mut()
            .unwrap()
            .wait()
            .ok();
        let mut time = TIME.borrow(cs).borrow_mut();
        time.millis += 1;
        if time.millis == 1000 {
            time.millis = 0;
            time.seconds += 1;
        }
    });
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        let mut serial = cortex_m::interrupt::free(move |cs| {
            let mut flash = p.FLASH;
            let rcc = p.RCC.configure().sysclk(24.mhz()).freeze(&mut flash);

            // Use USART1 with PA2 and PA3 as serial port
            let gpioa = p.GPIOA.split();
            let tx = gpioa.pa2.into_alternate_af1();
            let rx = gpioa.pa3.into_alternate_af1();

            // Set up a timer expiring every millisecond
            let mut timer = Timer::tim16(p.TIM16, 1000.hz(), &rcc.clocks);

            // Generate an interrupt when the timer expires
            timer.listen(Event::TimeOut);

            // Move the timer into our global storage
            *GINT.borrow(cs).borrow_mut() = Some(timer);

            // Enable TIM1 IRQ, set prio 1 and clear any pending IRQs
            let mut nvic = cp.NVIC;
            unsafe {
                nvic.set_priority(Interrupt::TIM16, 1);
                cortex_m::peripheral::NVIC::unmask(Interrupt::TIM16);
            }
            cortex_m::peripheral::NVIC::unpend(Interrupt::TIM16);

            // Set up our serial port
            p.USART1.serial((tx, rx), 115_200.bps(), &rcc.clocks)
        });

        // Print a welcome message
        writeln!(
            serial,
            "Welcome to the stop watch, hit any key to see the current value and 0 to reset\r",
        )
        .ok();

        loop {
            // Wait for reception of a single byte
            let received = nb::block!(serial.rx.read()).unwrap();

            let time = cortex_m::interrupt::free(|cs| {
                let mut time = TIME.borrow(cs).borrow_mut();

                // If we received a 0, reset the time
                if received == b'0' {
                    time.millis = 0;
                    time.seconds = 0;
                }

                *time
            });

            // Print the current time
            writeln!(serial, "{}.{:03}s\r", time.seconds, time.millis).ok();
        }
    }

    loop {
        continue;
    }
}
