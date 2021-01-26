use std::time::Duration;

pub async fn run_loop() {
    loop {
        match machine_ip::get() {
            Some(ip) => println!("Local ip: {}", ip),
            _ => println!("Failed getting IP.")
        }
        
        // TODO: IP checking code
        // TODO: transform IP into QR
        // TODO: Display to SSD1306
        
        tokio::time::delay_for(Duration::from_secs(10)).await; 
    }
}