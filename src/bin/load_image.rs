use eframe::egui;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    eframe::run_native(
        "Load Image",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    imgn: i32,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self { imgn: 0 }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Next Image").clicked() {
                self.imgn = if self.imgn > 6 { 0 } else { self.imgn + 1 };
            }
            ui.image(format!("file://assets/ferris{}.png", self.imgn));
        });
    }
}
