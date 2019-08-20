use image;
use image::Pixel;
use num_complex::Complex64;

use color::{MnColor, MnSmoothScale};
use fractal::{MnComputation, MnPoint};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait Window {
    fn new(
        width: u32,
        height: u32,
        max_step: u64,
        color_gradient: ((u8, u8, u8, u8), (u8, u8, u8, u8)),
    ) -> Self;
    fn scope(&mut self, focus: Complex64, size: f64);
    fn fill(&self) -> image::RgbImage;
    fn set_antialiasing(&mut self, aa: u32);
    fn calc_pixel(&self, x: u32, y: u32) -> image::Rgb<u8>;
}

pub struct ImageWindow {
    pub dims: (u32, u32),
    focus: Complex64,
    win_re_size: f64,
    max_step: u64,
    pub antialiasing: u32,
    color_gradient: ((u8, u8, u8, u8), (u8, u8, u8, u8)),
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
        if item < self.max {
            Some(item)
        } else {
            None
        }
    }
}

fn step_iterator(start: f64, max: f64, step: f64) -> StepIterator {
    StepIterator {
        curr: start,
        max,
        step,
    }
}

impl Window for ImageWindow {
    fn new(
        width: u32,
        height: u32,
        max_step: u64,
        color_gradient: ((u8, u8, u8, u8), (u8, u8, u8, u8)),
    ) -> ImageWindow {
        ImageWindow {
            dims: (width, height),
            focus: Complex64::new(0.0, 0.0),
            win_re_size: 1.0,
            antialiasing: 1,
            max_step,
            color_gradient,
        }
    }

    fn calc_pixel(&self, x: u32, y: u32) -> image::Rgb<u8> {
        // start offset from top left
        let base_off_x = (f64::from(x) / f64::from(self.dims.0)) * self.win_re_size;
        let base_off_y = -(f64::from(y) / f64::from(self.dims.1))
            * (self.win_re_size * (f64::from(self.dims.1) / f64::from(self.dims.0)));

        // end offset from top left
        let next_off_x = ((f64::from(x + 1)) / f64::from(self.dims.0)) * self.win_re_size;
        let next_off_y = -((f64::from(y + 1)) / f64::from(self.dims.1))
            * (self.win_re_size * (f64::from(self.dims.1) / f64::from(self.dims.0)));

        // total size of a pixel in the fractal space
        let range_x = next_off_x - base_off_x;
        let range_y = next_off_y - base_off_y;

        // Size of an iteration for antialiasing
        let range_aa_x = range_x / f64::from(self.antialiasing);
        let range_aa_y = range_y / f64::from(self.antialiasing);

        // counters for pixel colors
        let mut r_counter = 0_u64;
        let mut g_counter = 0_u64;
        let mut b_counter = 0_u64;

        // Color each pixel based on it's escape speed
        let color_picker = MnSmoothScale::new(self.color_gradient.0, self.color_gradient.1);

        let center_offset_x = self.win_re_size / 2.0;
        // Account for image resolution
        let center_offset_y = (self.win_re_size * (self.dims.1 as f64 / self.dims.0 as f64)) / 2.0;

        for off_x in step_iterator(0.0, range_x, range_aa_x) {
            for off_y in step_iterator(0.0, range_y.abs(), range_aa_y.abs()) {
                // NOTE: The [-/+] signs in the computation are
                //       inverted for real and imaginary axis
                //       because the algorithm start from image
                //       top left corner. Therefore, x axis will
                //       grow while the y axis will shrink.
                // coordination pick:
                // 0. Get the center of the screen (self.focus.xx)
                // 1. Translate that point to the upper left of the
                //    final image ([-/+]center_offset_xx)
                // 2. Translate that point to the upper left of the
                //    pixel (+base_off_xx)
                // 3. Translate that point to the upper left of the
                //    current iteration for anti aliasing ([-/+]off_xx)
                let color = color_picker.from_point(
                    MnComputation::new(Complex64::new(
                        self.focus.re - center_offset_x + base_off_x + off_x,
                        self.focus.im + center_offset_y + base_off_y - off_y,
                    ))
                    .to_threshold(self.max_step),
                );

                // Merge all the anti-aliased rays channels.
                let channels = color.channels();
                r_counter += u64::from(channels[0]);
                g_counter += u64::from(channels[1]);
                b_counter += u64::from(channels[2]);
            }
        }

        // Average anti-aliased rays colors into a pixel
        let aa_div = u64::from(self.antialiasing * self.antialiasing);
        image::Rgb::<u8>::from_channels(
            (r_counter / aa_div) as u8,
            (g_counter / aa_div) as u8,
            (b_counter / aa_div) as u8,
            0,
        )
    }

    fn set_antialiasing(&mut self, aa: u32) {
        self.antialiasing = aa;
    }

    fn scope(&mut self, focus: Complex64, size: f64) {
        self.focus = focus;
        self.win_re_size = size;
    }

    fn fill(&self) -> image::RgbImage {
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(self.dims.0, self.dims.1);

        let pb = ProgressBar::new(self.dims.0 as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .progress_chars("#>-"));
        let mut row: Vec<image::Rgb<u8>>;
        for x in 0..self.dims.0 {
            row = (0..self.dims.1)
                .collect::<Vec<u32>>()
                .par_iter()
                .map(move |oy| {
                    let y = oy.to_owned();
                    self.calc_pixel(x, y)
                })
                .collect();
            pb.set_position(x as u64);
            for y in 0..self.dims.1 {
                imgbuf.put_pixel(x, y, row[y as usize]);
            }
        }
        pb.finish_with_message("Done");
        imgbuf
    }
}
