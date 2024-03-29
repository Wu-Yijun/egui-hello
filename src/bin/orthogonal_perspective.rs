use eframe::{egui, egui_glow, glow};
use egui::mutex::Mutex;
use glm;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 680.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    rotating_triangle: Arc<Mutex<RotatingTriangle>>,
    angle: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            rotating_triangle: Arc::new(Mutex::new(RotatingTriangle::new(gl))),
            angle: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.label(format!("Drag to rotate(angle:{})!", self.angle));
        });
        egui::CentralPanel::default()
            .frame(egui::containers::Frame {
                fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if ui.button("Menu").clicked() {
                    self.angle = 0.0;
                }
            });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.rotating_triangle.lock().destroy(gl);
        }
    }
}

impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let rotating_triangle = self.rotating_triangle.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                rotating_triangle.lock().paint(painter.gl(), angle);
            })),
        };
        ui.painter().add(callback);
    }
}

struct RotatingTriangle {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl RotatingTriangle {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                include_str!("../../assets/shader.vs"),
                include_str!("../../assets/shader.fs"),
            );
            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);

            Self {
                program,
                vertex_array,
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            // gl.depth_mask(true);
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.depth_func(glow::LEQUAL);
            let identity = glm::mat4(
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            );
            let colors = [
                0.0, 1.0, 0.5, 1.0, /* Left */
                1.0, 0.5, 0.4, 1.0, /* Top */
                1.0, 0.5, 0.0, 1.0, /* Bottom */
                0.5, 0.0, 1.0, 1.0, /* Right*/
            ];
            let points = [
                0.7, 0.0, 0.2, // Left
                0.0, 0.7, 0.2, // Top
                0.0, -0.7, 0.2, // Bottum
                -0.7, 0.0, 0.2, // Right
            ];
            let proj = [
                angle.cos(),
                0.0,
                -angle.sin(),
                0.0,
                1.0,
                0.0,
                angle.sin(),
                0.0,
                angle.cos(),
            ];
            gl.uniform_matrix_3_f32_slice(
                gl.get_uniform_location(self.program, "u_proj").as_ref(),
                false,
                &proj,
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_colors").as_ref(),
                false,
                &colors,
            );
            gl.uniform_matrix_4x3_f32_slice(
                gl.get_uniform_location(self.program, "u_points").as_ref(),
                false,
                &points,
            );
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                true as i32,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_mask_pos").as_ref(),
                0.0,
                0.0,
                0.0,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_mask_dir").as_ref(),
                1.0,
                1.0,
                0.0,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            let mut angle = angle;
            for _ in 0..10 {
                let points = glm::mat4(
                    0.7, 0.0, 0.2, 0.0, // Left
                    0.0, 0.7, 0.2, 0.0, // Top
                    0.0, -0.7, 0.2, 0.0, // Bottum
                    -0.7, 0.0, 0.2, 0.0, // Right
                );
                let rot = glm::ext::rotate(&identity, angle, glm::vec3(0.0, 1.0, 0.0));
                let rot = rot * points;
                let mut pts: Vec<f32> = vec![];
                for v in rot.as_array() {
                    pts.push(v.x);
                    pts.push(v.y);
                    pts.push(v.z);
                }
                gl.uniform_matrix_4x3_f32_slice(
                    gl.get_uniform_location(self.program, "u_points").as_ref(),
                    false,
                    pts.as_slice(),
                );
                // println!("{:#?}", pts.as_slice());
                angle += (36.0f32).to_radians();

                gl.uniform_1_i32(
                    gl.get_uniform_location(self.program, "u_base_layer")
                        .as_ref(),
                    true as i32,
                );
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

                gl.uniform_1_i32(
                    gl.get_uniform_location(self.program, "u_base_layer")
                        .as_ref(),
                    false as i32,
                );
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            }
            
            let points = [
                0.0, 0.0, 0.0, // Left
                1.0, 1.0, 0.05, // Top
                1.0, 1.05, 0.0, // Bottum
                0.0, 0.05, 0.0, // Right
            ];
            gl.uniform_matrix_4x3_f32_slice(
                gl.get_uniform_location(self.program, "u_points").as_ref(),
                false,
                &points,
            );
            let colors = [
                1.0, 0.0, 0.0, 1.0, /* Left */
                1.0, 0.0, 0.0, 1.0, /* Top */
                1.0, 0.0, 0.0, 1.0, /* Bottom */
                1.0, 0.0, 0.0, 1.0, /* Right*/
            ];
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_colors").as_ref(),
                false,
                &colors,
            );
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_base_layer")
                    .as_ref(),
                true as i32,
            );
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            
        }
    }
}
