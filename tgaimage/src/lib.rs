use std::io::Write;
use std::mem::size_of;
use std::ops::{Index, IndexMut, Mul};
use std::ptr;
use std::ptr::slice_from_raw_parts;

/// TGA image header
#[derive(Default)]
#[repr(packed)]
pub struct TGAHeader {
    pub idlength: u8,
    pub colormaptype: u8,
    pub datatypecode: u8,
    pub colormaporigin: u16,
    pub colormaplength: u16,
    pub colormapdepth: u8,
    pub x_origin: u16,
    pub y_origin: u16,
    pub width: u16,
    pub height: u16,
    pub bitsperpixel: u8,
    pub imagedescriptor: u8,
}

/// Color channel indexes representation of a TGA image
#[derive(Debug, Copy, Clone)]
pub enum ColorChannel {
    /// Red channel
    R = 2,
    /// Green channel
    G = 1,
    /// Blue channel
    B = 0,
    /// Alpha channel
    A = 3,
}

/// TGA image format
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TGAImageFormat {
    Unknown = 0,
    Grayscale = 1,
    RGB = 3,
    RGBA = 4,
}

impl Default for TGAImageFormat {
    fn default() -> Self {
        TGAImageFormat::Unknown
    }
}

/// TGA image color representation
#[derive(Default, Debug, Copy, Clone)]
pub struct TGAColor {
    /// BGRA array
    bgra: [u8; 4],
    /// Bytes per pixel value
    bytespp: u8,
}

impl Mul<f32> for TGAColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let mut color = self;
        let clamped = 0.0f32.max(rhs.min(1.0f32));

        color
            .bgra
            .iter_mut()
            .for_each(|elem| *elem = ((*elem as f32) * clamped) as u8);

        color
    }
}

impl Mul<f64> for TGAColor {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut color = self;
        let clamped = 0.0f64.max(rhs.min(1.0f64));

        color
            .bgra
            .iter_mut()
            .for_each(|elem| *elem = ((*elem as f64) * clamped) as u8);

        color
    }
}

impl Index<ColorChannel> for TGAColor {
    type Output = u8;

    fn index(&self, index: ColorChannel) -> &Self::Output {
        &self.bgra[index as usize]
    }
}

impl IndexMut<ColorChannel> for TGAColor {
    fn index_mut(&mut self, index: ColorChannel) -> &mut Self::Output {
        &mut self.bgra[index as usize]
    }
}

impl TGAColor {
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        TGAColor {
            bgra: [b, g, r, a],
            bytespp: 4,
        }
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new_rgba(r, g, b, 255)
    }

    pub fn new_from_iter<'a, I>(values: I, bytespp: u8) -> Self
    where
        I: Iterator<Item = &'a u8> + Clone,
    {
        assert!(bytespp <= 4);
        let mut it = values.clone();
        let size = values.count();
        assert_eq!(size as u8, bytespp);
        let mut bgra = [u8::default(); 4];

        for i in 0..size {
            bgra[i] = *it.next().unwrap();
        }

        TGAColor { bgra, bytespp }
    }
}

/// TGA image representation
pub struct TGAImage {
    data: Vec<u8>,
    /// Width of an image
    width: u32,
    /// Height of an image
    height: u32,
    /// TGA image color format
    bytespp: TGAImageFormat,
}

impl TGAImage {
    pub fn new(width: u32, height: u32, bytespp: TGAImageFormat) -> Self {
        TGAImage {
            data: vec![0; (width * height * bytespp as u32) as usize],
            width,
            height,
            bytespp,
        }
    }

    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|e| *e = 0);
    }

    pub fn buffer(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn get_bytespp(&self) -> TGAImageFormat {
        self.bytespp
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set(&mut self, x: u32, y: u32, color: TGAColor) {
        if self.data.len() == 0 || x >= self.width || y >= self.height {
            return;
        }

        let offset = ((x + y * self.width) * self.bytespp as u32) as usize;
        self.data[offset..(offset + self.bytespp as usize)]
            .copy_from_slice(&color.bgra[..(self.bytespp as usize)]);
    }

    pub fn flip_vertically(&mut self) {
        if self.data.len() == 0 {
            return;
        }

        let bytes_per_line = (self.width * self.bytespp as u32) as usize;
        let half = self.height / 2;

        for i in 0..half {
            let line1 = (i * bytes_per_line as u32) as usize;
            let line2 = ((self.height - i - 1) * bytes_per_line as u32) as usize;
            let chunk1 = self.data[line1..(line1 + bytes_per_line)].as_mut_ptr();
            let chunk2 = self.data[line2..(line2 + bytes_per_line)].as_mut_ptr();

            unsafe {
                ptr::swap_nonoverlapping(chunk1, chunk2, bytes_per_line);
            }
        }
    }

    fn unload_rle_data<T: std::io::Write>(&self, out: &mut T) -> std::io::Result<()> {
        const MAX_CHUNK_LENGTH: u8 = 128;
        let npixels: usize = (self.width * self.height) as usize;
        let mut curpix = 0;

        while curpix < npixels {
            let chunkstart = curpix * self.bytespp as usize;
            let mut curbyte = chunkstart;
            let mut run_length = 1u8;
            let mut raw = true;

            while curpix + (run_length as usize) < npixels && run_length < MAX_CHUNK_LENGTH {
                let mut succ_eq = true;

                for i in 0..self.bytespp as usize {
                    succ_eq =
                        self.data[curbyte + i] == self.data[curbyte + i + self.bytespp as usize];

                    if !succ_eq {
                        break;
                    }
                }

                curbyte += self.bytespp as usize;

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

            if raw {
                out.write(&[run_length - 1])?;
            } else {
                out.write(&[run_length + 127])?;
            }

            let to_write = if raw {
                (run_length * self.bytespp as u8) as usize
            } else {
                self.bytespp as usize
            };

            out.write(self.data[chunkstart..chunkstart + to_write].as_ref())?;
        }

        Ok(())
    }

    pub fn write_tga_file(&self, filename: &str, vflip: bool, rle: bool) -> std::io::Result<()> {
        fn get_data_type_code(image_fmt: TGAImageFormat, rle: bool) -> u8 {
            let rle_val = if rle { 11 } else { 3 };
            return if image_fmt == TGAImageFormat::Grayscale {
                rle_val
            } else {
                rle_val - 1
            };
        }

        const DEVELOPER_AREA_REF: [u8; 4] = [0u8; 4];
        const EXTENSION_AREA_REF: [u8; 4] = [0u8; 4];
        const FOOTER: [u8; 18] = [
            b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-', b'X', b'F', b'I',
            b'L', b'E', b'.', b'\0',
        ];

        let mut file = std::fs::File::create(filename)?;
        let mut header = TGAHeader::default();

        header.bitsperpixel = (self.bytespp as u8) << 3;
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.datatypecode = get_data_type_code(self.bytespp, rle);
        header.imagedescriptor = if vflip { 0x0u8 } else { 0x20u8 };

        let header = slice_from_raw_parts(&header as *const _ as *const u8, size_of::<TGAHeader>());

        unsafe {
            file.write(header.as_ref().unwrap())?;
        }

        if !rle {
            file.write(self.data.as_ref())?;
        } else {
            self.unload_rle_data(&mut file)?;
        }

        file.write(&DEVELOPER_AREA_REF)?;
        file.write(&EXTENSION_AREA_REF)?;
        file.write(&FOOTER)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests_tgacolor {
    // use crate::tgaimage::{ColorChannel, TGAColor};
    use super::*;

    #[test]
    fn tgacolor_default() {
        let default = TGAColor::default();

        for e in &default.bgra {
            assert_eq!(*e, 0);
        }

        assert_eq!(default.bytespp, 0);
    }

    #[test]
    fn tgacolor_rgb() {
        let tgacolor = TGAColor::new_rgb(1, 1, 1);

        for e in tgacolor.bgra.iter().take(3) {
            assert_eq!(*e, 1);
        }

        assert_eq!(tgacolor.bgra[3], 255);
        assert_eq!(tgacolor.bytespp, 4);

        let tgacolor = TGAColor::new_rgb(255, 255, 255);

        for e in tgacolor.bgra.iter().take(3) {
            assert_eq!(*e, 255);
        }

        assert_eq!(tgacolor.bgra[3], 255);
        assert_eq!(tgacolor.bytespp, 4);
    }

    fn index_to_rgba_color_channel(index: usize) -> Result<ColorChannel, &'static str> {
        match index {
            0 => Ok(ColorChannel::R),
            1 => Ok(ColorChannel::G),
            2 => Ok(ColorChannel::B),
            3 => Ok(ColorChannel::A),
            _ => Err("Unknown color channel"),
        }
    }

    fn index_to_bgra_color_channel(index: usize) -> Result<ColorChannel, &'static str> {
        match index {
            0 => Ok(ColorChannel::B),
            1 => Ok(ColorChannel::G),
            2 => Ok(ColorChannel::R),
            3 => Ok(ColorChannel::A),
            _ => Err("Unknown color channel"),
        }
    }

    #[test]
    fn tgacolor_rgba() {
        let tgacolor = TGAColor::new_rgba(1, 2, 3, 4);

        for index in 0..tgacolor.bgra.len() {
            let color_index = index_to_rgba_color_channel(index).unwrap();
            assert_eq!(tgacolor[color_index], index as u8 + 1);
        }

        assert_eq!(tgacolor.bytespp, 4);
    }

    #[test]
    fn tgacolor_iter() {
        let bgra1 = [1u8];
        let bgra2 = [1u8, 2];
        let bgra3 = [1u8, 2, 3];
        let bgra4 = [1u8, 2, 3, 4];
        let checker = |arr: &[u8]| {
            let tgacolor = TGAColor::new_from_iter(arr.iter(), arr.len() as u8);

            for index in 0..arr.len() {
                let color_index = index_to_bgra_color_channel(index).unwrap();
                let val = tgacolor[color_index];

                assert_eq!(val, index as u8 + 1);
            }

            assert_eq!(tgacolor.bytespp, arr.len() as u8);
        };

        checker(&bgra1);
        checker(&bgra2);
        checker(&bgra3);
        checker(&bgra4);
    }

    #[test]
    #[should_panic]
    fn tgacolor_iter_fails() {
        let bgra5 = [0u8; 4];
        let bgra6 = [0u8; 6];

        let checker = |arr: &[u8]| {
            TGAColor::new_from_iter(arr.iter(), arr.len() as u8);
        };

        checker(&bgra5);
        checker(&bgra6);
    }

    #[test]
    fn tgacolor_index_access() {
        let tgacolor = TGAColor::new_rgba(128, 128, 128, 128);

        for i in 0..4 {
            let color_index = index_to_rgba_color_channel(i).unwrap();
            assert_eq!(tgacolor[color_index], 128);
        }

        let mut tgacolor = TGAColor::new_rgba(128, 128, 128, 128);

        for i in 0..4 {
            let color_index = index_to_rgba_color_channel(i).unwrap();

            tgacolor[color_index] += 1;
            assert_eq!(tgacolor[color_index], 129);
        }
    }

    #[test]
    fn tgacolor_intensity() {
        let tgacolor = TGAColor::new_rgba(128, 128, 128, 128);
        let expected = 128.0 * 0.5;
        let new_tgacolor: TGAColor = tgacolor * 0.5;

        for i in 0..4 {
            let color_index = index_to_rgba_color_channel(i).unwrap();

            assert_eq!(new_tgacolor[color_index], expected as u8);
        }
    }
}

#[cfg(test)]
mod tests_tgaimage {
    use super::*;

    #[test]
    fn tgaimage_getters() {
        let image = TGAImage::new(100, 100, TGAImageFormat::Grayscale);

        assert_eq!(image.get_width(), 100);
        assert_eq!(image.get_height(), 100);
        assert_eq!(image.get_bytespp(), TGAImageFormat::Grayscale);
    }

    #[test]
    fn tgaimage_buffer() {
        let width = 100;
        let height = 100;
        let format = TGAImageFormat::Grayscale;
        let image = TGAImage::new(width, height, format);

        assert_eq!(image.data.len(), (width * height * format as u32) as usize);

        let format = TGAImageFormat::RGBA;
        let image = TGAImage::new(100, 100, format);

        assert_eq!(image.data.len(), (width * height * format as u32) as usize);
    }

    #[test]
    fn tgaimage_clear() {
        let width = 100;
        let height = 100;
        let format = TGAImageFormat::Grayscale;
        let mut image = TGAImage::new(width, height, format);

        image.buffer().iter_mut().for_each(|e| *e = 255);
        image.buffer().iter().for_each(|e| assert_eq!(*e, 255));
        image.clear();
        image.buffer().iter().for_each(|e| assert_eq!(*e, 0));
    }
}
