A binary for interacting with the NZXT Kraken X Series under linux.

The CLI provides the following commands:

- `info`

This will print the information about the Kraken device - including current fan
and pump speed, liquid temp and the firmware version.

- `fan`

The command takes the optional argument `SPEED` and will set the fan speed to
the given value (as a percent) or print the current fan speed if none is
provided.

- `pump`

The command operates the same as `fan` and takes the optional argument `SPEED`,
this will set the pump speed to the given value (as a percent) or print the
current pump speed if none is provided.

- `temp`

This command reports the current liquid temperature in degrees celcius.