use crate::widgets::progress_bar::ProgressBar;
use eframe::{
    egui::{
        self, Button, Context, FontData, FontDefinitions, Image, Label, Layout, RichText, Sense, Ui,
    },
    emath::Align,
    epaint::{Color32, FontFamily, Pos2, Rect, Rounding, Vec2},
    App, CreationContext, Frame,
};
use egui_extras::RetainedImage;

pub(crate) struct Vape {
    status: Status,
    logo: RetainedImage,
    top_corner: RetainedImage,
    bottom_corner: RetainedImage,
}

#[derive(Clone)]
enum Status {
    NotFound,
    Loading,
    Loaded,
}

impl Vape {
    pub(crate) fn new(ctx: &eframe::CreationContext<'_>) -> Box<Self> {
        Self::load_fonts(ctx);

        let (top_corner, bottom_corner) = Self::load_background_art();

        Box::new(Self {
            status: Status::Loading,
            logo: Self::load_logo(),
            top_corner,
            bottom_corner,
        })
    }

    fn load_logo() -> RetainedImage {
        RetainedImage::from_image_bytes(
            "vape",
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/vape.png")),
        )
        .expect("could not load vape logo")
    }

    fn load_background_art() -> (RetainedImage, RetainedImage) {
        (
            RetainedImage::from_image_bytes(
                "top-corner",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/top-corner.png"
                )),
            )
            .expect("could not load top-corner"),
            RetainedImage::from_image_bytes(
                "bottom-corner",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/bottom-corner.png"
                )),
            )
            .expect("could not load bottom-corner"),
        )
    }

    fn load_fonts(ctx: &CreationContext<'_>) {
        let proxima_nova = FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/ProximaNova.ttf"
        )));

        let mut fonts = FontDefinitions::default();

        fonts
            .font_data
            .insert("ProximaNova".to_string(), proxima_nova);

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "ProximaNova".to_string());

        ctx.egui_ctx.set_fonts(fonts);
    }

    fn render(&mut self, ctx: &Context, ui: &mut Ui, frame: &mut Frame) {
        window_controls(ui, frame);
        loader(self, ui, ctx, frame);
        // background_art(self, ui, ctx);
    }
}

impl App for Vape {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut style = egui::containers::Frame::default();
        style.fill = Color32::from_rgb(25, 25, 25);
        style.stroke = egui::Stroke::new(3.0, Color32::from_rgb(35, 35, 35));
        style.inner_margin = 10.0.into();
        style.outer_margin = 10.0.into();
        style.rounding = 10.0.into();

        frame.drag_window();

        egui::CentralPanel::default()
            .frame(style)
            .show(ctx, |ui| self.render(ctx, ui, frame));
    }
}

#[derive(Debug)]
struct ParseStatusError;

impl TryFrom<u32> for Status {
    type Error = ParseStatusError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotFound),
            1 => Ok(Self::Loading),
            2 => Ok(Self::Loaded),
            _ => Err(ParseStatusError),
        }
    }
}

impl Into<u32> for Status {
    fn into(self) -> u32 {
        match self {
            Self::NotFound => 0,
            Self::Loading => 1,
            Self::Loaded => 2,
        }
    }
}

fn background_art(app_state: &Vape, ui: &mut Ui, ctx: &Context) {
    ui.put(
        Rect::from_two_pos(Pos2::new(0., 0.), Pos2::new(147., 178.)),
        Image::new(
            app_state.top_corner.texture_id(ctx),
            Pos2::new(147., 178.).to_vec2(),
        ),
    );

    ui.put(
        Rect::from_two_pos(Pos2::new(720., 330.), Pos2::new(820., 480.)),
        Image::new(
            app_state.bottom_corner.texture_id(ctx),
            Vec2::new(100., 150.),
        ),
    );
}

fn swap_between_screens(app_state: &mut Vape) {
    const MAX_STATUS_VARIANTS: u32 = std::mem::variant_count::<Status>() as u32;
    let mut status: u32 = app_state.status.clone().into();
    status += 1;

    app_state.status = Status::try_from(status % MAX_STATUS_VARIANTS).expect("invalid status");
}

fn loader(app_state: &mut Vape, ui: &mut Ui, ctx: &Context, frame: &mut Frame) {
    let logo = Image::new(app_state.logo.texture_id(ctx), app_state.logo.size_vec2());

    ui.vertical_centered(|ui| {
        ui.add_space(137.);

        let response = ui.add(logo);

        if response.interact(Sense::click()).clicked() {
            swap_between_screens(app_state);
        };

        ui.add_space(34.);

        match app_state.status {
            Status::NotFound => not_found(ui),
            Status::Loading => loading(ui),
            Status::Loaded => loaded(ui, frame),
        }
    });
}

fn not_found(ui: &mut Ui) {
    ui.add(Label::new(RichText::new("No Minecraft found").size(14.0)));
    ui.add(Label::new(
        RichText::new("Open Minecraft to continue").size(14.0),
    ));
}

fn loading(ui: &mut Ui) {
    let progress_bar = ProgressBar::new(0.45)
        .desired_width(400.0)
        .desired_height(8.0)
        .fill(Color32::from_rgb(0, 90, 72));

    let secondary_bar = ProgressBar::new(0.75)
        .desired_width(400.0)
        .desired_height(8.0)
        .fill(Color32::from_rgb(0, 90, 72));

    ui.add(progress_bar);
    ui.add_space(16.);
    ui.add(secondary_bar);
}

fn loaded(ui: &mut Ui, frame: &mut Frame) {
    let close =
        Button::new(RichText::new("Close Window").size(20.)).fill(Color32::from_rgb(0, 90, 72));

    ui.add(Label::new(
        RichText::new("Vape has finished loading").size(14.0),
    ));

    ui.add(Label::new(
        RichText::new("Press RIGHT SHIFT while in game to open the GUI").size(14.0),
    ));

    ui.add_space(16.);

    if ui.add(close).clicked() {
        frame.close();
    };
}

fn window_controls(ui: &mut Ui, frame: &mut Frame) {
    let exit = Button::new(RichText::new("❌"))
        .fill(Color32::from_rgb(30, 30, 30))
        .rounding(Rounding {
            ne: 10.0,
            nw: 0.0,
            sw: 0.0,
            se: 10.0,
        });

    let minimize = Button::new(RichText::new("➖"))
        .fill(Color32::from_rgb(30, 30, 30))
        .rounding(Rounding {
            ne: 0.0,
            nw: 10.0,
            sw: 10.0,
            se: 0.0,
        });

    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        if ui.add(exit).clicked() {
            frame.close();
        }

        ui.add_space(-5.0);

        if ui.add(minimize).clicked() {
            frame.set_minimized(true);
        }
    });
}
