extern crate num;
extern crate image;

use std::fs::File;
use std::path::Path;

use num::complex::Complex;

struct Config {
    max_iter: u16,
    w: u32,
    h: u32,
    center_x: f32,
    center_y: f32,
}

struct Mandelbrot {
    conf: Config,
    data: Vec<u16>,
}

impl Mandelbrot {
    pub fn new(width: u32, center_x: f32, center_y: f32) -> Mandelbrot {
        let height = (width as f32 / 1.75) as u32;
        Mandelbrot {
            conf: Config {
                max_iter: 256u16,
                w: width,
                h: height,
                center_x: center_x,
                center_y: center_y,
            },
            data: vec!(0; (width*height) as usize),
        }
    }

    pub fn width(&self) -> u32 {
        self.conf.w
    }

    pub fn height(&self) -> u32 {
        self.conf.h
    }

    pub fn get(&self, x: u32, y: u32) -> u16 {
        self.data[(y*self.conf.w + x) as usize]
    }

    pub fn render(&mut self) {
        for (i, mut item) in self.data.iter_mut().enumerate() {
            *item = Self::calc_pixel(&self.conf, i);
        }
    }

    fn calc_pixel(conf: &Config, i: usize) -> u16 {
        let scaled_x = 3.5 / conf.w as f32;
        let scaled_y = 2.0 / conf.h as f32;

        let x = i as f32 % conf.w as f32;
        let y = i as f32 / conf.w as f32;

        let cx = (x as f32 * scaled_x - 1.75) + conf.center_x;
        let cy = (y as f32 * scaled_y - 1.00) + conf.center_y;

        let c = Complex::new(cx, cy);
        let mut z = c.clone();

        let mut iter = 0;
        for i in 0..conf.max_iter {
            if z.norm() > 2.0 {
                break;
            }

            z = z*z + c;
            iter = i;
        }

        conf.max_iter-iter
    }
}


fn main() {
    let mut mb = Mandelbrot::new(4000, -0.75, 0.00);
    mb.render();

    let mut imgbuf = image::ImageBuffer::new(mb.width(), mb.height());
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([mb.get(x, y) as u8]);
    }

    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
