#![feature(slice_patterns)]

#[macro_use]
extern crate clap;
extern crate itertools;
extern crate num_complex;
extern crate image;
extern crate rayon;
extern crate regex;

// extern crate pbr;

mod fractal;
mod color;
mod window;

use clap::App;
use window::{ImageWindow, Window};
use std::fs::File;
use num_complex::Complex64;
use regex::Regex;

fn parse_dimensions(fmt: &str) -> Option<(u32, u32)> {
    match *fmt.split('x').collect::<Vec<&str>>().as_slice() {
        [w, h] => Some((w, h)),
        _ => None
    }.and_then(|x| {
        let w = x.0.parse::<u32>().unwrap_or(0);
        let h = x.1.parse::<u32>().unwrap_or(0);
        if w == 0 || h == 0 { None } else { Some((w, h)) }
    })
}

fn parse_seed(fmt: &str) -> Option<Complex64> {
    let re = Regex::new(r"\(\s?(.*)\s?,\s?(.*)\s?\)").unwrap();
    for cap in re.captures_iter(fmt) {
        println!("Parse seed: x: {}, y: {}", &cap[1], &cap[2]);
        return cap[1].parse::<f64>().and_then(|re| {
            cap[2].parse::<f64>().and_then(|im| {
                Ok(Some(Complex64::new(re, im)))
            })

        }).unwrap_or(None);
    }
    None
}

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let dims = parse_dimensions(
        matches
            .value_of("resolution")
            .unwrap_or("1920x1080")
    ).expect("Expected format <num>x<num>, e.g: 1920x1080");

    let aa = matches
        .value_of("antialiasing")
        .unwrap_or("1")
        .parse::<u32>()
        .expect("Expected number for anti aliasing");

    let iterations = matches
        .value_of("iterations")
        .unwrap_or("1000")
        .parse::<u64>()
        .expect("Expected number for iterations");

    let output_file = matches
        .value_of("output")
        .unwrap_or("fractal.png");

    let seed = parse_seed(
        matches
            .value_of("seed")
            .unwrap_or("(-2.5, 1.25)")
    ).expect("Seed format need an argument: '(re, im)'");

    let camera_width = matches
        .value_of("camera_width")
        .unwrap_or("4.5")
        .parse::<f64>()
        .expect("Expected a float number as camera width");


    println!("Resolution     : {:?}", dims);
    println!("Anti aliasing  : {:?}", aa);
    println!("Max iterations : {:?}", iterations);
    //

    let mut window : ImageWindow = Window::new(
        dims.0,
        dims.1,
        iterations,
        ((242, 180, 224, 0), (90, 55, 80, 0))

        // ((237, 233, 132, 0), (15, 52, 70, 0))
    );
    window.scope(seed, camera_width);
    window.set_antialiasing(aa);
    let imgbuf = window.fill();

    println!("Exporting image to {}", output_file);
    let fout = &mut File::create(output_file).unwrap();
    image::ImageRgb8(imgbuf)
        .save(fout, image::PNG)
        .unwrap();

}
