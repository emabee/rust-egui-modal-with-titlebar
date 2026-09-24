use egui::{
    Align2, Area, Color32, Context, CornerRadius, Frame, Id, Margin, Modal, ModalResponse, Rect,
    Response, Sense, Ui, Vec2, WidgetInfo, WidgetType, emath::GuiRounding,
};
/// A thin wrapper around `egui::Modal` that adds a title bar and an optional close button.
///
/// ```rust,nocompile
///  // ...
/// struct TestApp {
///     show_modal: bool,
///     modal_with_title: ModalWithTitlebar,
/// }
///  // ...
/// impl TestApp {
///     pub fn new() -> Self {
///         Self {
///             show_modal: false,
///             modal_with_title: ModalWithTitlebar::new(
///                 "Title and close button",
///                 "Title of the Modal",
///                 true,
///             ),
///         }
///     }
/// }
///  // ...
/// impl App for TestApp {
///     #[rustfmt::skip]
///     fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
///     // ...
///         if self.show_modal {
///             // Draw the popup
///             let response = self.modal_with_title.show(ui.ctx(), |ui| {
///             // ... draw the content, including some button
///
///                 if response.should_close()  // the user clicked outside the modal
///                 || response.inner.0         // the user clicked our own button
///                 || response.inner.1         // the user clicked the close button in the title bar
///                 {
///                     self.show_modal = false;
///                 }
///             })
///         }
///     }
/// }
///
/// ```
///
/// See the crate documentation for the complete example.
pub struct ModalWithTitlebar {
    modal: Modal,
    o_title: Option<String>,
    show_close_button: bool,
}

impl ModalWithTitlebar {
    /// Creates a new Modal instance with titlebar and optionally a close button.
    #[must_use]
    pub fn new<I: Into<Id>, T: Into<String>>(id: I, title: T, with_close_button: bool) -> Self {
        Self {
            modal: Modal::new(id.into()),
            o_title: Some(title.into()),
            show_close_button: with_close_button,
        }
    }

    /// Like `ModalWithTitlebar::new`, but allows omitting the titlebar completely by setting
    /// `o_title` to `None`.
    #[must_use]
    pub fn new_with_optional_title<I: Into<Id>, T: Into<String>>(
        id: I,
        o_title: Option<T>,
        with_close_button: bool,
    ) -> Self {
        Self {
            modal: Modal::new(id.into()),
            o_title: o_title.map(Into::into),
            show_close_button: with_close_button,
        }
    }

    /// Set the frame of the modal.
    ///
    /// Default is `Frame::popup`.
    #[must_use]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.modal = self.modal.frame(frame);
        self
    }

    /// Set the backdrop color of the modal.
    ///
    /// Default is `Color32::from_black_alpha(100)`.
    #[must_use]
    pub fn backdrop_color(mut self, color: Color32) -> Self {
        self.modal = self.modal.backdrop_color(color);
        self
    }

    /// Set the area of the modal.
    ///
    /// Default is `Modal::default_area`.
    #[must_use]
    pub fn area(mut self, area: Area) -> Self {
        self.modal = self.modal.area(area);
        self
    }

    /// Show the modal.
    ///
    /// Note that if `add_contents` returns some type `T`, `show` returns `(T,bool)`, where the
    /// additional bool is true when the close button was visible and pressed.
    pub fn show<T>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> T,
    ) -> ModalResponse<(T, bool)> {
        let (modal, o_title, show_close_button) =
            (self.modal, self.o_title, self.show_close_button);

        modal.show(ctx, |ui| {
            let close_button_clicked =
                o_title.is_some() && titlebar(o_title.as_deref(), show_close_button, ui);
            (
                Frame::new().show(ui, add_contents).inner,
                close_button_clicked,
            )
        })
    }
}

fn titlebar(o_title: Option<&str>, show_close_button: bool, ui: &mut Ui) -> bool {
    // To pretend being a window title, we ignore the window margins top, left and right:
    let window_margins = ui.style().spacing.window_margin;
    let margins = Margin {
        left: -window_margins.left,
        right: -window_margins.right,
        top: -window_margins.top,
        bottom: 0,
    };
    let window_corners = ui.style().visuals.window_corner_radius;
    let corner_radius = CornerRadius {
        nw: window_corners.nw,
        ne: window_corners.ne,
        sw: 0,
        se: 0,
    };
    let frame_response = Frame::default()
        .outer_margin(margins)
        .corner_radius(corner_radius)
        .inner_margin(ui.style().spacing.window_margin)
        .fill(ui.style().visuals.widgets.open.weak_bg_fill)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(o_title.unwrap());
            });
        })
        .response;

    show_close_button && close_button(ui, window_margins, &frame_response).clicked()
}

// The close button is taken from egui/src/containers/window.rs and modified to work here.
fn close_button(ui: &mut Ui, window_margins: Margin, frame_response: &Response) -> Response {
    let actual_frame_rect = Rect {
        min: frame_response.rect.min
            - Vec2::new(
                f32::from(window_margins.left),
                f32::from(window_margins.top),
            ),
        max: frame_response.rect.max + Vec2::new(f32::from(window_margins.right), 0.0),
    };
    let button_center = Align2::RIGHT_CENTER
        .align_size_within_rect(Vec2::splat(frame_response.rect.height()), actual_frame_rect)
        .center();
    let button_size = Vec2::splat(ui.spacing().icon_width);
    let button_rect = Rect::from_center_size(button_center, button_size);
    let button_rect = button_rect.round_to_pixels(ui.pixels_per_point());

    let close_button_response = ui.interact(
        button_rect,
        ui.auto_id_with("mwt_close_button"),
        Sense::click(),
    );
    close_button_response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), "Close"));

    ui.expand_to_include_rect(close_button_response.rect);
    let visuals = ui.style().interact(&close_button_response);
    let final_rect = button_rect.shrink(2.0).expand(visuals.expansion);
    let stroke = visuals.fg_stroke;
    ui.painter() // paints \
        .line_segment([final_rect.left_top(), final_rect.right_bottom()], stroke);
    ui.painter() // paints /
        .line_segment([final_rect.right_top(), final_rect.left_bottom()], stroke);
    close_button_response
}
