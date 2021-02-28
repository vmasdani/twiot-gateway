use std::{fs::File, io::Read, time::Duration};

use embedded_graphics::{drawable::Drawable, image::Image, pixelcolor::BinaryColor, prelude::Point};
use image::{Luma, imageops::{self, FilterType}};
use linux_embedded_hal::I2cdev;
use qrcode::QrCode;
use ssd1306::{Builder, I2CDIBuilder, mode::GraphicsMode};
use tinybmp::Bmp;

pub async fn run_loop() {
    loop {
        match machine_ip::get() {
            Some(ip) => println!("Local ip: {}", ip),
            _ => println!("Failed getting IP."),
        }

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
                println!("i2c OK");

                let mut disp: GraphicsMode<_, _> =
                    Builder::new().connect(I2CDIBuilder::new().init(i2c)).into();

                match disp.init() {
                    Ok(_) => {
                        println!("OLED init OK");

                        disp.flush().unwrap();
                        let qr_file = File::open("./qr.bmp");

                        match qr_file {
                            Ok(mut file) => {
                                let mut buf = Vec::new() as Vec<u8>;
                                file.read_to_end(&mut buf);

                                let bmp = Bmp::from_slice(&buf).expect("Failed to parse BMP image");

                                let image: Image<Bmp, BinaryColor> =
                                    Image::new(&bmp, Point::zero());
                                image.draw(&mut disp);
                            }
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        }

                        disp.flush().unwrap();
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        tokio::time::delay_for(Duration::from_secs(10)).await;
    }
}
