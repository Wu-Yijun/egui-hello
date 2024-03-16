use eframe::egui;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    eframe::run_native(
        "一个小例子",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    frames: u64,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            frames: 0,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好 Hello World!");
            ui.horizontal(|ui| {
                for i in 0..10 {
                    if ui.button(format!("Button {i}")).clicked() {
                        ui.heading(format!("Button {i} clicked"));
                        if i == 0 {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else if i == 1 {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    }
                }
            });
            ui.label(self.frames.to_string());
            if ui.input(|k| k.key_pressed(egui::Key::Escape) || k.key_pressed(egui::Key::Backspace))
            {
                std::process::exit(0);
            }
            self.frames += 1;
        });
    }
}
