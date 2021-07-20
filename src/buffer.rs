use crate::color::Color;

pub struct Buffer {
    buf: Vec<u32>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            buf: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn blit(&mut self, x: i32, y: i32, color: Color) { // blit pixel to buffer with 0,0 as the center

        let x: usize = (self.width as i32 / 2 + x) as usize;
        let y: usize = (self.height as i32 / 2 - y) as usize;
        self.buf[y * self.width + x] = color.to_u32();

    }

    pub fn clear(&mut self) {
        for i in self.mut_buffer() {
            *i = 0;
        }
    }


    pub fn buffer(&self) -> &[u32] {
        &self.buf
    }

    pub fn mut_buffer(&mut self) -> &mut [u32] {
        &mut self.buf
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}