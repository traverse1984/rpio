#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use embedded_time::rate::*;
use rp_pico::{
    hal::{
        clocks,
        pac::{CorePeripherals, Peripherals},
        prelude::*,
        pwm::Slices,
        Adc, Sio, Watchdog,
    },
    Pins,
};

use panic_halt as _;

use embedded_hal::adc::OneShot;
use embedded_hal::PwmPin;

use embedded_hal::digital::v2::OutputPin;

mod keypad;
use keypad::*;
use rpio::{pico_oled::*, pinout, screen, spi::Spi, use_screen};

mod io;
mod programs;
use io::Io;
use programs::Program;

#[entry]
fn main() -> ! {
    let mut pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    rpio::dev::delay::Delay::init(core.SYST, clocks.system_clock.freq().integer());

    let mut delay = rpio::dev::delay::Delay;

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut keypad = {
        pinout!(
            output readable {
                col1 = pins.gpio0,
                col2 = pins.gpio1,
                col3 = pins.gpio2,
                col4 = pins.gpio3,
            }
            input pulldown {
                row1 = pins.gpio4,
                row2 = pins.gpio5,
                row3 = pins.gpio13,
                row4 = pins.gpio14,
            }
        );

        GpioKeypad::new(col1, col2, col3, col4, row1, row2, row3, row4)
    };

    let oled = {
        pinout!(
            output {
                dcmd = pins.gpio8,
                oled_reset = pins.gpio12,
            }

            spi {
                pins.gpio10,
                pins.gpio11,
            }
        );

        let spi = Spi::new_rp2040(
            pac.SPI1,
            pins.gpio9,
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            30_000_000u32.Hz(),
            &embedded_hal::spi::MODE_0,
        )
        .init();

        oled_reset.set_high().unwrap();
        oled_reset.set_low().unwrap();
        oled_reset.set_high().unwrap();

        let mut oled = PicoOled::new(spi, dcmd);
        oled.init();
        oled
    };

    let flash = {
        use rpio::flash::{Buffer, Device, FlashBuffer, Size};

        pinout!(
            spi {
                pins.gpio16,
                pins.gpio18,
                pins.gpio19
            }
        );

        let spi = Spi::new_rp2040(
            pac.SPI0,
            pins.gpio20,
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            20_000_000u32.Hz(),
            &embedded_hal::spi::MODE_0,
        )
        .init();

        let buf = Buffer::<5001>::new();
        Device::new(spi, Size::from_mb(1).unwrap(), buf)
    };

    pinout!(
        output {
            buzz = pins.gpio28,
        }
        output high {
            led = pins.led,
        }
        input pullup {
            btna = pins.gpio15,
            btnb = pins.gpio17,
        }
        disabled pulldown {
            pins.gpio7,
        }
    );

    // if let Some(_) = keypad.read() {
    //     keypad.read_keyup();
    // } else {
    //     programs::draw(Io::new(delay, oled, keypad));
    // }

    let mut screen = screen!(oled);

    let programs = [
        Program::FlashBrowser,
        Program::MultiKey,
        Program::Adc,
        Program::Pwm,
        Program::KeySeq,
        Program::Interrupt,
        Program::ReadIrq,
        Program::Draw,
    ];

    for (i, program) in programs.iter().enumerate().take(8) {
        screen
            .write_fmt(format_args!("{} {}\n", i + 1, program.name()))
            .ok()
            .unwrap();
    }

    screen.update();

    let selection = loop {
        match keypad.read_keyup() {
            Some(key @ 1..=8) => match programs.get(key as usize - 1) {
                Some(program) => {
                    break *program;
                }
                _ => (),
            },
            _ => (),
        }
    };

    screen.clear().update();
    let io = Io::new(delay, screen.destroy().0, keypad);

    match selection {
        Program::FlashBrowser => programs::flash(io, flash),
        Program::MultiKey => programs::multikey(io),
        Program::Adc => {
            let adc = Adc::new(pac.ADC, &mut pac.RESETS);
            let adc_pin = pins.gpio26.into_floating_input();
            programs::adc(io, adc, adc_pin);
        }
        Program::Pwm => {
            let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
            programs::pwm(io, pwm_slices, pins.gpio22);
        }
        Program::KeySeq => programs::keyseq(io),
        Program::Interrupt => loop {}, //programs::interrupt(io, led, btna),
        Program::ReadIrq => programs::read_irq(io, btnb, btna),
        Program::Draw => programs::draw(io),
    }
}
