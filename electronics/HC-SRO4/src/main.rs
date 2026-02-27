use esp_idf_hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(feature = "esp32")]
    log::info!("--- Running on ESP32 DevKit ---");

    #[cfg(feature = "esp32c3")]
    log::info!("--- Running on ESP32-C3 SuperMini ---");

    loop {
        log::info!("ping");
        FreeRtos::delay_ms(1000);
    }
}
