
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
    }
}

fn main() {
    let kraken = nzxt::Kraken::open().unwrap();
    
    let data = kraken.read().unwrap();
    
    println!("DATA: {:?}", data);

    

    


}