use eframe::egui;
use std::process;
mod tools_func;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    eframe::run_native(
        "UI OverLap",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    show_immediate_viewport: bool,
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        tools_func::load_fonts(&cc.egui_ctx);
        Self {
            show_immediate_viewport: false,
        }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let wind = egui::Window::new("Test window")
            .resizable(false)
            .default_rect(egui::Rect {
                min: egui::Pos2 { x: 300.0, y: 200.0 },
                max: egui::Pos2 { x: 500.0, y: 300.0 },
            });
        wind.show(ctx, |ui| {
            let _ = ui.button("ClickMe1");
            ui.group(|ui| {
                let _ = ui.button("ClickMe2");
                ui.checkbox(
                    &mut self.show_immediate_viewport,
                    "Show immediate child viewport",
                );
            });
        });

        let layout_layer0 = egui::containers::Frame {
            fill: egui::Color32::from_rgb(241, 233, 218),
            ..Default::default()
        };
        let layout_layer1 = egui::containers::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(layout_layer0)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    for i in 0..10 {
                        ui.push_id(i, |ui| {
                            ui.label(
                                egui::RichText::new("Back Ground Layer")
                                    .color(egui::Color32::from_rgba_unmultiplied(0, 255, 0, 128)),
                            )
                        });
                    }
                });
            });
        egui::CentralPanel::default()
            .frame(layout_layer1)
            .show(ctx, |ui| {
                if ui.input(|k| k.key_pressed(egui::Key::Escape)) {
                    process::exit(0);
                }
                // egui::Frame::default().fill(egui::Color32::from_rgba_unmultiplied(r, g, b, a)).show( add_contents);
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(255, 0, 0, 50))
                    .show(ui, |ui| {
                        ui.label("Frames cannot overlap");
                    });

                ui.put(
                    egui::Rect::from_min_size(
                        egui::Pos2 { x: 0.00, y: 30.0 },
                        egui::Vec2 { x: 50.0, y: 30.0 },
                    ),
                    egui::Label::new("Label fix 放在固定位置的 Label"),
                );
            });
        if self.show_immediate_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Immediate Viewport")
                    .with_inner_size([200.0, 100.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default()
                        .frame(egui::containers::Frame {
                            fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                            ..Default::default()
                        })
                        .show(ctx, |ui| {
                            ui.label("Hello from immediate viewport");
                        });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_immediate_viewport = false;
                    }
                },
            );
        }
    }
}
