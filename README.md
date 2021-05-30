# Mandelbrot-rs

Simple Mandelbrot fractal generator developped in Rust for training purposes.

## Gallery

**Complete Mandelbrot set**

![](./gallery/uhd_complete.png)
Command (default coordinates): `mandelbrot -s '(-0.25,0.0)' -w 4.5`

> For an UHD high quality version you can run this command:
  `mandelbrot -r 7680x4320 -i 25000 -a 16`
  This command will take **MUCH LONGER** to run (34mn with a Ryzen 3900X), and will stress the CPU during all the process.

**Near the "Valley of seahorses"**

![](./gallery/seahorse.png)
command: `mandelbrot -s '(-0.75, 0.18)' -w 0.5`


## Requirements
*One* of:
- rustc compiler + cargo : [Install instructions](https://github.com/rust-lang-nursery/rustup.rs)
- Docker (tested on 19.03.1-ce)


## Build

```sh
git clone https://github.com/Xide/mandelbrot-rs.git
cd mandelbrot-rs
```

- With Cargo

```sh
cargo build --release
./target/release/mandelbrot --help
```

- With Docker
```sh
docker build -t mandelbrot .
docker run -it mandelbrot --help
```

## Usage

```

# mandelbrot --help

# valley of seahorses example

## With binary
mandelbrot -r 1920x1080 -a 8 -i 5000 -o seahorses.png -s '(-0.75, 0.18)' -w 0.5

## With Docker
docker run -it sigbilly/mandelbrot:master -r 1920x1080 -a 8 -i 5000 -o seahorses.png -s '(-0.75, 0.18)' -w 0.5

```

## Todo
- [x] Optimization: cycle checking in divergence test
- [ ] Incremental / resumable fractal computation
- [ ] Interface: Dynamic exploration of the set
- [ ] Image: Add color methods / custom colors
- [ ] Cli: shortcuts for common resolutions
- [ ] Corresponding Julia set explorer ?
- [ ] Video generation ?
