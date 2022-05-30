use crate::io::*;

use core::str;
use embedded_hal::PwmPin;
use rp2040_hal::{
    gpio::{bank0::BankPinId, Pin, PinId, PinMode, ValidPinMode},
    pwm::{Pwm3, Slices, ValidPwmOutputPin, A},
};

pub fn pwm<S, D, K, PwmPinId, PwmPinMode>(
    io: Io<S, D, K>,
    mut pwm_slices: Slices,
    pin: Pin<PwmPinId, PwmPinMode>,
) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
    PwmPinId: PinId + BankPinId + ValidPwmOutputPin<Pwm3, A>,
    PwmPinMode: PinMode + ValidPinMode<PwmPinId>,
{
    setup!(io => delay, screen: ScaledBuf::new(), keypad);

    let pwm = &mut pwm_slices.pwm3;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_a;
    channel.output_to(pin);

    loop {
        if keypad.key_is_pressed() {
            clear!();
            cur!(8, 8);
            draw!(txt "Duty");
            cur!(8, 16);
            draw!(txt "00000");
            update!();

            keypad.read_keyup();

            let mut duty = [48; 5];
            let mut idx = 0;

            fn parse_duty(duty: &[u8], idx: usize) -> u16 {
                if idx > 0 {
                    str::from_utf8(&duty[0..idx])
                        .ok()
                        .unwrap()
                        .parse::<u16>()
                        .unwrap_or(65535)
                } else {
                    0
                }
            }

            loop {
                match keypad.read_keyup() {
                    Some(0xC) => {
                        clear!(now);
                        break;
                    }
                    Some(0xD) => {
                        duty.fill(48);
                        idx = 0;
                        channel.set_duty(0);
                    }
                    Some(0xE) => {
                        channel.set_duty(parse_duty(&duty, idx));
                        idx = 0;
                        duty.fill(48);
                        continue;
                    }
                    Some(key @ 0..=9) if idx < 5 => {
                        duty[idx] = key + 48;
                        idx += 1;
                    }
                    _ => continue,
                }

                cur!(8, 16);
                draw!(8, 16, &[0u64; 5]);
                draw!(fmt "{:05}", parse_duty(&duty, idx));
                update!();
            }
        }

        for offset in 0..64u16 {
            draw!(0, 16, &[!(u64::MAX >> offset)]);
            update!();

            for i in 0..400 {
                delay.delay_us(10);
                channel.set_duty(i + i * offset);
            }
        }

        for offset in (0..64u16).rev() {
            draw!(0, 16, &[!(u64::MAX >> offset)]);
            update!();

            for i in (0..400).rev() {
                delay.delay_us(10);
                channel.set_duty(i + i * offset);
            }
        }

        delay.delay_ms(1000);
    }
}
