use std::old_io::IoResult;
use tgaimage::{Image, Color};
use model::Model;
use geom::GeomActions;
use vec::Vec2;

pub trait Renderer : GeomActions {
    fn render(&mut self, model: Model) -> IoResult<()>;
}

impl Renderer for Image {
    fn render(&mut self, model: Model) -> IoResult<()> {
        let white = Color::rgba(255, 255, 255, 0);
        for faceid in range(0, model.nfaces()) {
            let face = model.face(faceid);
            for vert in range(0, 3) {
                let mut v0 = model.vert(face[vert]).to_vec2();
                let mut v1 = model.vert(face[(vert + 1) % 3]).to_vec2();

                v0 = v0 + Vec2::new(1., 1.);
                v1 = v1 + Vec2::new(1., 1.);
                v0.scale(self.width as f32 - 1., self.height as f32 - 1.);
                v1.scale(self.width as f32 - 1., self.height as f32 - 1.);
                println!("Drawing line from {:?} to {:?}", v0, v1);
                try!(self.line(v0, v1, &white));
            }
        }
        Ok(())
    }
}
