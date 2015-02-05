use tgaimage::{Color, Image};
use zbuffer::ZBuffer;
use std::old_io::IoResult;
use std::mem::swap;
use std::num::{SignedInt, NumCast};
pub use vec::*;

pub trait GeomActions {
    fn line<T: NumCast>(&mut self, from: Vec2<T>, to: Vec2<T>, c: &Color) -> IoResult<()>;
    fn triangle(&mut self, mut t0: Vec3f, mut t1: Vec3f, mut t2: Vec3f, c: &Color, zbuf: &mut ZBuffer) -> IoResult<()>;
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

    fn triangle(&mut self, mut t0: Vec3f, mut t1: Vec3f, mut t2: Vec3f, c: &Color, zbuf: &mut ZBuffer) -> IoResult<()> {
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

        let total_height = t2.y - t0.y;

        for i in range(0us, total_height as usize) {
            let second_half = i as f32 > t1.y - t0.y || t1.y == t0.y;
            let segment_height = if second_half { t2.y - t1.y } else { t1.y - t0.y };
            let alpha = i as f32 / total_height;
            let beta = if second_half {
                (i as f32 + t0.y - t1.y) / segment_height
            } else {
                i as f32 / segment_height
            };

            let mut a: Vec3i = (t0 + (t2 - t0) * alpha).to();
            let mut b: Vec3i = if second_half {
                (t1 + (t2 - t1) * beta).to()
            } else {
                (t0 + (t1 - t0) * beta).to()
            };

            if a.x > b.x {
                swap(&mut a, &mut b);
            }

            for j in range(a.x as isize, b.x as isize) {
                let phi = if b.x == a.x {
                    1.
                } else {
                    (j - a.x as isize) as f32 / (b.x - a.x) as f32
                };
                let p = (a.to::<f32>() + (b - a).to::<f32>() * phi).to::<isize>();
                if (*zbuf.val(p.x as usize, p.y as usize) as isize) < p.z {
                    *zbuf.val_mut(p.x as usize, p.y as usize) = p.z as i32;
                    try!(self.set(p.x as usize, p.y as usize, c));
                }
            }
        }

        Ok(())
    }
}
