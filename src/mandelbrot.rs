use std::vec;

use crate::complex::Complex;
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Mandelbrot {
    iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    size: usize,
    mandelbrot_color_values: Vec<u8>,
}

impl Mandelbrot {
    #[allow(dead_code)]
    pub fn new(iters: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64, size: usize) -> Self {
        Self {
            iters,
            x_min,
            x_max,
            y_min,
            y_max,
            size,
            mandelbrot_color_values: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn mandelbrot_values_at_point(&mut self, cx: f64, cy: f64, iters: usize) -> usize {
        let c_complex = Complex::new(cx, cy, 1);
        let mut z_complex = Complex::new(0.0, 0.0, 1);

        for i in 0..iters {
            if Complex::complex_abs(z_complex) > 2.0 {
                return i;
            }

            z_complex = z_complex * z_complex + c_complex;
        }
        iters
    }
    #[allow(dead_code)]
    pub fn calculate_mandelbort(&mut self) {
        let mut mandelbrot_result = Vec::new();
        for img_y in 0..self.size {
            for img_x in 0..self.size {
                let cx = self.x_min + (self.x_max - self.x_min) * (img_x as f64 / self.size as f64);
                let cy = self.y_min + (self.y_max - self.y_min) * (img_y as f64 / self.size as f64);
                let temp_value = self.mandelbrot_values_at_point(cx, cy, self.iters);
                mandelbrot_result.push(temp_value);
            }
        }
        self.mandelbrot_color_values = self.get_mandelbrot_color(mandelbrot_result);
    }

    #[allow(dead_code)]
    pub fn mandelbrot_values_to_rgb(&mut self, value: usize) -> Vec<u8> {
        let color_value: f64 = (value - 1) as f64 / (self.iters as f64 - 1.0);
        match color_value {
            color_value if color_value < 0.1 => vec![0, 7, 100, 155],
            color_value if color_value < 0.42 => vec![32, 107, 203, 255],
            color_value if color_value < 0.6425 => vec![237, 255, 255, 255],
            color_value if color_value < 0.98 => vec![255, 170, 0, 255],
            _ => vec![0, 0, 0, 255],
        }
    }
    pub fn return_mandelbrot_color_values(&mut self) -> Vec<u8> {
        self.mandelbrot_color_values.clone()
    }

    #[allow(dead_code)]
    pub fn get_mandelbrot_color(&mut self, mandelbrot_list: Vec<usize>) -> Vec<u8> {
        mandelbrot_list
            .iter()
            .map(|value| self.mandelbrot_values_to_rgb(*value))
            .collect::<Vec<_>>()
            .concat()
    }
}
