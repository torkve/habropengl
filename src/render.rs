extern crate rand;

use std::old_io::IoResult;
use self::rand::random;
use tgaimage::{Image, Color};
use model::Model;
use geom::GeomActions;
use vec::Vec2;

pub trait Renderer : GeomActions {
    fn render(&mut self, model: Model) -> IoResult<()>;
}

impl Renderer for Image {
    fn render(&mut self, model: Model) -> IoResult<()> {
        for faceid in range(0, model.nfaces()) {
            let face = model.face(faceid);
            let mut coords = [
                model.vert(face[0]).to_vec2(),
                model.vert(face[1]).to_vec2(),
                model.vert(face[2]).to_vec2(),
                ];
            for i in range(0, 3) {
                coords[i] = coords[i] + Vec2::new(1., 1.);
                coords[i].scale(self.width as f32 - 1., self.height as f32 - 1.);
                coords[i] = coords[i] + Vec2::new(0.5, 0.5); // for the cast will round it properly
            }
            try!(self.triangle(coords[0].to(), coords[1].to(), coords[2].to(), &Color::rgba(random::<u8>(), random::<u8>(), random::<u8>(), 0)));
        }
        Ok(())
    }
}
