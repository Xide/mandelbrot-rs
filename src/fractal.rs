use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num_complex::Complex64;

#[derive(Debug, Clone)]
pub struct MnComputation {
    pub step: u64,
    pub zn: Complex64,
    pub c: Complex64,
}

pub trait MnPoint: Sized {
    fn new(seed: Complex64) -> Self;
    // TODO: Turn the option into something like an Either(BailedOut, StillRunning)
    fn to_threshold(&self, max_iter: u64) -> Option<Self>;
    fn bailed_out(&self) -> bool;
    fn is_in_bulb(&self) -> bool;
}

fn mandelbrot_next(z: Complex64, c: Complex64) -> Complex64 {
    (z * z) + c
}

impl MnPoint for MnComputation {
    fn new(seed: Complex64) -> MnComputation {
        MnComputation {
            zn: Complex64::new(0.0, 0.0),
            step: 0,
            c: seed,
        }
    }

    fn to_threshold(&self, max_iter: u64) -> Option<MnComputation> {
        if self.is_in_bulb() {
            return None;
        }
        match (1..max_iter)
            .fold_while(self.clone(), |acc, _idx| {
                if acc.bailed_out() {
                    Done(acc)
                } else {
                    Continue(MnComputation {
                        step: acc.step + 1,
                        zn: mandelbrot_next(acc.zn, acc.c),
                        c: acc.c,
                    })
                }
            })
            .into_inner()
        {
            ref r if r.bailed_out() => Some(r.to_owned()),
            _ => None,
        }
    }

    fn is_in_bulb(&self) -> bool {
        let p = (((self.c.re - 0.25) * (self.c.re - 0.25)) + (self.c.im * self.c.im)).sqrt();

        self.c.re < p - (2.0 * (p * p)) + 0.25
            || ((self.c.re + 1.0) * (self.c.re + 1.0)) + (self.c.im * self.c.im) < (1.0 / 16.0)
    }

    fn bailed_out(&self) -> bool {
        self.zn.norm() > 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diverging_computation_reach_threshold() {
        let c = MnComputation::new(Complex64::new(-0.86, 0.3));

        assert_eq!(c.to_threshold(50).is_none(), false);
    }

    #[test]
    fn converging_computation_doesnt_reach_threshold() {
        let c = MnComputation::new(Complex64::new(-0.1, 0.1));

        assert_eq!(c.to_threshold(50).is_none(), true);
    }

}
