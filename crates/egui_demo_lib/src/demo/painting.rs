use std::sync::{Arc, RwLock};

use egui::*;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PaintingData {
    /// in 0-1 normalized coordinates
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Clone)]
pub struct Painting {
    data: Arc<RwLock<PaintingData>>,
}

impl Default for PaintingData {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut data = self.data.write().unwrap();
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut data.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                data.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let mut data = self.data.write().unwrap();

        if data.lines.is_empty() {
            data.lines.push(vec![]);
        }

        let current_line = data.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            data.lines.push(vec![]);
            response.mark_changed();
        }

        let shapes = data
            .lines
            .iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, data.stroke)
            });

        painter.extend(shapes);

        response
    }
}

impl super::Demo for Painting {
    fn name(&self) -> &'static str {
        "🖊 Painting"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        let clone = self.clone();
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(512.0, 512.0))
            .vscroll(false)
            .show(ctx, move |ui, _, _| clone.clone().ui(ui));
    }
}

impl super::View for Painting {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add(crate::egui_github_link_file!());
        });
        self.ui_control(ui);
        ui.label("Paint with your mouse/touch!");
        Frame::canvas(ui.style()).show(ui, |ui| {
            self.ui_content(ui);
        });
    }
}
