use eframe::{egui, App, Frame, NativeOptions};
use num_complex::Complex;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITER: u32 = 100;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Mandelbrot Set",
        native_options,
        Box::new(|_cc| Box::new(MandelbrotApp::default())),
    );
}

struct MandelbrotApp {
    pixels: Vec<u8>,
    zoom: f64,
}

impl Default for MandelbrotApp {
    fn default() -> Self {
        let pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, 1.0);
        Self { pixels, zoom: 1.0 }
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &self.pixels);
            let texture_handle = ui.ctx().load_texture("mandelbrot", texture, Default::default());
            ui.image(&texture_handle, [WIDTH as f32, HEIGHT as f32]);

            egui::Window::new("Zoom")
                .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
                .fixed_size([200.0, 50.0])
                .show(ctx, |ui| {
                    ui.label(format!("Zoom: {:.2}", self.zoom));
                    if ui
                        .add(egui::Slider::new(&mut self.zoom, 0.1..=5.0).text("Zoom"))
                        .changed()
                    {
                        self.pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, self.zoom);
                    }
                });
        });
    }
}

fn generate_mandelbrot(width: usize, height: usize, max_iter: u32, zoom: f64) -> Vec<u8> {
    let mut pixels = vec![0; width * height * 4];

    for x in 0..width {
        for y in 0..height {
            let cx = (x as f64 / width as f64 - 0.5) * 3.5 / zoom - 0.5;
            let cy = (y as f64 / height as f64 - 0.5) * 2.0 / zoom;
            let c = Complex::new(cx, cy);

            let mut z = Complex::new(0.0, 0.0);
            let mut iter = 0;

            while iter < max_iter && z.norm() <= 2.0 {
                z = z * z + c;
                iter += 1;
            }

            let pixel_index = (y * width + x) * 4;
            let color = if iter < max_iter {
                let t = iter as f64 / max_iter as f64;
                let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
                let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
                let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
                [r, g, b, 255]
            } else {
                [0, 0, 0, 255]
            };

            pixels[pixel_index..pixel_index + 4].copy_from_slice(&color);
        }
    }

    pixels
}
