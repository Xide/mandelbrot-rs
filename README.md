# Mandelbrot-rs

Simple Mandelbrot fractal generator developped in Rust for training purposes.

## Gallery

**Complete Mandelbrot set**

![](./gallery/uhd_complete.png)
coords: `( re: -2.5, im: 1.25), size: 4.5`

**Near the "Valley of seahorses"**

![](./gallery/seahorse.png)
coords: `( re: -1.0, im: 0.33), size: 0.5`


## Requirements
- nightly rustc compiler + cargo : [Install instructions](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)

## Build
```sh
cargo build --release
./target/release/mandelbrot --help
```

## Usage

```

# mandelbrot --help

# valley of seahorses example
mandelbrot -r 1920x1080 -a 8 -i 5000 -o seahorses.png -s "(-1.0, 0.33)" -w 0.5

```

## Todo
- Optimization: cycle checking in divergence test
- Incremental / resumable fractal computation
- Interface: Dynamic exploration of the set
- Image: Add color methods / custom colors
- Cli: shortcuts for common resolutions
- Corresponding Julia set explorer ?
- Video generation ?
