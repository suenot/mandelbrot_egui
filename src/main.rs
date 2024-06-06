use eframe::{egui, App, Frame, NativeOptions};
use num_complex::Complex;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITER: u32 = 100;

// Define a struct to hold gradient color information
#[derive(Clone, Copy, PartialEq)]
struct Gradient {
    pub start_color: [u8; 3],
    pub end_color: [u8; 3],
}

// Define some predefined gradients
const GRADIENT_RAINBOW: Gradient = Gradient {
    start_color: [255, 0, 0],
    end_color: [0, 0, 255],
};
const GRADIENT_PURPLE: Gradient = Gradient {
    start_color: [128, 0, 128],
    end_color: [255, 0, 255],
};
const GRADIENT_GREEN: Gradient = Gradient {
    start_color: [0, 128, 0],
    end_color: [0, 255, 0],
};

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
    selected_gradient: Gradient, // Track the selected gradient separately
}

impl Default for MandelbrotApp {
    fn default() -> Self {
        let pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, 1.0, &GRADIENT_RAINBOW); // Default to rainbow gradient
        Self { pixels, zoom: 1.0, selected_gradient: GRADIENT_RAINBOW }
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
                        self.pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, self.zoom, &self.selected_gradient);
                    }
                });

            // Add a window to select the gradient
            egui::Window::new("Gradient")
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
            .fixed_size([200.0, 100.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.radio(&mut self.selected_gradient == &GRADIENT_RAINBOW, "Rainbow").clicked() {
                        self.selected_gradient = GRADIENT_RAINBOW;
                        self.pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, self.zoom, &self.selected_gradient);
                    }
                    if ui.radio(&mut self.selected_gradient == &GRADIENT_PURPLE, "Purple").clicked() {
                        self.selected_gradient = GRADIENT_PURPLE;
                        self.pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, self.zoom, &self.selected_gradient);
                    }
                    if ui.radio(&mut self.selected_gradient == &GRADIENT_GREEN, "Green").clicked() {
                        self.selected_gradient = GRADIENT_GREEN;
                        self.pixels = generate_mandelbrot(WIDTH, HEIGHT, MAX_ITER, self.zoom, &self.selected_gradient);
                    }
                });
            });
        });
    }
}

fn generate_mandelbrot(width: usize, height: usize, max_iter: u32, zoom: f64, gradient: &Gradient) -> Vec<u8> {
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
                let r = interpolate(gradient.start_color[0], gradient.end_color[0], t);
                let g = interpolate(gradient.start_color[1], gradient.end_color[1], t);
                let b = interpolate(gradient.start_color[2], gradient.end_color[2], t);
                [r, g, b, 255]
            } else {
                [0, 0, 0, 255]
            };

            pixels[pixel_index..pixel_index + 4].copy_from_slice(&color);
        }
    }

    pixels
}

fn interpolate(start: u8, end: u8, t: f64) -> u8 {
    (start as f64 * (1.0 - t) + end as f64 * t) as u8
}
