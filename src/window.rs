use image;
use image::Pixel;
use num_complex::Complex64;

use color::{MnColor, MnSmoothScale};
use fractal::{MnPoint, MnComputation};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};


pub trait Window {
    fn new(width: u32, height: u32, max_step: u64) -> Self;
    fn scope(&mut self, top_left: Complex64, size: f64);
    fn fill(&self) -> image::RgbImage;
    fn set_antialiasing(&mut self, aa: u32);
    fn calc_pixel(&self, x: u32, y: u32) -> image::Rgb<u8>;
}

pub struct ImageWindow {
    pub dims: (u32, u32),
    top_left: Complex64,
    win_re_size: f64,
    max_step: u64,
    pub antialiasing: u32,
}

struct StepIterator {
    curr: f64,
    max: f64,
    step: f64,
}

impl Iterator for StepIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.curr;
        self.curr += self.step;
        if item < self.max { Some(item) } else { None }
    }
}

fn step_iterator(start: f64, max: f64, step: f64) -> StepIterator {
    StepIterator { curr: start, max: max, step: step }
}

impl Window for ImageWindow {
    fn new(width: u32, height: u32, max_step: u64) -> ImageWindow {
        ImageWindow {
            dims: (width, height),
            top_left: Complex64::new(0.0, 0.0),
            win_re_size: 1.0,
            antialiasing: 1,
            max_step: max_step,
        }
    }

    fn calc_pixel(&self, x: u32, y: u32) -> image::Rgb<u8> {
        // println!("Calculating pixel ({}, {})", x, y);
        // start offset from top_left
        let base_off_x = ((x as f64) / (self.dims.0 as f64)) * self.win_re_size;
        let base_off_y = -((y as f64) / (self.dims.1 as f64)) * (self.win_re_size * (self.dims.1 as f64 / self.dims.0 as f64));

        // end offset from top_left
        let next_off_x = (((x + 1) as f64) / (self.dims.0 as f64)) * self.win_re_size;
        let next_off_y = -(((y + 1) as f64) / (self.dims.1 as f64)) * (self.win_re_size * (self.dims.1 as f64 / self.dims.0 as f64));

        // total size of a pixel in the fractal
        let range_x = next_off_x - base_off_x;
        let range_y = next_off_y - base_off_y;

        // Size of an iteration for antialiasing
        let range_aa_x = range_x / self.antialiasing as f64;
        let range_aa_y = range_y / self.antialiasing as f64;

        // counters for pixel colors
        let mut r = 0_u64;
        let mut g = 0_u64;
        let mut b = 0_u64;

        for off_x in step_iterator(0.0, range_x, range_aa_x) {
            for off_y in step_iterator(0.0, range_y.abs(), range_aa_y.abs()) {
                let color = MnSmoothScale::from_point(
                    MnComputation::new(Complex64::new(
                        self.top_left.re + base_off_x + off_x,
                        self.top_left.im + base_off_y - off_y,
                    )).to_threshold(self.max_step)
                );
                let channels = color.channels();
                r += channels[0] as u64;
                g += channels[1] as u64;
                b += channels[2] as u64;
            }
        }

        let aa_div = (self.antialiasing * self.antialiasing) as u64;
        return image::Rgb::<u8>::from_channels(
            (r / aa_div) as u8,
            (g / aa_div) as u8,
            (b / aa_div) as u8,
            0
        );
    }

    fn set_antialiasing(&mut self, aa: u32) {
        self.antialiasing = aa;
    }

    fn scope(&mut self, top_left: Complex64, size: f64) {
        self.top_left = top_left;
        self.win_re_size = size;
    }

    fn fill(&self) -> image::RgbImage {
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(self.dims.0, self.dims.1);

        let mut row : Vec<image::Rgb<u8>>;
        for x in 0..self.dims.0 {
            row = (0..self.dims.1).collect::<Vec<u32>>().par_iter().map(move |oy| {
                let y = oy.to_owned();
                print!("\r{:.1}%", (((x) * self.dims.1 + y) as f32 / (self.dims.0 * self.dims.1) as f32) * 100.0);
                self.calc_pixel(x, y)
            }).collect();
            for y in 0..self.dims.1 {
                imgbuf.put_pixel(x, y, row[y as usize]);
            }
        }
        println!("\rDone .");
        imgbuf
    }
}
