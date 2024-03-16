use eframe::egui;
mod tools_func;

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
        tools_func::load_fonts(&cc.egui_ctx);
        Self { imgn: 0 }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter()
                .rect_filled(ui.clip_rect(), 5.0, egui::Color32::GREEN);
            if ui.button("Next Image").clicked() {
                self.imgn = if self.imgn > 6 { 0 } else { self.imgn + 1 };
            }
            let img = egui::Image::new(format!("file://assets/ferris{}.png", self.imgn))
                .max_height(100.0);
            let btn3 = egui::Button::image(img)
                .fill(egui::Color32::TRANSPARENT)
                .frame(false);
            let btn3 = ui.add(btn3);
            if btn3.clicked() {
                self.imgn = if self.imgn < 1 { 6 } else { self.imgn - 1 };
            }
            btn3.on_hover_text_at_pointer("Previous Image");

            let img = ui.add(
                egui::Image::new(format!("file://assets/ferris{}.png", self.imgn)).max_height(50.0),
            );
            ui.painter().text(
                img.rect.center(),
                egui::Align2::CENTER_CENTER,
                "点我",
                egui::FontId {
                    size: 20.0,
                    family: egui::FontFamily::Proportional,
                },
                egui::Color32::BLUE,
            );
            if img.interact(egui::Sense::click()).clicked() {
                println!("Clicked");
            }
            ui.label("两种实现方式");
        });
    }
}
