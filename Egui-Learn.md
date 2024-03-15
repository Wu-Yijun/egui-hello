![@important](参考页面 https://www.cnblogs.com/absalom/articles/17943481)

## 先从简单的 Hello, world. 开始

我假定你有一些 fltk-rs 或 gtk-rs 基础，不然一点点解释太累了。

```Rust
use eframe::egui;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    ).unwrap();
}
#[derive(Default)]
struct MyEguiApp {}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好世界 Hello World!");
        });
    }
}
```

重点在于 `run_native(app_name, native_options, app_creator)` 这个函数，其中 `NativeOptions::default()` 表示我们运行的是桌面原生应用，egui/eframe同样支持WASM应用开发。
后面的 update 就是render时的更新，每帧都会有一次。我们在其中创建了一个 CentralPanel 也就是在正中显示的面板（侧面的有 SidePanel::left/right 和 TopBottomPanel::top/bottom），然后设置了标题级别文字。最终显示如下
![@image](images/image.png)

Unicode 不能显示，英文正常。

## 加入一些新元素

将结构体改为
```Rust
struct MyEguiApp {
    frames: u64,
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { frames: 0 }
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
            self.frames += 1;
            if ui.input(|k| k.key_pressed(egui::Key::Escape) || k.key_pressed(egui::Key::Backspace)) {
                std::process::exit(0);
            }
        });
    }
}
```

frames 记录渲染更新的帧数，我们发现只有当鼠标移动时，才会重新渲染，否则它连变都不变。
然后当前两个按钮按下之后，在深色和浅色模式间切换，但是新显示的文字只会显示一帧，这就是即时模式，每一次渲染都是重新计算的。
然后 ui.horizontal 将布局变为水平。
最下方的 input 方法检测键盘按键。

![@image](images/image2.png)

## 输入输出与中文

输入输出可以使用 `TextEdit::singleline(str)` 来实现，其中的 str 会先打印出来，然后根据键盘输入反馈修改 str 的内容。
具体如下：
```Rust

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
```
显示中文需要加载字体，这里使用了 fonts.font_data.insert 来新增一种字体，然后 include_bytes! 宏将文件内容在编译时转为二进制数据，直接输入给函数。
下面两行分别将我们的字体加载到了 Proportional 开头和 Monospace 的末尾。
```Rust
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
```

![@image](images/image3.png)

## 动态加载图像

首先我们要在 Cargo.toml 中引入依赖 `egui_extras` ，然后在结构体初始化时加载图像加载器 `egui_extras::install_image_loaders(&cc.egui_ctx)` ，最后在 UI 中使用 `ui.image(path/url)` 来加载图像。

代码示例如下：

```Rust
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
```

最终样貌如下：

![@image](images/image5.png)

## 更多内容

egui的各种特性都可以在! [这个网页上](https://www.egui.rs/#Demo)找到，而且附带源代码可供参考。因此，我们不需要更多的教程了。下面先尝试几个简单的特性，就可以直接开始游戏开发了。

![@image](images/image4.png)

# 特性测试
! [@important](同时也是游戏渲染器开发积累)

## UI overlap

```Rust
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
                    egui::CentralPanel::default()
                        .frame(egui::containers::Frame {
                            fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                            ..Default::default()
                        })
                        .show(ctx, |ui| {
                            ui.label("Hello from immediate viewport");
                        });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_immediate_viewport = false;
                    }
                },
            );
        }
    }
}
```
效果如下：
![@image](images/image6.png)

首先，各大框架都不能指定固定位置，只有实现 Widget 的小部件才能通过 `ui.put(Rect, Widget)` 来指定位置。
**如果想要多层，需要多个 `egui::CentralPanel::default()` 同时出现，并将上层的设置为半透明，来满足上下叠加的层次。**
然后，`egui::Window::new("Test window")` 只会建立一个新的视窗，窗口的位置一定被置于最上面，因此也不能用来实现浮动显示。
最后，`ctx.show_viewport_immediate(id, builder, viewport_cb)` 是创建的一个新的窗口，和原本的窗口关联性不强，而且我们可以看出，设置透明背景会导致它成黑色的。

## 半透明
UI 的透明度选项于 2 月 16 号更新到了 git 储存库中，但是最新发布版本 0.26.2 发布于 2 月 14 号。 所以... 你懂的，NMD 没有这个选项，官网上的示例倒是挺齐全的，就是我用不了。但不急，反正也用不到。

## Animation

明天再搞，我还要写作业。

## 渐变填充
> 希望可以不要用到 GL ，不然后面的部分都是 Opengl 大法了。

## 纹理填充

## 正交投影伪三维

## 三维前后关系

## Clip

## Infinifold
