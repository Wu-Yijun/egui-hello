use eframe::egui;
mod tools_func;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    eframe::run_native(
        "Animation",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    frames: i32,
    frames2: i32,
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        tools_func::load_fonts(&cc.egui_ctx);
        Self {
            frames: 0,
            frames2: 0,
        }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let color = if ui.visuals().dark_mode {
                egui::Color32::from_additive_luminance(196)
            } else {
                egui::Color32::from_black_alpha(240)
            };

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.ctx().request_repaint();
                let time = ui.input(|i| i.time);

                let desired_size = ui.available_width() * egui::vec2(1.0, 0.35);
                let (_id, rect) = ui.allocate_space(desired_size);

                let to_screen = egui::emath::RectTransform::from_to(
                    egui::Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                    rect,
                );

                let mut shapes = vec![];

                for &mode in &[2, 3, 5] {
                    let mode = mode as f64;
                    let n = 120;
                    let speed = 1.5;

                    let points: Vec<egui::Pos2> = (0..=n)
                        .map(|i| {
                            let t = i as f64 / (n as f64);
                            let amp = (time * speed * mode).sin() / mode;
                            let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                            to_screen * egui::pos2(t as f32, y as f32)
                        })
                        .collect();

                    let thickness = 10.0 / mode as f32;
                    shapes.push(egui::epaint::Shape::line(
                        points,
                        egui::Stroke::new(thickness, color),
                    ));
                }
                ui.painter().extend(shapes);
                ui.label(format!("frames{}", self.frames));
                self.frames += 1;
            });
            ui.label(format!("frames2{}", self.frames2));
            self.frames2 += 1;
        });
    }
}
