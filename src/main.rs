#![no_main]
#![no_std]
#![feature(array_methods)]
#![feature(iterator_try_collect)]
#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "alloc")]
pub const HEAP_SIZE: usize = 1024;

#[cfg(feature = "alloc")]
#[global_allocator]
pub static ALLOCATOR: alloc_cortex_m::CortexMHeap = alloc_cortex_m::CortexMHeap::empty();

#[cfg(feature = "alloc")]
pub static mut HEAP: [core::mem::MaybeUninit<u8>; HEAP_SIZE] = [core::mem::MaybeUninit::uninit(); HEAP_SIZE];

#[cfg(feature = "alloc")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}

#[cfg(feature = "serial")]
use core::fmt::{Debug, Write};

mod games;
pub mod util;

use hal::{gpio::PinState, prelude::*};
use oorandom::Rand32;
use stm32f4xx_hal as hal;

const DISPLAY_DIMS: (usize, usize) = (8, 8);
const NUM_PIXELS: usize = DISPLAY_DIMS.0 * DISPLAY_DIMS.1;

use stm32f4::stm32f411::interrupt;
#[interrupt]
fn TIM2() {
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // stm32f4::stm32f411::Interrupt::TIM2
    if cfg!(feature = "alloc") {
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    let dp = hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();
    let _delay = cp.SYST.delay(&clocks);

    // if cfg!(feature = "serial") {
    //     use hal::serial::Serial;
    //     let mut ser1: Serial<_, _, u8> = dp.USART1.serial(
    //         (gpioa.pa9.into_alternate(), gpioa.pa10.into_alternate()),
    //         115200.bps(),
    //         &clocks
    //     ).unwrap();
    // }

    // led matrix pins
    // it would be a lot cleaner and probably a lot faster to use the entire register rather than
    // splitting out the pins for each led, but I just want to get a poc done.
    let mut led_hiside_0 = gpioa.pa0.into_push_pull_output();
    let mut led_hiside_1 = gpioa.pa1.into_push_pull_output();
    let mut led_hiside_2 = gpioa.pa2.into_push_pull_output();
    let mut led_hiside_3 = gpioa.pa3.into_push_pull_output();
    let mut led_hiside_4 = gpioa.pa4.into_push_pull_output();
    let mut led_hiside_5 = gpioa.pa5.into_push_pull_output();
    let mut led_hiside_6 = gpioa.pa6.into_push_pull_output();
    let mut led_hiside_7 = gpioa.pa7.into_push_pull_output();

    let mut led_lowside_0 = gpiob.pb0.into_push_pull_output();
    let mut led_lowside_1 = gpiob.pb1.into_push_pull_output();
    let mut led_lowside_2 = gpiob.pb2.into_push_pull_output();
    let mut led_lowside_3 = gpiob.pb3.into_push_pull_output();
    let mut led_lowside_4 = gpiob.pb4.into_push_pull_output();
    let mut led_lowside_5 = gpiob.pb5.into_push_pull_output();
    let mut led_lowside_6 = gpiob.pb6.into_push_pull_output();
    let mut led_lowside_7 = gpiob.pb7.into_push_pull_output();

    // button inputs
    let right = gpioa.pa8.into_floating_input();
    let down = gpioa.pa9.into_floating_input();
    let left = gpioa.pa10.into_floating_input();
    let center = gpioa.pa11.into_floating_input();
    let up = gpioa.pa12.into_floating_input();

    // function to multiplex the leds
    let mut set_pixel = |idx: usize, state: bool| {
        let (x, y) = util::idx_to_pos(idx);

        led_lowside_0.set_state(PinState::from(true));
        led_lowside_1.set_state(PinState::from(true));
        led_lowside_2.set_state(PinState::from(true));
        led_lowside_3.set_state(PinState::from(true));
        led_lowside_4.set_state(PinState::from(true));
        led_lowside_5.set_state(PinState::from(true));
        led_lowside_6.set_state(PinState::from(true));
        led_lowside_7.set_state(PinState::from(true));

        led_hiside_0.set_state(PinState::from(false));
        led_hiside_1.set_state(PinState::from(false));
        led_hiside_2.set_state(PinState::from(false));
        led_hiside_3.set_state(PinState::from(false));
        led_hiside_4.set_state(PinState::from(false));
        led_hiside_5.set_state(PinState::from(false));
        led_hiside_6.set_state(PinState::from(false));
        led_hiside_7.set_state(PinState::from(false));

        led_lowside_0.set_state(PinState::from(!((y == 0) && state)));
        led_lowside_1.set_state(PinState::from(!((y == 1) && state)));
        led_lowside_2.set_state(PinState::from(!((y == 2) && state)));
        led_lowside_3.set_state(PinState::from(!((y == 3) && state)));
        led_lowside_4.set_state(PinState::from(!((y == 4) && state)));
        led_lowside_5.set_state(PinState::from(!((y == 5) && state)));
        led_lowside_6.set_state(PinState::from(!((y == 6) && state)));
        led_lowside_7.set_state(PinState::from(!((y == 7) && state)));

        led_hiside_0.set_state(PinState::from((x == 0) && state));
        led_hiside_1.set_state(PinState::from((x == 1) && state));
        led_hiside_2.set_state(PinState::from((x == 2) && state));
        led_hiside_3.set_state(PinState::from((x == 3) && state));
        led_hiside_4.set_state(PinState::from((x == 4) && state));
        led_hiside_5.set_state(PinState::from((x == 5) && state));
        led_hiside_6.set_state(PinState::from((x == 6) && state));
        led_hiside_7.set_state(PinState::from((x == 7) && state));
    };

    // do this to get a random seed
    let mut rng_seed = 0;
    while !center.is_low() {
        rng_seed += 1;
    }

    use games::{Context, GameState, InputState};
    let mut frame = 0;
    let mut rng = Rand32::new(rng_seed);

    let mut game = GameState::new(&mut Context {
        rng: &mut rng,
        global_frame: &frame,
    });

    loop {
        game.update(
            &InputState {
                up: up.is_low(),
                down: down.is_low(),
                left: left.is_low(),
                right: right.is_low(),
                center: center.is_low(),
            },
            &mut Context {
                rng: &mut rng,
                global_frame: &frame,
            },
        );

        // multiplex the leds
        // this really is not optimal, I should be using timers/interrupts here...
        let pixels = game.display();
        for j in 0..1000 {
            let blinking = (j / 250) % 2 == 0;

            for i in 0..pixels.len() {
                let val = match pixels[i] {
                    0 => false,
                    1 => blinking,
                    _ => true,
                };
                set_pixel(i, val);
            }
        }

        frame += 1;
    }
}
