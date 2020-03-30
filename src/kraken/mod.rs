use std::error;
use std::fmt;

const NZXT_VENDOR_ID: u16 = 0x1e71;
const NZXT_KRAKEN_X_PRODUCT_ID: u16 = 0x170e;

#[derive(Debug)]
pub struct KrakenData {
    pub liquid_temp: u8,
    pub fan_speed: u16,
    pub pump_speed: u16,
    pub firmware_version: (u8, u16, u8),
}

#[derive(Debug)]
pub enum KrakenError {
    FanSpeedOutOfRange,
    PumpSpeedOutOfRange,
    Comms,
    UsbError(hidapi::HidError),
}

impl fmt::Display for KrakenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KrakenError::FanSpeedOutOfRange => write!(f, "Fan speed must be between 25% and 100%"),
            KrakenError::PumpSpeedOutOfRange => write!(f, "Pump speed must be between 60% and 100%"),
            KrakenError::Comms => write!(f, "Did not receive enough data from the device"),
            KrakenError::UsbError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for KrakenError {
    fn source(&self) ->Option<&(dyn error::Error + 'static)> {
        match *self {
            KrakenError::UsbError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<KrakenError> for std::io::Error {
    fn from (error: KrakenError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}

pub struct Kraken {
    device: hidapi::HidDevice,
}

impl Kraken {
    pub fn open() -> Result<Kraken, KrakenError> {
        let api = match hidapi::HidApi::new() {
            Ok(r) => r,
            Err(e) => return Err(KrakenError::UsbError(e)),
        };
        let device = match api.open(NZXT_VENDOR_ID, NZXT_KRAKEN_X_PRODUCT_ID) {
            Ok(r) => r,
            Err(e) => return Err(KrakenError::UsbError(e)),
        };

        Ok(Kraken {
            device,
        })
    }

    pub fn read(&self) -> Result<KrakenData, KrakenError> {
        let mut buf = [0u8; 64];
        let res = match self.device.read_timeout(&mut buf, 1000) {
            Ok(r) => r,
            Err(e) => return Err(KrakenError::UsbError(e)),
        };
        
        if res < 0x0f {
            // We don't have enough data to extract the values we need - something went wrong.
            return Err(KrakenError::Comms);
        }

        Ok(KrakenData {
            liquid_temp: buf[1],
            fan_speed: (buf[3] as u16) << 8 | buf[4] as u16,
            pump_speed: (buf[5] as u16) << 8 | buf[6] as u16,
            firmware_version: (buf[0x0b], (buf[0x0c] as u16) << 8 | buf[0x0d] as u16, buf[0x0e]),
        })
    }

    pub fn set_fan_speed(&self, fan_speed: u8) -> Result<(), KrakenError> {
        if fan_speed < 25 || fan_speed > 100 {
            return Err(KrakenError::FanSpeedOutOfRange);
        }
        
        let mut buf = [0u8; 5];
        buf[0] = 0x02;
        buf[1] = 0x4d;
        buf[2] = 0x00;
        buf[3] = 0x00;
        buf[4] = fan_speed;

        let res = match self.device.write(&buf) {
            Ok(r) => r,
            Err(e) => return Err(KrakenError::UsbError(e)),
        };

        if res != buf.len() {
            return Err(KrakenError::Comms);
        }

        Ok(())
    }

    pub fn set_pump_speed(&self, pump_speed: u8) -> Result<(), KrakenError> {
        if pump_speed < 60 || pump_speed > 100 {
            return Err(KrakenError::PumpSpeedOutOfRange);
        }
        
        let mut buf = [0u8; 5];
        buf[0] = 0x02;
        buf[1] = 0x4d;
        buf[2] = 0x00;
        buf[3] = 0x00;
        buf[4] = pump_speed;

        let res = match self.device.write(&buf) {
            Ok(r) => r,
            Err(e) => return Err(KrakenError::UsbError(e)),
        };

        if res != buf.len() {
            return Err(KrakenError::Comms);
        }

        Ok(())
    }
}