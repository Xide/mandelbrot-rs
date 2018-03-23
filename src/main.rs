#![feature(slice_patterns)]

#[macro_use]
extern crate clap;
extern crate itertools;
extern crate num_complex;
extern crate image;
extern crate rayon;

// extern crate pbr;

mod fractal;
mod color;
mod window;

use clap::App;
use window::{ImageWindow, Window};
use std::fs::File;
use num_complex::Complex64;

fn parse_dimensions(fmt: &str) -> Option<(u32, u32)> {
    match fmt.split("x").collect::<Vec<&str>>().as_slice() {
        &[w, h] => Some((w, h)),
        _ => None
    }.and_then(|x| {
        let w = x.0.parse::<u32>().unwrap_or(0);
        let h = x.1.parse::<u32>().unwrap_or(0);
        if w == 0 || h == 0 { None } else { Some((w, h)) }
    })
}

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let dims_o = parse_dimensions(matches.value_of("resolution").unwrap_or("1920x1080"));
    let aa = matches.value_of("antialiasing").unwrap_or("1").parse::<u32>().expect("Expected number for anti aliasing");;
    let iterations = matches.value_of("iterations").unwrap_or("1000").parse::<u64>().expect("Expected number for iterations");
    let output_file = matches.value_of("output").unwrap_or("fractal.png");
    dims_o.expect("Expected format <num>x<num>, e.g: 1920x1080");
    let dims = dims_o.unwrap();
    println!("Resolution     : {:?}", dims);
    println!("Anti aliasing  : {:?}", aa);
    println!("Max iterations : {:?}", iterations);

    let mut window : ImageWindow = Window::new(dims.0, dims.1, iterations);
    window.scope(Complex64::new(-2.5, 1.25), 4.5);
    window.set_antialiasing(aa);
    let imgbuf = window.fill();
    let ref mut fout = File::create(output_file).unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();

}
