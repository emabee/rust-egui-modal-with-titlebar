use anyhow::{Result, anyhow};
use eframe::{App, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Ui, ViewportBuilder};
use egui_modal_with_titlebar::ModalWithTitlebar;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error occured: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    run_native(
        "test_modal_with_titlebar",
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([800., 400.])
                //.with_min_inner_size([200., 200.])
                .with_app_id("app_id"),
            ..Default::default()
        },
        Box::new(|_cc| {
            Ok(Box::new(
                // hand instance of App over to eframe::run_native,
                // which will then call its method `update()` in an endless loop
                TestApp::new(),
            ))
        }),
    )
    .map_err(|e| anyhow!("Couldn't start GUI, caused by {e:?}"))
}
struct TestApp {
    show_modal: bool,
}
impl TestApp {
    pub fn new() -> Self {
        Self { show_modal: false }
    }
}
impl App for TestApp {
    #[rustfmt::skip]
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show(ui, |ui| {
            ui.label("Test!");

            if ui.button("Show the Popup!").clicked() {
                self.show_modal = true;
            }
            ui.take_available_space();
        });

        if self.show_modal {
            let response = ModalWithTitlebar::new(
                "with title and close button",
                "Nice Modal with title",
                true,
            ).show(ui.ctx(), |ui| {
                ui.set_width(300.);
                ui.label("bli bla blub");
                ui.button("Put it away!").clicked()
            });

            if response.should_close()  // the user clicked outside the modal
            || response.inner.0         // the user clicked our "Put it away!" button
            || response.inner.1         // the user clicked the close button in the title bar
            {
                self.show_modal = false;
            }
        }
    }
}
