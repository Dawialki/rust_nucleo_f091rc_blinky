#![no_main]
#![no_std]

extern crate cortex_m_rt;
use panic_halt as _;


#[rtic::app(device = stm32f0xx_hal::pac)]
mod app {
    use cortex_m::peripheral::syst::SystClkSource;
    use stm32f0xx_hal::gpio::{Output, PushPull, gpioa::PA5};
    use stm32f0xx_hal::prelude::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut device = cx.device;
        let mut systick = cx.core.SYST;

        // configures the system timer to trigger a SysTick exception every second
        // this is configured for the STM32F091 which has a default CPU clock of 8 MHz
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(8_000_000);
        systick.clear_current();
        systick.enable_counter();
        systick.enable_interrupt();

        // Enable Clock to GPIOA
        let mut rcc = device.RCC.configure().sysclk(8.mhz()).freeze(&mut device.FLASH);
        let gpioa = device.GPIOA.split(&mut rcc);

        let led = cortex_m::interrupt::free(move |cs| {
            // (Re-)configure PA1 as output
            gpioa.pa5.into_push_pull_output(cs)
        });

        (Shared {},Local {led}, init::Monotonics())
    }

    #[task(binds = SysTick, local = [toggle: bool = false, led])]
    fn systick(cx: systick::Context) {

        // Toggle LED
        cx.local.led.toggle().unwrap();
    }
}