use tgaimage::{Color, Image};
use std::io::{IoResult, IoError, IoErrorKind};
use std::mem::swap;
use std::num::SignedInt;
pub use vec::*;

pub trait GeomActions {
    fn line(&mut self, from: Vec2i, to: Vec2i, c: &Color) -> IoResult<()>;
}

impl GeomActions for Image {
    fn line(&mut self, from: Vec2i, to: Vec2i, c: &Color) -> IoResult<()> {
        let mut steep = false;
        let mut x0 = from.x;
        let mut x1 = to.x;
        let mut y0 = from.y;
        let mut y1 = to.y;

        if (x0 - x1).abs() < (y0 - y1).abs() {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;

        let mut error2 = 0;
        let mut y = y0;

        for x in range(x0, x1 + 1) {
            if steep {
                try!(self.set(y as usize, x as usize, c));
            } else {
                try!(self.set(x as usize, y as usize, c));
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
