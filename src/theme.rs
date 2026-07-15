// really bad attempt at kinda replicating gtk looks
// gtk adwaita dark look, gray surfaces and flat gray buttons
use eframe::egui;

// the accent blue gtk uses for suggested actions
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(53, 132, 228);
pub const DIM_TEXT: egui::Color32 = egui::Color32::from_rgb(154, 153, 150);
pub const ERROR_TEXT: egui::Color32 = egui::Color32::from_rgb(255, 123, 99);

const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(36, 36, 36);
const VIEW: egui::Color32 = egui::Color32::from_rgb(30, 30, 30);
const BUTTON: egui::Color32 = egui::Color32::from_rgb(58, 58, 58);
const BUTTON_HOVER: egui::Color32 = egui::Color32::from_rgb(68, 68, 68);
const BUTTON_PRESSED: egui::Color32 = egui::Color32::from_rgb(48, 48, 48);
const TEXT: egui::Color32 = egui::Color32::from_rgb(238, 238, 236);
const CORNER: egui::CornerRadius = egui::CornerRadius::same(8);

pub fn semibold() -> egui::FontFamily {
    egui::FontFamily::Name("semibold".into())
}

pub fn apply(ctx: &egui::Context) {
    load_fonts(ctx);

    // force our own look on both system themes
    ctx.set_theme(egui::ThemePreference::Dark);
    let mut style = egui::Style::default();
    set_spacing(&mut style);
    set_text_styles(&mut style);
    style.visuals = build_visuals();
    ctx.set_style_of(egui::Theme::Dark, style);
}

// bundle inter so the app looks the same everywhere
fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "inter".to_string(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "inter-semibold".to_string(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-SemiBold.ttf"
        ))),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "inter".to_string());
    fonts.families.insert(
        semibold(),
        vec!["inter-semibold".to_string(), "inter".to_string()],
    );
    ctx.set_fonts(fonts);
}

fn set_spacing(style: &mut egui::Style) {
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
}

fn set_text_styles(style: &mut egui::Style) {
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(21.0, semibold()),
    );
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(14.5));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(14.5));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(12.0));
}

// every widget state looks the same, only the colors change
fn paint_state(
    state: &mut egui::style::WidgetVisuals,
    fill: egui::Color32,
    text_color: egui::Color32,
) {
    state.weak_bg_fill = fill;
    state.bg_fill = fill;
    state.bg_stroke = egui::Stroke::NONE;
    state.fg_stroke = egui::Stroke::new(1.0, text_color);
    state.corner_radius = CORNER;
}

fn build_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = BACKGROUND;
    visuals.extreme_bg_color = VIEW;
    visuals.selection.bg_fill = ACCENT;

    // labels and separators, keep the default bg stroke or separators vanish
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT);
    visuals.widgets.noninteractive.corner_radius = CORNER;

    paint_state(&mut visuals.widgets.inactive, BUTTON, TEXT);
    paint_state(
        &mut visuals.widgets.hovered,
        BUTTON_HOVER,
        egui::Color32::WHITE,
    );
    paint_state(
        &mut visuals.widgets.active,
        BUTTON_PRESSED,
        egui::Color32::WHITE,
    );
    paint_state(&mut visuals.widgets.open, BUTTON_PRESSED, TEXT);
    visuals
}
