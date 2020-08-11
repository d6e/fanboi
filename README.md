# Fanboi
A linux PID controller for fans.

## What is This?
I simple fan controller I wrote for fun for my [rockpro64](https://www.pine64.org/rockpro64).

## Features
- Runs as a systemd service.
- Tuneable PID controller.
- 🦀

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