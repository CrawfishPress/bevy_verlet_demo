/*
Experiments in learning egui.
https://docs.rs/egui/latest/egui/

This Does nothing? How does Frame interact?
ui.style_mut().spacing.window_margin = Margin {left: 100.0, right: 100.0, top: 50.0, bottom: 50.0};
*/

use bevy::prelude::*;
use bevy_egui::*;
use egui::*;
use crate::{PitActive, GuiData, GameState};

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut action_check: ResMut<PitActive>,
                   mut random_data: ResMut<GuiData>,
) {
    let my_frame = egui::containers::Frame {
        outer_margin: Default::default(),
        inner_margin: egui::style::Margin { left: 20.0, right: 20.0, top: 20.0, bottom: 20.0 },
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
    let timer_slider = egui::Slider::new(&mut foo.circle_delay, 100..=1000).step_by(100 as f64);

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
    let big_text_btn_9 = RichText::new("Release-Delay: ")
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

    let big_text_lbl_9 = RichText::new("delay in milliseconds - low delay may cause pop-corning")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let big_text_lbl_11 = RichText::new("BROWSER? Hit <ctrl>- a few times to shrink, while we work on it.")
        .color(Color32::WHITE).font(FontId::proportional(40.0));

    egui::SidePanel::right("top panel?")
        .frame(my_frame)
        .min_width(700.0)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 {x: 10.0, y: 20.0};

            ui.label(big_text_lbl_1);

            ui.horizontal(|ui| {  // Radius-slider
                ui.style_mut().spacing.slider_width = 200.0;
                ui.label(big_text_btn_5);
                ui.add_sized([300.0, 30.0], radius_slider);
            });

            ui.horizontal(|ui| { // Ball-count slider
                ui.style_mut().spacing.slider_width = 500.0;
                ui.label(big_text_btn_7);
                ui.add(ball_slider);
            });

            ui.add(Label::new(big_text_lbl_7));

            ui.horizontal(|ui| { // Release-delay slider
                ui.style_mut().spacing.slider_width = 200.0;
                ui.label(big_text_btn_9);
                ui.add(timer_slider);
                ui.label(big_text_lbl_9);
            });

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
            ui.label(big_text_lbl_11);
        });
}
