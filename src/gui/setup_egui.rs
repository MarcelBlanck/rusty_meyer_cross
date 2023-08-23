use std::cmp;

use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::meyer_cross::{
    attack_sequence::AttackSequenceLength,
    preparation::{Preparation, StreichenCount},
    types::Strike,
};

use bevy::{app::Plugin, prelude::*};

pub struct MeyerUiPlugin;

impl Plugin for MeyerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(UiState::new())
            .add_systems(Update, update_ui);
    }
}

#[derive(Resource)]
pub struct UiState {
    pub streichen: bool,
    pub random_streichen: bool,
    pub huten: bool,
    pub tag_left: bool,
    pub tag_right: bool,
    pub tag_above: bool,
    pub pflug_left: bool,
    pub pflug_right: bool,
    pub ochs_left: bool,
    pub ochs_right: bool,
    pub eisenport: bool,
    pub langort: bool,
    pub alber: bool,
    pub long_edge_allowed: bool,
    pub short_edge_allowed: bool,
    pub flat_allowed: bool,
    pub fehler_allowed: bool,
    pub doppelfehler_allowed: bool,
    pub attack_count_min: usize,
    pub attack_count_max: usize,
    pub timer_active: bool,
    pub time_for_preparation_ms: usize,
    pub time_per_strike_ms: usize,
    pub apply_pending: bool,
}

impl UiState {
    fn new() -> UiState {
        UiState {
            streichen: true,
            random_streichen: false,
            huten: false,
            tag_left: false,
            tag_right: false,
            tag_above: false,
            pflug_left: false,
            pflug_right: false,
            ochs_left: false,
            ochs_right: false,
            eisenport: false,
            langort: false,
            alber: false,
            long_edge_allowed: true,
            short_edge_allowed: false,
            flat_allowed: false,
            fehler_allowed: false,
            doppelfehler_allowed: false,
            attack_count_min: 4,
            attack_count_max: 4,
            timer_active: false,
            time_for_preparation_ms: 2000,
            time_per_strike_ms: 2000,
            apply_pending: false,
        }
    }
}

macro_rules! setting_checkbox {
    ($ui:expr, $ui_state:expr, $text:expr, $property:ident) => {
        $ui.label($text);
        $ui.checkbox(&mut $ui_state.$property, "");
        $ui.end_row();
    };
}

fn update_ui(mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    let mut apply = false;
    egui::Window::new("Meyer Cross Settings").show(ctx, |ui| {
        egui::Grid::new("preview").show(ui, |ui| {
            setting_checkbox!(ui, ui_state, "Streichen Preparation", streichen);
            if ui_state.streichen {
                setting_checkbox!(ui, ui_state, "Random Streichen Count", random_streichen);
            }

            setting_checkbox!(ui, ui_state, "Huten Preperation", huten);
            if ui_state.huten {
                setting_checkbox!(ui, ui_state, "Tag Left", tag_left);
                setting_checkbox!(ui, ui_state, "Tag Right", tag_right);
                setting_checkbox!(ui, ui_state, "Tag Above", tag_above);
                setting_checkbox!(ui, ui_state, "Pflug Left", pflug_left);
                setting_checkbox!(ui, ui_state, "Pflug Right", pflug_right);
                setting_checkbox!(ui, ui_state, "Ochs Left", ochs_left);
                setting_checkbox!(ui, ui_state, "Ochs Right", ochs_right);
                setting_checkbox!(ui, ui_state, "Eisenport", eisenport);
                setting_checkbox!(ui, ui_state, "Alber", alber);
            }

            ui.label("Allowed Strikes:");
            ui.end_row();
            setting_checkbox!(ui, ui_state, "Long Edge", long_edge_allowed);
            setting_checkbox!(ui, ui_state, "Short Edge", short_edge_allowed);
            setting_checkbox!(ui, ui_state, "Flat", flat_allowed);
            setting_checkbox!(ui, ui_state, "Fehler", fehler_allowed);
            if ui_state.fehler_allowed {
                setting_checkbox!(ui, ui_state, "Doppelfehler", doppelfehler_allowed);
            }

            ui.add(
                egui::Slider::new(
                    &mut ui_state.attack_count_min,
                    AttackSequenceLength::MIN_LENGTH..=AttackSequenceLength::MAX_LENGTH,
                )
                .text("Attack Count Min"),
            );
            ui.end_row();

            if ui_state.attack_count_min > ui_state.attack_count_max {
                ui_state.attack_count_max = ui_state.attack_count_min;
            }

            ui.add(
                egui::Slider::new(
                    &mut ui_state.attack_count_max,
                    AttackSequenceLength::MIN_LENGTH..=AttackSequenceLength::MAX_LENGTH,
                )
                .text("Attack Count Max"),
            );
            ui.end_row();

            if ui_state.attack_count_max < ui_state.attack_count_min {
                ui_state.attack_count_min = ui_state.attack_count_max;
            }

            ui.label("Timer active:");
            ui.checkbox(&mut ui_state.timer_active, "");
            ui.end_row();

            if ui_state.timer_active {
                ui.add(
                    egui::Slider::new(&mut ui_state.attack_count_max, 1..=10)
                        .text("Time for Preparation (sec)"),
                );
                ui.end_row();
                ui.add(
                    egui::Slider::new(&mut ui_state.attack_count_max, 1..=10)
                        .text("Time per Strike (sec)"),
                );
                ui.end_row();
            }
        });

        if ui.button("Apply").clicked() {
            ui_state.apply_pending = true;
        }
    });
}
