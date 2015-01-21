#![allow(unstable)]

use tgaimage::{Image, Color};

mod tgaimage;

fn main() {
    let mut img = Image::new(10, 10, 3);
    img.set(2, 2, &Color::rgba(255, 0, 0, 0));
    img.scale(256, 256);
    img.write_tga_file("rle.tga", true);
    img.write_tga_file("norle.tga", false);
}
