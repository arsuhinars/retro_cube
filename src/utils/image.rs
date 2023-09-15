use crate::utils::color::PixelData;
use crate::utils::color::Color;

pub struct Image {
    width: usize,
    height: usize,
    data: Box<[PixelData]>
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        let mut vec = Vec::new();
        vec.resize(width * height, 0);

        Image {
            width, height, data: vec.into_boxed_slice()
        }
    }

    #[inline]
    fn read_pixel(&self, x: usize, y: usize) -> PixelData {
        self.data[x + y * self.width]
    }

    #[inline]
    fn read_color(&self, x: usize, y: usize) -> Color {
        Color::from_pixel_data(self.read_pixel(x, y))
    }

    #[inline]
    fn write_pixel(&mut self, x: usize, y: usize, data: PixelData) {
        self.data[x + y * self.width] = data;
    }

    #[inline]
    fn write_color(&mut self, x: usize, y: usize, color: Color) {
        self.write_pixel(x, y, color.to_pixel_data());
    }
}
