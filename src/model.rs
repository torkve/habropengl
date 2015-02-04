#![feature(macro_rules)]

use std::vec::Vec;
use std::slice::SliceExt;
use std::str::FromStr;
use vec::{Vec3, Vec3f};
use std::path::posix::Path;
use std::old_io::{File, BufferedReader, IoResult, IoError, IoErrorKind};

#[derive(Debug)]
pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<usize>>,
}

macro_rules! try_parse (
    ($f:expr) => (
        try!(FromStr::from_str(($f).trim()).map_err(|_| IoError{kind: IoErrorKind::InvalidInput, desc: "Invalid file format", detail: None}))
    )
);

macro_rules! try_parse_opt (
    ($f:expr) => (
        match ($f) {
            None => return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Invalid file format", detail: None}),
            Some(x) => try_parse!(x)
        }
    )
);

impl Model {
    pub fn new(filename: &str) -> IoResult<Model> {
        let p = Path::new(filename);
        let mut f = BufferedReader::new(File::open(&p));
        let mut verts = Vec::new();
        let mut faces = Vec::new();

        for line in f.lines() {
            let parts: Vec<&str> = line.as_ref().unwrap().split(' ').filter(|x| !x.is_empty()).collect();
            if parts.len() == 0 {
                continue
            }
            if parts[0] == "v" && parts.len() > 3 {
                let v = Vec3::new(
                    try_parse!(parts[1]),
                    try_parse!(parts[2]),
                    try_parse!(parts[3]),
                    );
                verts.push(v);
            } else if parts[0] == "f" && parts.len() > 5 {
                let mut indices: Vec<usize> = Vec::new();
                for part in parts[1..].iter() {
                    let idx = part.split('/').next();
                    indices.push(try_parse_opt!(idx) - 1);
                }
                faces.push(indices);
            }
        }
        Ok(Model {verts: verts, faces: faces})
    }

    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(& self, idx: usize) -> &Vec<usize> {
        &self.faces[idx]
    }
}
