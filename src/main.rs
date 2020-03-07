
pub mod nzxt {
    const NZXT_VENDOR_ID: u16 = 0x1e71;
    const NZXT_KRAKEN_X_PRODUCT_ID: u16 = 0x170e;

    pub struct Kraken {
        device: hidapi::HidDevice,
    }

    #[derive(Debug)]
    pub struct KrakenData {
        liquid_temp: u8,
        fan_speed: u16,
        pump_speed: u16,
        firmware_version: (u8, u16, u8),
    }

    impl Kraken {
        pub fn open() -> Result<Kraken, hidapi::HidError> {
            let api = hidapi::HidApi::new()?;
            let device = api.open(NZXT_VENDOR_ID, NZXT_KRAKEN_X_PRODUCT_ID)?;

            Ok(Kraken {
                device,
            })
        }

        pub fn read(&self) -> Result<KrakenData, hidapi::HidError> {
            let mut buf = [0u8; 64];
            let res = self.device.read_timeout(&mut buf, 1000)?;
            
            if res < 0x0f {
                // We don't have enough data to extract the values we need - something went wrong.
                return Err(hidapi::HidError::HidApiError{
                    message: format!("Only got {} bytes back from the API.", res),
                });
            }

            Ok(KrakenData {
                liquid_temp: buf[1],
                fan_speed: (buf[3] as u16) << 8 | buf[4] as u16,
                pump_speed: (buf[5] as u16) << 8 | buf[6] as u16,
                firmware_version: (buf[0x0b], (buf[0x0c] as u16) << 8 | buf[0x0d] as u16, buf[0x0e]),
            })
        }

        pub fn set_fan_speed(&self, fan_speed: u8) -> Result<(), hidapi::HidError> {
            if fan_speed < 25 || fan_speed > 100 {
                return Err(hidapi::HidError::HidApiError{
                    message: format!("Fan speed must be between 25% and 100%."),
                });
            }
            
            let mut buf = [0u8; 5];
            buf[0] = 0x02;
            buf[1] = 0x4d;
            buf[2] = 0x00;
            buf[3] = 0x00;
            buf[4] = fan_speed;

            let res = self.device.write(&buf)?;

            if res != buf.len() {
                return Err(hidapi::HidError::HidApiError{
                    message: format!("Could not write all of the message to the device."),
                });
            }

            Ok(())
        }

        pub fn set_pump_speed(&self, pump_speed: u8) -> Result<(), hidapi::HidError> {
            if pump_speed < 60 || pump_speed > 100 {
                return Err(hidapi::HidError::HidApiError{
                    message: format!("Pump speed must be between 60% and 100%."),
                });
            }
            
            let mut buf = [0u8; 5];
            buf[0] = 0x02;
            buf[1] = 0x4d;
            buf[2] = 0x00;
            buf[3] = 0x00;
            buf[4] = pump_speed;

            let res = self.device.write(&buf)?;

            if res != buf.len() {
                return Err(hidapi::HidError::HidApiError{
                    message: format!("Could not write all of the message to the device."),
                });
            }

            Ok(())
        }
    }
}

fn main() {
    let kraken = nzxt::Kraken::open().unwrap();
    
    let data = kraken.read().unwrap();
    
    println!("DATA: {:?}", data);

    kraken.set_fan_speed(25).unwrap();

    


}