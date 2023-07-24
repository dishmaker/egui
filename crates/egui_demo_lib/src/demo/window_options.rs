use std::sync::{Arc, RwLock};

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WindowOptionsData {
    title: String,
    title_bar: bool,
    closable: bool,
    collapsible: bool,
    resizable: bool,
    scroll2: [bool; 2],
    disabled_time: f64,

    anchored: bool,
    anchor: egui::Align2,
    anchor_offset: egui::Vec2,
}
#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WindowOptions {
    data: Arc<RwLock<WindowOptionsData>>,
}

impl PartialEq for WindowOptions {
    fn eq(&self, other: &Self) -> bool {
        *self.data.read().unwrap() == *other.data.read().unwrap()
    }
}

impl Default for WindowOptionsData {
    fn default() -> Self {
        Self {
            title: "🗖 Window Options".to_owned(),
            title_bar: true,
            closable: true,
            collapsible: true,
            resizable: true,
            scroll2: [true; 2],
            disabled_time: f64::NEG_INFINITY,
            anchored: false,
            anchor: egui::Align2::RIGHT_TOP,
            anchor_offset: egui::Vec2::ZERO,
        }
    }
}

impl super::Demo for WindowOptions {
    fn name(&self) -> &'static str {
        "🗖 Window Options"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let WindowOptionsData {
            title,
            title_bar,
            closable,
            collapsible,
            resizable,
            scroll2,
            disabled_time,
            anchored,
            anchor,
            anchor_offset,
        } = self.data.read().unwrap().clone();

        let enabled = ctx.input(|i| i.time) - disabled_time > 2.0;
        if !enabled {
            ctx.request_repaint();
        }

        use super::View as _;
        let mut window = egui::Window::new(title)
            .id(egui::Id::new("demo_window_options")) // required since we change the title
            .resizable(resizable)
            .collapsible(collapsible)
            .title_bar(title_bar)
            .scroll2(scroll2)
            .enabled(enabled);
        if closable {
            window = window.open(open);
        }
        if anchored {
            window = window.anchor(anchor, anchor_offset);
        }
        let clone = self.clone();
        window.show(ctx, move |ui, _, _| {
            let mut clone = clone.clone();
            clone.ui(ui)
        });
    }
}

impl super::View for WindowOptions {
    fn ui(&mut self, ui: &mut egui::Ui) {
        {
            let WindowOptionsData {
                title,
                title_bar,
                closable,
                collapsible,
                resizable,
                scroll2,
                disabled_time: _,
                anchored,
                anchor,
                anchor_offset,
            } = &mut *self.data.write().unwrap();
            ui.horizontal(|ui| {
                ui.label("title:");
                ui.text_edit_singleline(title);
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.checkbox(title_bar, "title_bar");
                        ui.checkbox(closable, "closable");
                        ui.checkbox(collapsible, "collapsible");
                        ui.checkbox(resizable, "resizable");
                        ui.checkbox(&mut scroll2[0], "hscroll");
                        ui.checkbox(&mut scroll2[1], "vscroll");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.checkbox(anchored, "anchored");
                        ui.set_enabled(*anchored);
                        ui.horizontal(|ui| {
                            ui.label("x:");
                            ui.selectable_value(&mut anchor[0], egui::Align::LEFT, "Left");
                            ui.selectable_value(&mut anchor[0], egui::Align::Center, "Center");
                            ui.selectable_value(&mut anchor[0], egui::Align::RIGHT, "Right");
                        });
                        ui.horizontal(|ui| {
                            ui.label("y:");
                            ui.selectable_value(&mut anchor[1], egui::Align::TOP, "Top");
                            ui.selectable_value(&mut anchor[1], egui::Align::Center, "Center");
                            ui.selectable_value(&mut anchor[1], egui::Align::BOTTOM, "Bottom");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Offset:");
                            ui.add(egui::DragValue::new(&mut anchor_offset.x));
                            ui.add(egui::DragValue::new(&mut anchor_offset.y));
                        });
                    });
                });
            });
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Disable for 2 seconds").clicked() {
                self.data.write().unwrap().disabled_time = ui.input(|i| i.time);
            }
            egui::reset_button(ui, self);
            ui.add(crate::egui_github_link_file!());
        });
    }
}
