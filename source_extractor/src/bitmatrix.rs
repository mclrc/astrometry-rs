use std::fmt::Display;

pub struct BitMatrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl BitMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![0; (width * height + 7) / 8];

        BitMatrix {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let offset = y * self.width + x;
        let byte = offset / 8;
        let bit = 7 - offset % 8;
        self.data[byte] & (1 << bit) != 0
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let offset = y * self.width + x;
        let byte = offset / 8;
        let bit = 7 - offset % 8;

        let value = if value { 1 } else { 0 } << bit;

        self.data[byte] |= value;
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Display for BitMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get(x, y) { "1" } else { "0" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
