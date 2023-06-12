use eframe::App;
use egui::TextureHandle;
use egui_extras::image::load_svg_bytes;
use qrcode::QrCode;
use qrcode::render::svg;

pub struct QrcodeApp {
    input: String,
    show: Option<TextureHandle>
}

impl Default for QrcodeApp {
    fn default() -> Self {
        Self { input: "".to_owned(), show: Option::None }
    }
}

impl QrcodeApp {
    pub fn new() -> Self {
        Default::default()
    }
}

impl App for QrcodeApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self {input,show} = self;
        egui::CentralPanel::default().show(ctx, |ui|{
        //    ui.input(|i| i.key_pressed(desired_key))
            ui.horizontal(|ui|{
                ui.label("Input: ".to_string());
                if ui.text_edit_singleline(input).changed() {
                    let code = QrCode::new(input.as_bytes()).unwrap();
                    let img = code.render()
                    .min_dimensions(300, 300)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build();
                    // println!("{}",img);
                    let svg = load_svg_bytes(img.as_bytes());
                    if let Ok(svg) = svg {
                        *show = Some(ctx.load_texture("code", svg, Default::default()).clone());
                    } else {
                        *show = None
                    }
                }
            });
            // ui.label(show.to_owned());
            if let Some(show) = show {
                ui.image(show.id(), show.size_vec2());
            }
        });
    }
}