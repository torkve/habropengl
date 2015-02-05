#![feature(io)]
#![feature(core)]

use tgaimage::Image;
use model::Model;
use render::Renderer;

mod tgaimage;
mod geom;
mod vec;
mod model;
mod render;

fn main() {
    let width = 800;
    let height = 800;
    let mut img = Image::new(width, height, 3);
    let model = Model::new("african_head.obj").unwrap();
    println!("Loaded {} faces, {} verts", model.nfaces(), model.nverts());
    img.render(model).unwrap();
    img.flip_vertically().unwrap();
    img.write_tga_file("rle.tga", true).unwrap();
    img.write_tga_file("norle.tga", false).unwrap();
}
