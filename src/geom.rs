use tgaimage::{Color, Image};
use std::old_io::IoResult;
use std::mem::swap;
use std::num::{SignedInt, NumCast};
pub use vec::*;

pub trait GeomActions {
    fn line<T: NumCast>(&mut self, from: Vec2<T>, to: Vec2<T>, c: &Color) -> IoResult<()>;
    fn triangle(&mut self, mut t0: Vec2<isize>, mut t1: Vec2<isize>, mut t2: Vec2<isize>, c: &Color) -> IoResult<()>;
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

    fn triangle(&mut self, mut t0: Vec2<isize>, mut t1: Vec2<isize>, mut t2: Vec2<isize>, c: &Color) -> IoResult<()> {
        if t0.y == t1.y && t0.y == t2.y {
            //return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Degenerated triangle", detail: None})
            return Ok(())
        }
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
        }

        try!(self.line(t0, t1, c));
        try!(self.line(t1, t2, c));
        try!(self.line(t2, t0, c));

        let total_height = t2.y - t0.y;

        for i in range(0, total_height) {
            let second_half = i > t1.y - t0.y || t1.y == t0.y;
            let segment_height = if second_half { t2.y - t1.y } else { t1.y - t0.y };
            let alpha = i as f32 / total_height as f32;
            let beta = if second_half {
                (i + t0.y - t1.y) as f32 / segment_height as f32
            } else {
                i as f32 / segment_height as f32
            };

            let mut a = t0 + (t2 - t0) * alpha;
            let mut b = if second_half {
                t1 + (t2 - t1) * beta
            } else {
                t0 + (t1 - t0) * beta
            };

            if a.x > b.x {
                swap(&mut a, &mut b);
            }

            for j in range(a.x, b.x) {
                try!(self.set(j as usize, t0.y as usize + i as usize, c));
            }
        }

        Ok(())
    }
}
