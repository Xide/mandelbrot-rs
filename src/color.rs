use image;
use image::{Rgb};
use image::Pixel;

use fractal::{MnComputation};

pub trait MnColor {
    fn from_point(p: Option<MnComputation>) -> Rgb<u8>;
}

pub struct MnSmoothScale;

impl MnColor for MnSmoothScale {
    fn from_point(p: Option<MnComputation>) -> Rgb<u8> {
        match p {
            _ if p.is_none() => Rgb::<u8>::from_channels(0, 0, 0, 0),
            op => {
                let p = op.unwrap();
                let c = (
                    p.step as f64 -
                    (p.zn.norm().log(10.0) / 2.0_f64.log(10.0)).log(2.0)
                ) as u8;
                image::Rgb::<u8>::from_channels(c, c, c, 0)
            }
        }
    }
}
