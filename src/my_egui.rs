/*
Experiments in learning egui.
https://docs.rs/egui/latest/egui/
*/

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_egui::*;
use egui::*;
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

    let ball_count = format!("Total balls added: {}", random_data.total_balls);

    // I can't believe this worked. See lessons.md
    let foo = &mut *random_data;
    let radius_slider = egui::Slider::new(&mut foo.radius_slider_value, 5.0..=50.0).step_by(5.0);
    let ball_slider = egui::Slider::new(&mut foo.ball_slider_value, 1..=500);

    let pause_string: String;
    if action_check.is_paused {
        pause_string = "Click the magic-button to start".parse().unwrap();
    } else {
        pause_string = "Click the magic-button to pause".parse().unwrap();
    }

    let big_text_btn_1 = RichText::new(pause_string)
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let big_text_btn_3 = RichText::new("Reset Stuff")
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let big_text_btn_5 = RichText::new("Radius: ")
        .color(Color32::GREEN).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(20.0);
    let big_text_btn_7 = RichText::new("Balls: ")
        .color(Color32::GREEN).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(20.0);

    let big_text_lbl_1 = RichText::new("Verlet Equations-of-Motion Demo, in Rust/Bevy")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let big_text_lbl_3 = RichText::new(ball_count)
        .color(Color32::BLACK).background_color(Color32::GRAY).font(FontId::proportional(20.0)).size(25.0);

    let big_text_lbl_5 = RichText::new("WARNING: there is no sanity-checking on ball-radius versus number of balls. \
    \nYou can fill up the pit!")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let big_text_lbl_7 = RichText::new("(I think you can also click the boxes, to enter a number)")
        .color(Color32::BLACK).background_color(Color32::GRAY).font(FontId::proportional(20.0)).size(25.0);


    egui::SidePanel::right("top panel?")
        .frame(my_frame)
        .min_width(700.0)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(big_text_lbl_1);
            ui.horizontal(|ui| {
                ui.label(big_text_btn_5);
                ui.add_sized([100.0, 20.0], radius_slider);
            });
            ui.horizontal(|ui| {
                ui.label(big_text_btn_7);
                ui.add(ball_slider);
            });
            ui.add(Label::new(big_text_lbl_7));
            if ui.button(big_text_btn_1).clicked() {
                action_check.is_paused = !action_check.is_paused;

                // Start the game
                if action_check.is_paused == false && action_check.game_status == GameState::NotStarted {
                    action_check.game_status = GameState::Running;
                }
            }
            ui.label(big_text_lbl_5);
            ui.add(Label::new(big_text_lbl_3));

            if ui.button(big_text_btn_3).clicked() {
                action_check.game_status = action_check.game_status.cycle();
            }
        });
}
