# NZXT Kraken X Series API and CLI.

## API

The API is centred around the `Kraken` struct and allows you to read the current
data with the `read()` method. 

Consumers can also set the fan and pump speeds with the `set_fan_speed` and
`set_pump_speed` methods.

## CLI

This also includes a binary for interacting with the API from the command line
under linux.

The CLI provides the following commands:

#### `info`

This will print the information about the Kraken device - including current fan
and pump speed, liquid temp and the firmware version.

#### `fan`

The command takes the optional argument `SPEED` and will set the fan speed to
the given value (as a percent) or print the current fan speed if none is
provided.

#### `pump`

The command operates the same as `fan` and takes the optional argument `SPEED`,
this will set the pump speed to the given value (as a percent) or print the
current pump speed if none is provided.

#### `temp`

This command reports the current liquid temperature in degrees celcius.

## Credits

This was developed referencing the [NZXT Kraken protocol
documentation](https://github.com/KsenijaS/krakenx/blob/master/doc/protocol.md)
by [Ksenijas@](https://github.com/KsenijaS/).