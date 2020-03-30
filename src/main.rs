use clap::{Arg, App, AppSettings, SubCommand};

mod kraken;


fn main() -> Result<(), std::io::Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("info")
            .about("Prints info about the Kraken device."))
        .subcommand(SubCommand::with_name("pump")
            .about("Reports or sets the speed of the pump. This can be set between 60% and 100%.")
            .arg(Arg::with_name("SPEED")
                    .required(false)
                    .help("Pump speed to set - 60-100 (percent)")))
        .subcommand(SubCommand::with_name("fan")
            .about("Reports or sets the speed of the fan. This can be set between 25% and 100%.")
            .arg(Arg::with_name("SPEED")
                    .required(false)
                    .help("Fan speed to set - 25-100 (percent)")))
        .subcommand(SubCommand::with_name("temp")
            .about("Reports or the liquid temperature of the cooler in degrees celcius."))
        .get_matches();

    let kraken = kraken::Kraken::open()?;

    if matches.subcommand_matches("info").is_some() {
        let data = kraken.read()?;
        println!("{0: <15}{1} rpm", "Fan Speed:", data.fan_speed);
        println!("{0: <15}{1} rpm", "Pump Speed:", data.pump_speed);
        println!("{0: <15}{1} C", "Liquid Temp:", data.liquid_temp);
        println!("{0: <15}{1}.{2}.{3}", "Version:", data.firmware_version.0,
                                                    data.firmware_version.1,
                                                    data.firmware_version.2);
    }
    else if let Some(cmd) = matches.subcommand_matches("pump") {
        match cmd.value_of("SPEED") {
            Some(speed) => {
                let pct = match speed.parse::<u8>() {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
                    }
                };
                kraken.set_pump_speed(pct)?;
            }
            None => {
                let data = kraken.read()?;
                println!("{}", data.pump_speed);
            }
        }
    }
    else if let Some(cmd) = matches.subcommand_matches("fan") {
        match cmd.value_of("SPEED") {
            Some(speed) => {
                let pct = match speed.parse::<u8>() {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
                    }
                };
                kraken.set_fan_speed(pct)?;
            }
            None => {
                let data = kraken.read()?;
                println!("{}", data.fan_speed);
            }
        }
    }
    else if matches.subcommand_matches("temp").is_some() {
        let data = kraken.read()?;
        println!("{}", data.liquid_temp);
    }

    Ok(())
}