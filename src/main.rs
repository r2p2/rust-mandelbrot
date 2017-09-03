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
    zoom:     f32,
}

struct Mandelbrot {
    conf: Config,
}

impl Mandelbrot {
    pub fn new(width: u32, center_x: f32, center_y: f32, zoom: f32) -> Mandelbrot {
        let height = (width as f32 / 1.75) as u32;
        Mandelbrot {
            conf: Config {
                max_iter: 256u16,
                w: width,
                h: height,
                center_x: center_x,
                center_y: center_y,
                zoom: zoom,
            },
        }
    }

    pub fn width(&self) -> u32 {
        self.conf.w
    }

    pub fn height(&self) -> u32 {
        self.conf.h
    }

    pub fn render(&self) -> image::GrayImage {
        let mut imgbuf = image::ImageBuffer::new(self.width(), self.height());

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let iterations = Self::calc_pixel(&self.conf, x as f32, y as f32);
            *pixel = image::Luma([iterations as u8]);
        }

        imgbuf
    }

    fn calc_pixel(conf: &Config, x: f32, y: f32) -> u16 {
        let scaled_x = (1.0 / conf.zoom * 3.5) / conf.w as f32;
        let scaled_y = (1.0 / conf.zoom * 2.0) / conf.h as f32;
/*
        let x = i as f32 % conf.w as f32;
        let y = i as f32 / conf.w as f32;
*/
        let cx = (x as f32 * scaled_x - (1.0 / conf.zoom * 1.75)) + conf.center_x;
        let cy = (y as f32 * scaled_y - (1.0 / conf.zoom * 1.00)) + conf.center_y;

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

        iter
    }
}

fn main() {
    let mb = Mandelbrot::new(4000, -0.75, 0.00, 1.00);

    let imgbuf = mb.render();
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
