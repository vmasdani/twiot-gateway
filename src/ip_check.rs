use std::{fs::File, io::Read, time::Duration};

use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font12x16, Text},
    image::Image,
    pixelcolor::BinaryColor,
    prelude::Point,
    style::TextStyle,
};
use image::{
    imageops::{self, FilterType},
    Luma,
};
use linux_embedded_hal::I2cdev;
use qrcode::QrCode;
use ssd1306::{mode::GraphicsMode, Builder, I2CDIBuilder};
use tinybmp::Bmp;

pub async fn run_loop() {
    loop {
        let ip = match machine_ip::get() {
            Some(ip) => {
                println!("[ip_check] Local ip: {}", ip);
                ip.to_string()
            }
            _ => {
                println!("[ip_check] Failed getting IP.");
                "".to_string()
            }
        };

        let qr_gen = QrCode::new(match machine_ip::get() {
            Some(ip) => format!("http://{}", ip.to_string()),
            _ => "".to_string(),
        });

        match qr_gen {
            Ok(qr) => {
                let image = qr.render::<Luma<u8>>().max_dimensions(64, 64).build();
                let resized = imageops::resize(&image, 64, 64, FilterType::Nearest);
                resized.save("./qr.bmp");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        match I2cdev::new("/dev/i2c-0") {
            Ok(i2c) => {
                println!("[ip_check] i2c OK");

                let mut disp: GraphicsMode<_, _> =
                    Builder::new().connect(I2CDIBuilder::new().init(i2c)).into();

                match disp.init() {
                    Ok(_) => {
                        println!("[ip_check] OLED init OK");

                        disp.flush().unwrap();
                        let qr_file = File::open("./qr.bmp");

                        match qr_file {
                            Ok(mut file) => {
                                let mut buf = Vec::new() as Vec<u8>;
                                file.read_to_end(&mut buf);

                                let bmp = Bmp::from_slice(&buf).expect("Failed to parse BMP image");

                                let image: Image<Bmp, BinaryColor> =
                                    Image::new(&bmp, Point::zero());

                                for (i, bit) in
                                    ip.split(".").collect::<Vec<&str>>().iter().enumerate()
                                {
                                    println!("[ip_check] {}", bit);

                                    let text_style = TextStyle::new(Font12x16, BinaryColor::On);
                                    let text = Text::new(bit, Point::new(64, 16 * (i as i32)))
                                        .into_styled(text_style);

                                    text.draw(&mut disp);
                                }

                                image.draw(&mut disp);
                            }
                            Err(e) => {
                                println!("[ip_check] {:?}", e);
                            }
                        }

                        disp.flush().unwrap();
                    }
                    Err(e) => {
                        println!("[ip_check] {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("[ip_check] {:?}", e);
            }
        }

        tokio::time::delay_for(Duration::from_secs(10)).await;
    }
}
