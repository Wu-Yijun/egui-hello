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
    strs: Vec<String>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        Self {
            frames: 0,
            strs: vec![String::from("Aluria")],
        }
    }
}

// impl eframe::App for MyEguiApp {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("你好 Hello World!");
//             ui.horizontal(|ui| {
//                 for i in 0..10 {
//                     if ui.button(format!("Button {i}")).clicked() {
//                         ui.heading(format!("Button {i} clicked"));
//                         if i == 0 {
//                             ctx.set_visuals(egui::Visuals::dark());
//                         } else if i == 1 {
//                             ctx.set_visuals(egui::Visuals::light());
//                         }
//                     }
//                 }
//             });
//             ui.label(self.frames.to_string());
//             if ui.input(|k| k.key_pressed(egui::Key::Escape) || k.key_pressed(egui::Key::Backspace))
//             {
//                 std::process::exit(0);
//             }
//             self.frames += 1;
//         });
//     }
// }

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好中文");
            ui.label(self.frames.to_string());
            for str in &mut self.strs {
                ui.horizontal(|ui| {
                    if *str != "".to_string() {
                        ui.add(egui::TextEdit::singleline(str).hint_text("张三"));
                        ui.label("你好 ".to_string() + str);
                    } else {
                        ui.add(egui::TextEdit::singleline(str).hint_text("张三"))
                            .on_hover_text("请输入姓名");
                        ui.label("等待输入");
                    }
                });
            }
            if ui.input_mut(|k| k.consume_key(egui::Modifiers::CTRL, egui::Key::D)) {
                self.strs.pop();
            }
            if ui.input_mut(|k| k.consume_key(egui::Modifiers::CTRL, egui::Key::N)) {
                self.strs.push(String::from(""));
            }
        });
        self.frames += 1;
    }
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "font_key".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/AlibabaPuHuiTi-2-55-Regular.otf")),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "font_key".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("font_key".to_owned());
    ctx.set_fonts(fonts);
}
