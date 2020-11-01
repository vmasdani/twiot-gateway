use std::time::Duration;

pub async fn run_loop() {
    loop {
        println!("Checking IP...");
        // TODO: IP checking code
        // TODO: transform IP into QR
        // TODO: Display to SSD1306
        tokio::time::delay_for(Duration::from_secs(10)).await; 
    }
}