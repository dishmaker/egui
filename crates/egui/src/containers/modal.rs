use crate::{
    Area, Color32, Context, Frame, Id, InnerResponse, Order, Response, Sense, Ui, UiBuilder,
};
use emath::{Align2, Vec2};

/// A modal dialog.
/// Similar to a [`crate::Window`] but centered and with a backdrop that
/// blocks input to the rest of the UI.
///
/// You can show multiple modals on top of each other. The top most modal will always be
/// the most recently shown one.
pub struct Modal {
    pub area: Area,
    pub backdrop_color: Color32,
    pub frame: Option<Frame>,
}

impl Modal {
    /// Create a new Modal. The id is passed to the area.
    pub fn new(id: Id) -> Self {
        Self {
            area: Self::default_area(id),
            backdrop_color: Color32::from_black_alpha(100),
            frame: None,
        }
    }

    /// Returns an area customized for a modal.
    /// Makes these changes to the default area:
    /// - sense: hover
    /// - anchor: center
    /// - order: foreground
    pub fn default_area(id: Id) -> Area {
        Area::new(id)
            .sense(Sense::hover())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .order(Order::Foreground)
    }

    /// Set the frame of the modal.
    ///
    /// Default is [`Frame::popup`].
    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Set the backdrop color of the modal.
    ///
    /// Default is `Color32::from_black_alpha(100)`.
    #[inline]
    pub fn backdrop_color(mut self, color: Color32) -> Self {
        self.backdrop_color = color;
        self
    }

    /// Set the area of the modal.
    ///
    /// Default is [`Modal::default_area`].
    #[inline]
    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    /// Show the modal.
    pub fn show<T>(self, ctx: &Context, content: impl FnOnce(&mut Ui) -> T) -> ModalResponse<T> {
        let (is_top_modal, any_popup_open) = ctx.memory_mut(|mem| {
            mem.set_modal_layer(self.area.layer());
            (
                mem.top_modal_layer() == Some(self.area.layer()),
                mem.any_popup_open(),
            )
        });
        let InnerResponse {
            inner: (inner, backdrop_response),
            response,
        } = self.area.show(ctx, |ui| {
            let mut backdrop = ui.new_child(UiBuilder::new().max_rect(ui.ctx().screen_rect()));
            let backdrop_response = backdrop_ui(&mut backdrop, self.backdrop_color);

            let frame = self.frame.unwrap_or_else(|| Frame::popup(ui.style()));

            // We need the extra scope with the sense since frame can't have a sense and since we
            // need to prevent the clicks from passing through to the backdrop.
            let inner = ui
                .scope_builder(
                    UiBuilder::new().sense(Sense {
                        click: true,
                        drag: true,
                        focusable: false,
                    }),
                    |ui| frame.show(ui, content).inner,
                )
                .inner;

            (inner, backdrop_response)
        });

        ModalResponse {
            response,
            backdrop_response,
            inner,
            is_top_modal,
            any_popup_open,
        }
    }
}

fn backdrop_ui(ui: &mut Ui, color: Color32) -> Response {
    // Ensure we capture any click and drag events
    let response = ui.allocate_response(
        ui.available_size(),
        Sense {
            click: true,
            drag: true,
            focusable: false,
        },
    );

    ui.painter().rect_filled(response.rect, 0.0, color);

    response
}

/// The response of a modal dialog.
pub struct ModalResponse<T> {
    /// The response of the modal contents
    pub response: Response,

    /// The response of the modal backdrop
    pub backdrop_response: Response,

    /// The inner response from the content closure
    pub inner: T,

    /// Is this the top most modal?
    pub is_top_modal: bool,

    /// Is there any popup open?
    /// We need to check this before the modal contents are shown, so we can know if any popup
    /// was open when checking if the escape key was clicked.
    pub any_popup_open: bool,
}

impl<T> ModalResponse<T> {
    /// Should the modal be closed?
    /// Returns true if:
    ///  - the backdrop was clicked
    ///  - this is the top most modal, no popup is open and the escape key was pressed
    pub fn should_close(&self) -> bool {
        let ctx = &self.response.ctx;
        let escape_clicked = ctx.input(|i| i.key_pressed(crate::Key::Escape));
        self.backdrop_response.clicked()
            || (self.is_top_modal && !self.any_popup_open && escape_clicked)
    }
}
