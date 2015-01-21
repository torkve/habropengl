use tgaimage::{Color, Image};
use std::io::{IoResult, IoError, IoErrorKind};
use std::mem::swap;
use std::num::SignedInt;

pub trait GeomActions {
    fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, c: &Color) -> IoResult<()>;
}

impl GeomActions for Image {
    fn line(&mut self, _x0: usize, _y0: usize, _x1: usize, _y1: usize, c: &Color) -> IoResult<()> {
        let mut steep = false;
        let mut x0 = _x0;
        let mut x1 = _x1;
        let mut y0 = _y0;
        let mut y1 = _y1;

        if (x0 as isize - x1 as isize).abs() < (y0 as isize - y1 as isize).abs() {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 as isize - x0 as isize;
        let dy = y1 as isize - y0 as isize;
        let derror2 = dy.abs() * 2;

        let mut error2 = 0;
        let mut y = y0;

        for x in range(x0, x1 + 1) {
            if steep {
                try!(self.set(y, x, c));
            } else {
                try!(self.set(x, y, c));
            }
            error2 += derror2;

            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
        Ok(())
    }
}
