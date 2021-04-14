#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate panic_semihosting;
extern crate ra8875;
extern crate stm32f4xx_hal;

use core::fmt::Write;

use cortex_m_rt::entry;
use hal::prelude::*;
use hal::spi;
use hal::spi::FullDuplex;

use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::spi::{Mode, Spi};
use stm32f4xx_hal::stm32;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Text, TextStyleBuilder},
};

use ra8875::RA8875;

const HSE_VALUE: u32 = 8_000_000; // On the discovery board, HSE == 8Mhz

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let mut led1 = gpiod.pd12.into_push_pull_output();
    let mut led2 = gpiod.pd14.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(HSE_VALUE.hz())
        .hclk(HSE_VALUE.hz())
        .pclk1((HSE_VALUE / 4).hz())
        .pclk2((HSE_VALUE / 2).hz())
        .freeze();

    let mut delay = stm32f4xx_hal::delay::Delay::new(cp.SYST, clocks);

    let pb13 = gpiob.pb13.into_alternate_af5();
    let pb14 = gpiob.pb14.into_alternate_af5();
    let pb15 = gpiob.pb15.into_alternate_af5();
    let spi_2_mode = spi::Mode {
        phase: spi::Phase::CaptureOnSecondTransition,
        polarity: spi::Polarity::IdleHigh,
    };
    let spi2 = Spi::spi2(
        dp.SPI2,
        (pb13, pb14, pb15),
        spi_2_mode,
        2_000_000.hz(),
        clocks,
    );
    let ready = gpioc.pc4.into_pull_up_input();
    let mut cs = gpioc.pc2.into_push_pull_output();
    cs.set_high().ok().unwrap();
    let rst = gpioc.pc3.into_push_pull_output();
    let mut display = RA8875::new(spi2, (800, 480), ready, cs, rst);

    display.rst.set_low().ok().unwrap();
    delay.delay_ms(100_u16);
    display.rst.set_high().ok().unwrap();
    delay.delay_ms(1400_u16);

    display.self_check().unwrap();

    // Set up RA8875 display
    display.set_up_pll().unwrap();
    delay.delay_ms(500_u16);
    display.init().unwrap();
    delay.delay_ms(500_u16);
    display.gpiox(true).unwrap();
    display.display_on(true).unwrap();
    display.pwm1_config(true, 0x0A).unwrap();
    display.pwm1_out(0xFF).unwrap();
    display.fill_screen(0x0000).unwrap();
    display.enable_touch().unwrap();

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::RED)
        .build();

    Text::new("Testing!!!!", Point::new(200, 200))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();
        delay.delay_ms(1000_u32);
        led1.set_low().unwrap();
        led2.set_high().unwrap();
        delay.delay_ms(1000_u32);
    }
}
