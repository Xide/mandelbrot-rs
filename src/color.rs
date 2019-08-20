use image;
use image::Pixel;
use image::Rgb;

use fractal::MnComputation;

pub trait MnColor {
    fn from_point(&self, p: Option<MnComputation>) -> Rgb<u8>;
}

pub struct MnSmoothScale {
    start: (u8, u8, u8, u8),
    end: (u8, u8, u8, u8),
}

impl MnSmoothScale {
    pub fn new(start: (u8, u8, u8, u8), end: (u8, u8, u8, u8)) -> MnSmoothScale {
        MnSmoothScale { start, end }
    }
}

impl MnColor for MnSmoothScale {
    fn from_point(&self, p: Option<MnComputation>) -> Rgb<u8> {
        match p {
            _ if p.is_none() => Rgb::<u8>::from_channels(0, 0, 0, 0),
            op => {
                let p = op.unwrap();
                let c =
                    (p.step as f64 - (p.zn.norm().log(10.0) / 2.0_f64.log(10.0)).log(2.0)) as u8;
                let (start, end) = (self.start, self.end);
                let nc = 1.0 - (f64::from(c) / 255.0);
                image::Rgb::<u8>::from_channels(
                    ((f64::from(end.0) * nc) + (f64::from(start.0) * (1.0 - nc))) as u8,
                    ((f64::from(end.1) * nc) + (f64::from(start.1) * (1.0 - nc))) as u8,
                    ((f64::from(end.2) * nc) + (f64::from(start.2) * (1.0 - nc))) as u8,
                    ((f64::from(end.3) * nc) + (f64::from(start.3) * (1.0 - nc))) as u8,
                )
            }
        }
    }
}

// pub struct MnNorm;
//
// fn hsb_to_rgba(hue: f32, saturation: f32, brightness: f32) -> (f32, f32, f32, f32) {
//     let chroma: f32 = saturation * brightness;
//     let h: f32 = hue / 60.0;
//     let x: f32 = chroma * (1.0 - ((h % 2.0) - 1.0).abs());
//     let raw_rgb : (f32, f32, f32) = match h {
//         _ if h <= 0.0 && h <= 1.0 => (chroma, x, 0.0),
//         _ if h <= 1.0 && h <= 2.0 => (x, chroma, 0.0),
//         _ if h <= 2.0 && h <= 3.0 => (0.0, chroma, x),
//         _ if h <= 3.0 && h <= 4.0 => (0.0, x, chroma),
//         _ if h <= 4.0 && h <= 5.0 => (x, 0.0, chroma),
//         _ if h <= 6.0 && h <= 6.0 => (chroma, 0.0, x),
//         _ => (0.0, 0.0, 0.0),
//     };
//     let m = brightness - chroma;
//     (raw_rgb.0 + m, raw_rgb.1 + m, raw_rgb.2 + m, 0.0)
// }
//
// impl MnColor for MnNorm {
//     fn from_point(p: Option<MnComputation>) -> Rgb<u8> {
//         match p {
//             _ if p.is_none() => Rgb::<u8>::from_channels(0, 0, 0, 0),
//             op => {
//                 let p = op.unwrap();
//                 let c = p.step as f64 - (p.zn.norm().log(10.0) / 2.0_f64.log(10.0)).log(2.0);
//                 let (r, g, b, a) = hsb_to_rgba(0.95 + 10.0 * (c as f32), 0.6, 1.0);
//                 image::Rgb::<u8>::from_channels(r as u8, g as u8, b as u8, a as u8)
//             }
//         }
//     }
// }
