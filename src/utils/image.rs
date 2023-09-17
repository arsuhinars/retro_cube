use super::color::{Color, PixelData};

pub struct Image {
    width: usize,
    height: usize,
    data: Box<[PixelData]>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut vec = Vec::new();
        vec.resize(width * height, 0);

        Image {
            width, height, data: vec.into_boxed_slice()
        }
    }

    #[inline]
    pub fn get_width(&self) -> usize { self.width }

    #[inline]
    pub fn get_height(&self) -> usize { self.height }

    #[inline]
    pub fn read_pixel(&self, x: usize, y: usize) -> PixelData {
        self.data[x + y * self.width]
    }

    #[inline]
    pub fn read_color(&self, x: usize, y: usize) -> Color {
        Color::from(self.read_pixel(x, y))
    }

    #[inline]
    pub fn write_pixel(&mut self, x: usize, y: usize, data: PixelData) {
        self.data[x + y * self.width] = data;
    }

    #[inline]
    pub fn write_color(&mut self, x: usize, y: usize, color: Color) {
        self.write_pixel(x, y, color.into());
    }

    pub fn iter(&self) -> ImageIter {
        ImageIter { image: self, curr_x: 0, curr_y: 0 }
    }
}

pub struct ImageIter<'a> {
    image: &'a Image,
    curr_x: usize,
    curr_y: usize
}

impl<'a> Iterator for ImageIter<'a> {
    type Item = (usize, usize, Color);

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(
            (self.curr_x, self.curr_y, self.image.read_color(self.curr_x, self.curr_y))
        );

        self.curr_x += 1;
        if self.curr_x == self.image.width {
            self.curr_y += 1;
            self.curr_x = 0;
            if self.curr_y == self.image.height {
                return None;
            }
        }

        return result;
    }
}
