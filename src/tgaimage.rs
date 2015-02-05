use std::old_io::{File, Reader, IoResult, IoError, IoErrorKind};
use std::old_path::posix::Path;
use std::ops::{Index, IndexMut};
use std::default::Default;
use std::ptr::{set_memory, copy_memory, copy_nonoverlapping_memory};
use std::mem::transmute;

#[packed]
#[derive(PartialEq, Clone, Default)]
struct Header {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: i16,
    color_map_length: i16,
    color_map_depth: u8,
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bits_per_pixel: u8,
    image_descriptor: u8
}

impl Header {
    pub fn from_stream(buf: &mut Reader) -> IoResult<Self> {
        let idl = try!(buf.read_u8());
        let cmt = try!(buf.read_u8());
        let dtc = try!(buf.read_u8());
        let cmo = try!(buf.read_le_i16());
        let cml = try!(buf.read_le_i16());
        let cmd = try!(buf.read_u8());
        let xo = try!(buf.read_le_i16());
        let yo = try!(buf.read_le_i16());
        let w = try!(buf.read_le_i16());
        let h = try!(buf.read_le_i16());
        let bpp = try!(buf.read_u8());
        let id = try!(buf.read_u8());
        Ok(Header {
            id_length: idl,
            color_map_type: cmt,
            data_type_code: dtc,
            color_map_origin: cmo,
            color_map_length: cml,
            color_map_depth: cmd,
            x_origin: xo,
            y_origin: yo,
            width: w,
            height: h,
            bits_per_pixel: bpp,
            image_descriptor: id
        })
    }

    pub fn to_stream(&self, buf: &mut Writer) -> IoResult<()> {
        try!(buf.write_u8(self.id_length));
        try!(buf.write_u8(self.color_map_type));
        try!(buf.write_u8(self.data_type_code));
        try!(buf.write_le_i16(self.color_map_origin));
        try!(buf.write_le_i16(self.color_map_length));
        try!(buf.write_u8(self.color_map_depth));
        try!(buf.write_le_i16(self.x_origin));
        try!(buf.write_le_i16(self.y_origin));
        try!(buf.write_le_i16(self.width));
        try!(buf.write_le_i16(self.height));
        try!(buf.write_u8(self.bits_per_pixel));
        try!(buf.write_u8(self.image_descriptor));
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Color {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
    bytespp: usize
}

impl Color {
    pub fn new() -> Self {
        Color {b: 0, g: 0, r: 0, a: 0, bytespp: 1}
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {b: b, g: g, r: r, a: a, bytespp: 4}
    }

    pub fn from_raw(raw: &[u8], bpp: usize) -> Self {
        let b = raw[0];
        let g = if bpp > 1 { raw[1] } else { 0 };
        let r = if bpp > 2 { raw[2] } else { 0 };
        let a = if bpp > 3 { raw[3] } else { 0 };
        Color {b: b, g: g, r: r, a: a, bytespp: bpp}
    }

    pub fn from_stream(buf: &mut Reader, bpp: usize) -> IoResult<Self> {
        let b = try!(buf.read_u8());
        let g = if bpp > 1 { try!(buf.read_u8()) } else { 0 };
        let r = if bpp > 2 { try!(buf.read_u8()) } else { 0 };
        let a = if bpp > 3 { try!(buf.read_u8()) } else { 0 };
        Ok(Color {b: b, g: g, r: r, a: a, bytespp: bpp})
    }
}

impl Index<usize> for Color {
    type Output = u8;

    fn index<'a>(&'a self, _index: &usize) -> &'a u8 {
        assert!(*_index < 4);

        match *_index {
            0 => &(self.b),
            1 => &(self.g),
            2 => &(self.r),
            3 => &(self.a),
            _ => panic!("Oops!")
        }
    }
}

impl IndexMut<usize> for Color {
    type Output = u8;

    fn index_mut<'a>(&'a mut self, index: &usize) -> &'a mut u8 {
        assert!(*index < 4);

        match *index {
            0 => &mut (self.b),
            1 => &mut (self.g),
            2 => &mut (self.r),
            3 => &mut (self.a),
            _ => panic!("Oops!")
        }
    }
}

const GRAYSCALE: u8 = 1;
const RGB: u8 = 3;
const RGBA: u8 = 4;

#[derive(PartialEq, Clone)]
pub struct Image {
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    bytespp: usize
}

impl Image {
    pub fn read_tga_file(filename: &str) -> IoResult<Self> {
        let p = Path::new(filename);
        let mut f = try!(File::open(&p));
        let header = try!(Header::from_stream(&mut f));

        let w = header.width;
        let h = header.height;
        let bpp = header.bits_per_pixel >> 3;
        if w <= 0 || h <= 0 || (bpp != GRAYSCALE && bpp != RGB && bpp != RGBA) {
            return Err(IoError{kind: IoErrorKind::MismatchedFileTypeForOperation, desc: "Invalid header format", detail: None})
        }

        let w = w as usize;
        let h = h as usize;

        let nbytes: usize = (bpp as usize) * w * h;
        let data = match header.data_type_code {
            2 | 3 => {
                try!(f.read_exact(nbytes))
            },
            10 | 11 => {
                try!(Image::load_rle_data(w, h, bpp as usize, &mut f))
            },
            _ => {
                return Err(IoError{kind: IoErrorKind::MismatchedFileTypeForOperation, desc: "Invalid header format", detail: None})
            }
        };

        let mut result = Image {
            data: data,
            width: w,
            height: h,
            bytespp: bpp as usize
        };

        if (header.image_descriptor & 0x20) == 0 {
            try!(result.flip_vertically());
        }
        if (header.image_descriptor & 0x10) != 0 {
            try!(result.flip_horizontally());
        }

        Ok(result)
    }

    fn load_rle_data<'a>(w: usize, h: usize, bpp: usize, buf: &'a mut Reader) -> IoResult<Vec<u8>> {
        let pixelcount = w * h;
        let mut currentpixel = 0us;
        let mut currentbyte = 0us;

        let mut data = Vec::with_capacity(pixelcount * bpp);
        data.resize(pixelcount * bpp, 0);
        loop {
            let mut chunkheader = try!(buf.read_u8());
            if chunkheader < 128 {
                chunkheader += 1;
                for _ in range(0u8, chunkheader) {
                    for __ in range(0us, bpp) {
                        data[currentbyte] = try!(buf.read_u8());
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Too many pixels read", detail: None})
                    }
                }
            } else {
                chunkheader -= 127;
                let color = try!(Color::from_stream(buf, bpp));
                for _ in range(0u8, chunkheader) {
                    for j in range(0us, bpp) {
                        data[currentbyte] = color[j];
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Too many pixels read", detail: None})
                    }
                }
            }
            if currentpixel == pixelcount {
                break;
            }
        }
        Ok(data)
    }

    pub fn write_tga_file(&self, filename: &str, rle: bool) -> IoResult<()> {
        let developer_area_ref = [0u8, 0u8, 0u8, 0u8];
        let extension_area_ref = [0u8, 0u8, 0u8, 0u8];
        let footer = "TRUEVISION-XFILE.\0";

        let p = Path::new(filename);
        let mut f = try!(File::create(&p));

        let mut header: Header = Default::default();
        header.bits_per_pixel = (self.bytespp as u8) << 3;
        header.width = self.width as i16;
        header.height = self.height as i16;
        header.data_type_code = if self.bytespp as u8 == GRAYSCALE {
            if rle { 11 } else { 3 }
        } else {
            if rle { 10 } else { 2 }
        };
        header.image_descriptor = 0x20; // top-left origin
        try!(header.to_stream(&mut f));
        if !rle {
            try!(f.write_all(self.data.as_slice()));
        } else {
            try!(self.dump_rle_data(&mut f));
        }
        try!(f.write_all(&developer_area_ref));
        try!(f.write_all(&extension_area_ref));
        try!(f.write_all(footer.as_bytes()));
        Ok(())
    }

    fn dump_rle_data(&self, buf: &mut Writer) -> IoResult<()> {
        const MAX_CHUNK_LENGTH: u8 = 128u8;
        let npixels = self.width * self.height;
        let mut curpix = 0us;
        while curpix < npixels {
            let chunkstart = curpix * self.bytespp;
            let mut curbyte = chunkstart;
            let mut run_length = 1u8;
            let mut raw = true;

            while (curpix + (run_length as usize) < npixels) && run_length < MAX_CHUNK_LENGTH {
                let mut succ_eq = true;
                for t in range(0, self.bytespp) {
                    succ_eq = self.data[curbyte + t] == self.data[curbyte + t + self.bytespp];
                    if !succ_eq {
                        break;
                    }
                }
                curbyte += self.bytespp;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }
                run_length += 1;
            }
            curpix += run_length as usize;
            try!(buf.write_u8(if raw { run_length - 1 } else { run_length + 127 }));
            try!(buf.write_all(&self.data
                           .as_slice()
                           [chunkstart..]
                           [..if raw { (run_length as usize) * self.bytespp } else { self.bytespp }]
                          ));
        }
        Ok(())
    }

    pub fn new(w: usize, h: usize, bpp: usize) -> Self {
        let mut data = Vec::with_capacity(w * h * bpp);
        data.resize(w * h * bpp, 0);
        Image {data: data, width: w, height: h, bytespp: bpp}
    }

    pub fn flip_horizontally(&mut self) -> IoResult<()> {
        let half: usize = self.width >> 1;
        let w = self.width;
        for i in range(0us, half) {
            for j in range(0us, self.height) {
                let c1 = try!(self.get(i, j));
                let c2 = try!(self.get(w - 1 - i, j));
                try!(self.set(i, j, &c2));
                try!(self.set(w - 1 - i, j, &c1));
            }
        }
        Ok(())
    }

    pub fn flip_vertically(&mut self) -> IoResult<()> {
        let half: usize = self.height >> 1;
        let bytes_per_line = self.width * self.bytespp;
        let mut line = Vec::with_capacity(bytes_per_line);
        line.resize(bytes_per_line, 0u8);

        for j in range(0us, half) {
            let l1 = j * bytes_per_line;
            let l2 = (self.height - 1 - j) * bytes_per_line;
            unsafe {
                copy_memory(line.as_mut_ptr(), self.data.as_ptr().offset(l1 as isize), bytes_per_line);
                copy_memory(self.data.as_mut_ptr().offset(l1 as isize), self.data.as_ptr().offset(l2 as isize), bytes_per_line);
                copy_memory(self.data.as_mut_ptr().offset(l2 as isize), line.as_ptr(), bytes_per_line);
            }
        }
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> IoResult<Color> {
        if x >= self.width || y >= self.height {
            return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Wrong coords", detail: None})
        }
        let start = (x + y * self.width) * self.bytespp;
        let bytes = &self.data.as_slice()[start .. start + self.bytespp];
        Ok(Color::from_raw(bytes, self.bytespp))
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) -> IoResult<()> {
        if x >= self.width || y >= self.height {
            return Err(IoError{kind: IoErrorKind::InvalidInput, desc: "Wrong coords", detail: None})
        }
        let start = (x + y * self.width) * self.bytespp;
        unsafe
        {
            let cc: &u8 = transmute(c);
            copy_nonoverlapping_memory(self.data.as_mut_ptr().offset(start as isize), &*cc as *const u8, self.bytespp);
        }
        Ok(())
    }

    pub fn scale(&mut self, w: usize, h: usize) -> IoResult<()> {
        let mut newdata = Vec::with_capacity(w * h * self.bytespp);
        newdata.resize(w * h * self.bytespp, 0u8);

        let mut nscanline = 0is;
        let mut oscanline = 0is;
        let mut erry = 0is;
        let nlinebytes = w * self.bytespp;
        let olinebytes = (self.width * self.bytespp) as isize;

        for _ in range(0, self.height) {
            let mut errx = (self.width as isize) - (w as isize);
            let mut nx = -(self.bytespp as isize);
            let mut ox = -(self.bytespp as isize);
            for __ in range(0, self.width) {
                ox += self.bytespp as isize;
                errx += w as isize;
                while errx >= self.width as isize {
                    errx -= self.width as isize;
                    nx += self.bytespp as isize;
                    unsafe {
                        copy_nonoverlapping_memory(newdata.as_mut_ptr().offset(nscanline + nx), self.data.as_ptr().offset(oscanline + ox), self.bytespp)
                    }
                }
            }
            erry += h as isize;
            oscanline += olinebytes;
            while erry >= self.height as isize {
                if erry >= ((self.height as isize) << 1) {
                    unsafe {
                        copy_nonoverlapping_memory(newdata.as_mut_ptr().offset(nscanline + nlinebytes as isize), newdata.as_ptr().offset(nscanline), nlinebytes);
                    }
                }
                erry -= self.height as isize;
                nscanline += nlinebytes as isize;
            }
        }
        self.data = newdata;
        self.width = w;
        self.height = h;
        Ok(())
    }

    pub fn clear(&mut self) {
        unsafe {
            set_memory(self.data.as_mut_slice().as_mut_ptr(), 0u8, self.data.len())
        }
    }
}
