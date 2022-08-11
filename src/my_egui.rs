/*

*/

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{Color32, Label};
use crate::SpritesMovable;

// GUI Resource
#[derive(Debug, Default)]
pub struct GuiData {
    pub some_name: String,
    pub my_value: i32,
    pub my_other_value: f64,
}

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut random_data: ResMut<GuiData>,
                   mut move_active: ResMut<SpritesMovable>,
) {
    let my_frame = egui::containers::Frame {
        outer_margin: Default::default(),
        inner_margin: egui::style::Margin { left: 10.0, right: 10.0, top: 10.0, bottom: 10.0 },
        rounding: Default::default(),
        // shadow: epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
        shadow: Default::default(),
        fill: Color32::DARK_GREEN,
        // stroke: egui::Stroke::new(2.0, Color32::BLACK),  // Border
        stroke: Default::default(),
    };

    let some_label = Label::new("I think you can also click the box to enter a number "); // Testing...

    egui::SidePanel::right("top panel?")
        .frame(my_frame)
        .min_width(800.0)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("This is a Header that does nothing useful");
            ui.horizontal(|ui| {
                ui.label("balls: ");
                ui.add(egui::Slider::new(&mut random_data.my_other_value, 0.0..=500.0));
                ui.add(some_label);
            });
            if ui.button("Click the magic-button").clicked() {
                random_data.my_value += 1;
                move_active.is_active = !move_active.is_active;
            }
        });
}
