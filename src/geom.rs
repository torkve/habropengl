use tgaimage::{Color, Image};
use std::old_io::IoResult;
use std::mem::swap;
use std::num::{SignedInt, NumCast};
pub use vec::*;

pub trait GeomActions {
    fn line<T: NumCast>(&mut self, from: Vec2<T>, to: Vec2<T>, c: &Color) -> IoResult<()>;
}

impl GeomActions for Image {
    fn line<T: NumCast>(&mut self, from: Vec2<T>, to: Vec2<T>, c: &Color) -> IoResult<()> {
        let mut steep = false;
        let mut x0: isize = NumCast::from(from.x).unwrap();
        let mut x1: isize = NumCast::from(to.x).unwrap();
        let mut y0: isize = NumCast::from(from.y).unwrap();
        let mut y1: isize = NumCast::from(to.y).unwrap();

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
