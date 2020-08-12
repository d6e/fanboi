# Fanboi 🌬🖥️🌡️
A linux PID controller for fans.

## What is This?
I simple fan controller I wrote for fun for my [rockpro64](https://www.pine64.org/rockpro64). While maxing out all six cores with a tall heatsink, the controller seems to settle on a pwm value of 121 and a temperature of 61°C. With the default target temperature value being 40°C, at idle the fan doesn't even run. I'm not sure how it would work on other boards. Those have yet to be tested. USE AT YOUR OWN RISK.

## Features
- Runs as a systemd service.
- Tuneable PID controller.
- 🦀

## Usage
```
Fanboi - A fan PID controller

USAGE:
    fanboi [FLAGS] [OPTIONS]

FLAGS:
        --dry-run    Set program the dry run.
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -c, --cpu-temp-ctl <CPU_TEMP_FILE>    The CPU temperature file. Default='/sys/class/thermal/thermal_zone0/temp'
    -d, --dvalue <D>                      Sets the D gain value of the PID controller.
    -f, --fan-pwm-ctl <FAN_FILE>          Fan control file. Default='/sys/class/hwmon/hwmon0/pwm1'
    -g, --gpu-temp-ctl <CPU_TEMP_FILE>    The GPU temperature file. Default='/sys/class/thermal/thermal_zone1/temp'
    -i, --ivalue <I>                      Sets the I gain value of the PID controller.
        --minimum-pwm <PWM>               The minimum control output pwm before activating the fan. Default='50'
        --poll-interval-secs <SECONDS>    The frequency at which to poll the temperature and update fan pwm.
                                          Default='10'
    -p, --pvalue <P>                      Sets the P gain value of the PID controller.
    -t, --target-temp <VALUE>             Target temperature. Default=40°C
```

## Docker
Comes with a nix config for generating a minimalist docker image.
```
nix-build docker.nix && docker load -i ./result
```

## Development
If you use Nix, the `shell.nix` config will give you a development environment:
```
nix-shell
```

Otherwise, if you're on anything else:
```
cargo build
```