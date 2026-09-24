#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

//! A thin wrapper around `egui::Modal` that adds a title bar and an optional close button.
//!
//! ## Example
//!
//! This is a complete `egui` program that uses [`ModalWithTitlebar`].
//!
//! ```rust
//! use anyhow::{Result, anyhow};
//! use eframe::{App, Frame, NativeOptions, run_native};
//! use egui::{CentralPanel, Ui, ViewportBuilder};
//! use egui_modal_with_titlebar::ModalWithTitlebar;
//! use std::process::ExitCode;
//!
//! fn main() -> ExitCode {
//!     match run() {
//!         Ok(()) => ExitCode::SUCCESS,
//!         Err(e) => {
//!             println!("Error occured: {e:?}");
//!             ExitCode::FAILURE
//!         }
//!     }
//! }
//!
//! fn run() -> Result<()> {
//!     run_native(
//!         "test_modal_with_titlebar",
//!         NativeOptions {
//!             viewport: ViewportBuilder::default()
//!                 .with_inner_size([800., 400.])
//!                 .with_app_id("some app id"),
//!             ..Default::default()
//!         },
//!         Box::new(|_cc| {Ok(Box::new(TestApp::new()))}),
//!     )
//!     .map_err(|e| anyhow!("Couldn't start GUI, caused by {e:?}"))
//! }
//! struct TestApp {
//!     show_modal: bool,
//!     modal_with_title: ModalWithTitlebar,
//! }
//! impl TestApp {
//!     pub fn new() -> Self {
//!         Self {
//!             show_modal: false,
//!             modal_with_title: ModalWithTitlebar::new(
//!                 "Title and close button",
//!                 "Title of the Modal",
//!                 true,
//!             ),
//!         }
//!     }
//! }
//! impl App for TestApp {
//!     #[rustfmt::skip]
//!     fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
//!         CentralPanel::default().show(ui, |ui| {
//!             ui.label("Test!");
//!
//!             if ui.button("Show the Popup!").clicked() {
//!                 self.show_modal = true;
//!             }
//!             ui.take_available_space();
//!         });
//!
//!         if self.show_modal {
//!             let response = self.modal_with_title.show(ui.ctx(), |ui| {
//!                 ui.set_width(300.);
//!                 ui.label("bli bla blub");
//!                 ui.button("Put it away!").clicked()
//!             });
//!
//!             if response.should_close()  // the user clicked outside the modal
//!             || response.inner.0         // the user clicked the "Put it away!" button
//!             || response.inner.1         // the user clicked the close button in the title bar
//!             {
//!                 self.show_modal = false;
//!             }
//!         }
//!     }
//! }
//!
//! ```

mod modal_with_titlebar;
pub use modal_with_titlebar::ModalWithTitlebar;
