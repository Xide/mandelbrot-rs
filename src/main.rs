extern crate itertools;
extern crate num_complex;
extern crate image;
extern crate rayon;
extern crate pbr;

use pbr::ProgressBar;
use std::fs::File;
use itertools::FoldWhile::{Done, Continue};
use itertools::Itertools;
use num_complex::Complex64;
use image::Pixel;
use rayon::prelude::*;

#[derive(Debug)]
struct MnIteration {
    step: u64,
    zn: Complex64,
}

fn mandelbrot_next(z: Complex64, c: Complex64) -> Complex64 {
    (z * z) + c
}

fn mandelbrot(seed: Complex64, iterations: u64) -> MnIteration {
    (1..iterations)
        .fold_while(
            MnIteration { zn: Complex64::new(0.0, 0.0), step: 0 },
            |acc, idx| {
                let item = mandelbrot_next(acc.zn, seed);
                let new_acc = MnIteration { zn: item, step: idx };
                let beyond_threshold = item.re.powi(2) + item.im.powi(2) > 4.0;
                if beyond_threshold { Done(new_acc) } else { Continue(new_acc) }
            }
    ).into_inner()
}

fn to_rgb_color(point: MnIteration) -> image::Rgb<u8> {
    let n = point.zn.norm();
    if n < 2.0 { image::Rgb::<u8>::from_channels(0, 0, 0, 0) }
    else {
        // let c = ((1.0 - (1.0 / (point.step as f64))) * 255.0) as u8;
        let c = (
            point.step as f64 -
            (point.zn.norm().log(10.0) / 2.0_f64.log(10.0)).log(2.0)
        ) as u8;
        image::Rgb::<u8>::from_channels(c, c, c, 0)
    }
}

fn main() {
    let wx = 36000;
    let wy = 20250;
    // let wx = 1280;
    // let wy = 720;

    let range_re = 3.5;
    let range_im = range_re * ((wy as f64) / (wx as f64));

    let off_re = -0.75;
    let off_im = 0.0;

    let step_re = (2.0 * range_re) / (wx as f64);
    let step_im = (2.0 * range_im) / (wy as f64);

    let mut pb = ProgressBar::new(wx);

    let pixels : Vec<Vec<image::Rgb<u8>>> = (0..wx).map(
        move |ox| {
            let x = ox.to_owned();
            pb.inc();
            if x == wx - 1 {pb.finish_print("Done, exporting image.")}
            (0..wy).collect::<Vec<u64>>().par_iter().map(move |oy| {
                let y = oy.to_owned();
                to_rgb_color(
                    mandelbrot(
                        Complex64::new(
                            off_re + ((x as f64) * step_re) - range_re,
                            off_im + ((y as f64) * step_im) - range_im
                        ),
                        5000
                    )
                )
            }).collect()
        }
    ).collect();

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(wx as u32, wy as u32);

    (0..wx).collect::<Vec<u64>>().iter().map(
        |ox| {
            let x = ox.to_owned();
            (0..wy).collect::<Vec<u64>>().iter().map( |oy| {
                let y = oy.to_owned();
                imgbuf.put_pixel(x as u32, y as u32, pixels[x as usize][y as usize].to_owned());
                ()
            }).collect::<()>();
            ()
        }
    ).collect::<()>();

    let ref mut fout = File::create("fractal.png").unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();

}
