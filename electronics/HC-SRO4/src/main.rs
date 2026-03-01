use esp_idf_hal::delay::{Ets, FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_sys::esp_timer_get_time;

pub struct Hcsr04<'a> {
    trig: PinDriver<'a, Gpio5, Output>,
    echo: PinDriver<'a, Gpio18, Input>,
}

impl<'a> Hcsr04<'a> {
    pub fn new(
        mut trig: PinDriver<'a, Gpio5, Output>,
        echo: PinDriver<'a, Gpio18, Input>,
    ) -> anyhow::Result<Self> {
        trig.set_low()?;
        FreeRtos::delay_ms(50); // Give the sensor a moment to settle
        Ok(Self { trig, echo })
    }

    pub fn measure_distance_cm(&mut self) -> anyhow::Result<Option<f64>> {
        // 1. Send the 10 microsecond trigger pulse
        self.trig.set_high()?;
        Ets::delay_us(10); // Precise microsecond blocking delay
        self.trig.set_low()?;

        // 2. Wait for the ECHO pin to go HIGH (Start of the ping)
        let mut timeout = 10_000;
        while self.echo.is_low() {
            if timeout == 0 {
                return Ok(None); // Timeout waiting for pulse to start
            }
            Ets::delay_us(1);
            timeout -= 1;
        }

        // Record the exact time the pulse started
        let start_time = unsafe { esp_timer_get_time() };

        // 3. Wait for the ECHO pin to go LOW (End of the ping)
        // 30_000 microseconds is roughly equivalent to a 5-meter distance limit
        timeout = 30_000;
        while self.echo.is_high() {
            if timeout == 0 {
                return Ok(None); // Timeout waiting for pulse to end (out of range)
            }
            Ets::delay_us(1);
            timeout -= 1;
        }

        // Record the exact time the pulse ended
        let end_time = unsafe { esp_timer_get_time() };

        // 4. Calculate the distance
        let duration_us = end_time - start_time;
        if duration_us > 0 {
            // HC-SR04 formula: distance in cm = time in microseconds / 58
            let distance = duration_us as f64 / 58.0;
            Ok(Some(distance))
        } else {
            Ok(None)
        }
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let trig = PinDriver::output(peripherals.pins.gpio5)?;
    let echo = PinDriver::input(peripherals.pins.gpio18)?;

    let mut sensor = Hcsr04::new(trig, echo)?;

    #[cfg(feature = "esp32")]
    log::info!("--- Running on ESP32 DevKit ---");

    loop {
        match sensor.measure_distance_cm() {
            Ok(Some(distance)) => log::info!("Distance: {:.2} cm", distance),
            Ok(None) => log::info!("Out of range"),
            Err(e) => log::error!("Error: {:?}", e),
        }
        FreeRtos::delay_ms(500);
    }
}
