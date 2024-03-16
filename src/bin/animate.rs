use eframe::egui::{self, ecolor::linear_u8_from_linear_f32, pos2, Color32, Id, Rect, Rounding};
use rand::prelude::random;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
    .unwrap();
}

struct MyApp {
    id_r: Id,
    id_g: Id,
    id_b: Id,
    r: f32,
    g: f32,
    b: f32,
    rt: f32,
    gt: f32,
    bt: f32,
    frames: i32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let s = Self {
            id_r: Id::new("anim-color-r"),
            id_g: Id::new("anim-color-g"),
            id_b: Id::new("anim-color-b"),
            r: random(),
            g: random(),
            b: random(),
            rt: 1.0 + 3.0 * random::<f32>(),
            gt: 1.0 + 3.0 * random::<f32>(),
            bt: 1.0 + 3.0 * random::<f32>(),
            frames: 0,
        };
        cc.egui_ctx.animate_value_with_time(s.id_r, s.r, s.rt);
        cc.egui_ctx.animate_value_with_time(s.id_g, s.g, s.gt);
        cc.egui_ctx.animate_value_with_time(s.id_b, s.b, s.bt);
        s
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let r = (ctx.animate_value_with_time(self.id_r, self.r, self.rt) * 255.0f32) as u8;
            let g = (ctx.animate_value_with_time(self.id_g, self.g, self.gt) * 255.0f32) as u8;
            let b = (ctx.animate_value_with_time(self.id_b, self.b, self.bt) * 255.0f32) as u8;
            ui.painter().rect_filled(
                Rect::from_min_max(pos2(0.0, 0.0), pos2(300.0, 300.0)),
                Rounding::default(),
                Color32::from_rgb(r, g, b),
            );

            if ui.button("Animate").clicked() {
                // Switch animation direction.
                self.r = random();
                self.g = random();
                self.b = random();

                ctx.request_repaint();
            }
            ui.label(format!("frames{}", self.frames));
            self.frames += 1;
        });
    }
}
