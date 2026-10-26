//! Software updates are disabled in this build: it never contacts the network. Help ▸ Check for
//! Updates only shows a notice.

use crate::WordApp;

#[derive(Default)]
pub(crate) struct Runtime {
    pub(crate) manual_window_open: bool,
    pub(crate) status_message: Option<String>,
}

pub(crate) fn check(app: &mut WordApp) {
    app.updates_rt.manual_window_open = true;
    app.updates_rt.status_message = Some(tl!("Automatic updates are disabled in this build.").to_string());
}

pub(crate) fn tick(_app: &mut WordApp, _ctx: &egui::Context) {}

pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    if !app.updates_rt.manual_window_open {
        return;
    }
    let mut open = true;
    let mut close = false;
    egui::Window::new(tl!("Check for Updates"))
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            if let Some(msg) = &app.updates_rt.status_message {
                ui.label(msg);
            }
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(tl!("Close")).clicked() {
                    close = true;
                }
            });
        });
    if !open || close {
        app.updates_rt.manual_window_open = false;
        app.updates_rt.status_message = None;
    }
}
