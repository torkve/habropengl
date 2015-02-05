use std::old_io::IoResult;
use tgaimage::{Image, Color};
use model::Model;
use geom::GeomActions;
use vec::{Vec2, Vec3, Vec3f};
use zbuffer::ZBuffer;

pub trait Renderer : GeomActions {
    fn render(&mut self, model: Model) -> IoResult<()>;
}

impl Renderer for Image {
    fn render(&mut self, model: Model) -> IoResult<()> {
        let light_dir: Vec3f = Vec3::new(0f32, 0f32, -1f32);
        let mut zbuf = ZBuffer::new(self.width, self.height);
        let depth = 256;
        for faceid in range(0, model.nfaces()) {
            let face = model.face(faceid);
            let world_coords = [
                model.vert(face[0]),
                model.vert(face[1]),
                model.vert(face[2]),
                ];
            let mut coords = [
                world_coords[0] + &Vec3::new(1., 1., 1.),
                world_coords[1] + &Vec3::new(1., 1., 1.),
                world_coords[2] + &Vec3::new(1., 1., 1.),
                ];
            for i in range(0, 3) {
                coords[i].scale(self.width as f32 - 1., self.height as f32 - 1., depth as f32 - 1.);
                coords[i] = coords[i] + Vec3::new(0.5, 0.5, 0.5); // for the cast will round it properly
            }

            let mut n: Vec3f = (*world_coords[2] - *world_coords[0]) ^ (*world_coords[1] - *world_coords[0]);
            n = n.normalize();
            let intensity = n.vec_mul(&light_dir);
            if intensity > 0f32 {
                let c = (intensity * 255. + 0.5) as u8;
                try!(self.triangle(
                        coords[0].to(),
                        coords[1].to(),
                        coords[2].to(),
                        &Color::rgba(c, c, c, 0),
                        &mut zbuf,
                        ));
            }
        }
        Ok(())
    }
}
