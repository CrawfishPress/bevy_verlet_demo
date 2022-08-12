/*

*/

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{Color32, Label};
use crate::{PitActive, GuiData, GameState};

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut action_check: ResMut<PitActive>,
                   mut random_data: ResMut<GuiData>,
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
    let ball_count = format!("Total balls added: {}", random_data.total_balls);

    // I can't believe this worked. See lessons.md
    let foo = &mut *random_data;
    let radius_slider = egui::Slider::new(&mut foo.radius_slider_value, 5.0..=50.0).step_by(5.0);
    let ball_slider = egui::Slider::new(&mut foo.ball_slider_value, 1..=500);

    egui::SidePanel::right("top panel?")
        .frame(my_frame)
        .min_width(700.0)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("This is a Header that does nothing useful");
            ui.horizontal(|ui| {
                ui.label("Radius: ");
                ui.add(radius_slider);
            });
            ui.horizontal(|ui| {
                ui.label("balls: ");
                ui.add(ball_slider);
                ui.add(some_label);
            });
            if ui.button("Click the magic-button").clicked() {
                action_check.is_paused = !action_check.is_paused;

                // Start the game
                if action_check.is_paused == false && action_check.game_status == GameState::NotStarted {
                    action_check.game_status = GameState::Running;
                }
            }
            ui.label(ball_count);
            ui.label("WARNING: there is no sanity-checking on ball-radius versus number of balls.");
            ui.label("You can fill up the pit!");
            if ui.button("RESET").clicked() {
                action_check.game_status = action_check.game_status.cycle();
            }
        });
}
