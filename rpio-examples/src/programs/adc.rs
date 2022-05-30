use crate::io::*;

use embedded_hal::adc::{Channel, OneShot};
use rp2040_hal::adc::Adc;

pub fn adc<S, D, K, AdcChannel>(io: Io<S, D, K>, mut adc: Adc, mut adc_pin: AdcChannel) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
    AdcChannel: Channel<Adc, ID = u8>,
{
    setup!(io => delay, screen: ScaledBuf::new(), _keypad);

    offset!(4, 4);

    loop {
        let mut temperature_sensor = adc.enable_temp_sensor();
        let temp_sens_adc_counts: u16 = adc.read(&mut temperature_sensor).unwrap();
        let pin_adc_counts: u16 = adc.read(&mut adc_pin).unwrap();

        clear!();
        draw!(fmtln "Temp {}", temp_sens_adc_counts);
        draw!(fmtln "Vin {:04}", pin_adc_counts);
        update!();

        delay.delay_ms(100);
    }
}
