#![allow(unstable)]

use tgaimage::{Image, Color};
use geom::GeomActions;
use vec::Vec2i;

mod tgaimage;
mod geom;
mod vec;

fn main() {
    let mut img = Image::new(10, 10, 3);
    img.set(2, 2, &Color::rgba(255, 0, 0, 0));
    img.scale(256, 256);
    img.line(Vec2i::new(10, 220), Vec2i::new(200, 120), &Color::rgba(0, 255, 0, 0));
    img.flip_vertically();
    img.write_tga_file("rle.tga", true);
    img.write_tga_file("norle.tga", false);
}
