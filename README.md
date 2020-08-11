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
    -c, --config <PATH>          Sets a custom config file
    -d, --dvalue <D>             Sets the D gain value of the PID controller.
    -i, --ivalue <I>             Sets the I gain value of the PID controller.
    -p, --pvalue <P>             Sets the P gain value of the PID controller.
    -t, --target_temp <VALUE>    Target temperature. Default is 40°C
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