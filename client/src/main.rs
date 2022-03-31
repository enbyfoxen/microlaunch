mod gui;
mod auth;

fn main() {
    let app = gui::MicrolaunchApp::default();
    let native_options = eframe::NativeOptions {
        decorated: true,
        transparent: false,
        min_window_size: Some(egui::vec2(320.0, 10.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}