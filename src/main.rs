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
        let c = ((1.0 - (1.0 / (point.step as f64))) * 255.0) as u8;
        image::Rgb::<u8>::from_channels(c, c, c, 0)
    }
}

fn main() {
    let wx = 3840;
    let wy = 2160;
    // let wx = 1280;
    // let wy = 720;

    let range_re = 1.75;
    let range_im = range_re * ((wy as f64) / (wx as f64));

    let off_re = -0.75;
    let off_im = 0.0;

    let step_re = (2.0 * range_re) / (wx as f64);
    let step_im = (2.0 * range_im) / (wy as f64);

    let mut pb = ProgressBar::new(wx);

    // let mut imgbuf: image::RgbImage = image::ImageBuffer::new(wx, wy);
    let pixels : Vec<Vec<image::Rgb<u8>>> = (0..wx).map(
        move |ox| {
            let x = ox.to_owned();
            pb.inc();
            (0..wy).collect::<Vec<u64>>().par_iter().map( move |oy| {
                let y = oy.to_owned();
                to_rgb_color(
                    mandelbrot(
                        Complex64::new(
                            off_re + ((x as f64) * step_re) - range_re,
                            off_im + ((y as f64) * step_im) - range_im
                        ),
                        2000
                    )
                )
            }).collect()
        }
    ).collect();
    println!("{:?}", pixels);
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     *pixel = to_rgb_color(
    //         mandelbrot(
    //             Complex64::new(
    //                 off_re + ((x as f64) * step_re) - range_re,
    //                 off_im + ((y as f64) * step_im) - range_im
    //             ),
    //             2000
    //         )
    //     );
    //     println!("({}, {})", x, y);
    // }
    //
    // println!("Hello, world!");
    // let ref mut fout = File::create("fractal.png").unwrap();
    // image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();

}
