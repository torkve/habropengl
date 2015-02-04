#![feature(io)]
#![feature(core)]
#![feature(macro_rules)]

use tgaimage::{Image, Color};
use geom::GeomActions;
use vec::Vec2;
use model::Model;

mod tgaimage;
mod geom;
mod vec;
mod model;

fn main() {
    let width = 800;
    let height = 800;
    let white = Color::rgba(255, 0, 0, 0);
    let mut img = Image::new(width, height, 3);
    let model = Model::new("african_head.obj").unwrap();
    println!("Loaded {} faces, {} verts", model.nfaces(), model.nverts());
    for faceid in range(0, model.nfaces()) {
        let face = model.face(faceid);
        for vert in range(0, 3) {
            let mut v0 = model.vert(face[vert]).to_vec2();
            let mut v1 = model.vert(face[(vert + 1) % 3]).to_vec2();

            v0 = v0 + Vec2::new(1., 1.);
            v1 = v1 + Vec2::new(1., 1.);
            v0.scale(width as f32, height as f32);
            v1.scale(width as f32, height as f32);
            println!("Drawing line from {:?} to {:?}", v0, v1);
            img.line(v0, v1, &white);
        }
    }
    img.flip_vertically();
    img.write_tga_file("rle.tga", true);
    img.write_tga_file("norle.tga", false);
}
