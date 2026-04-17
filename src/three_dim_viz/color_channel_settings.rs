use bevy::{ecs::component::Component, reflect::Reflect};
use bevy_egui::egui::{DragValue, Ui};
use egui_double_slider::DoubleSlider;
use crate::ui::ui_traits::Setting;

#[derive(Debug, Clone, Copy)]
pub struct ColorChannelSettings(pub ColorChannel, pub ColorChannel, pub ColorChannel,);

impl Setting for ColorChannelSettings {
    fn heading(&self) -> &str{
        "Perceptual Offset"
    }
    fn ui(&mut self, ui: &mut Ui){

        let width = ui.available_width();

        ui.horizontal(|ui| {
                    ui.label("Channel Settings");
        });

        //Channel A
        ui_channel( ui, "A", &mut self.0, width);

        //Channel B
        ui_channel( ui, "B", &mut self.1, width);

        //Channel C
        ui_channel( ui, "C", &mut self.2, width);
    }
}

impl Default for ColorChannelSettings {
    fn default() -> ColorChannelSettings {
        ColorChannelSettings(
            ColorChannel { start: 0., end: 1., steps: 12, step_type: StepType::Forward },
            ColorChannel { start: 0., end: 1., steps: 8, step_type: StepType::Inclusive },
            ColorChannel { start: 0., end: 1., steps: 8, step_type: StepType::Inclusive},
        )
    }
}

#[derive(Component, Debug, Clone, Reflect, Copy)]
pub struct ColorChannel {
    pub start: f32,
    pub end: f32,
    pub steps: usize,
    pub step_type: StepType,
}

impl Default for ColorChannel {
    fn default() -> Self {
        Self {
            start: 0.,
            end: 1.,
            steps: 8,
            step_type: StepType::Forward,
        }
    }
}

impl ColorChannel {
    pub fn generate_range(&self, along_grain: bool) -> (f32, usize, f32) {

        let mut start = self.start;
        let mut wrap = false;

        match self.step_type {
                StepType::Forward => {},
                StepType::Reverse => start = self.end,
                StepType::Inclusive => wrap = true,
        };



        let steps = if wrap { self.steps + 1 } else { self.steps };
        let steps = if along_grain { steps - 1 } else { steps };

        let step_size = (self.end - self.start) / (self.steps as f32);

        (start, steps, step_size)
    }

}

#[derive(Component, Debug, Clone, Reflect, PartialEq, Copy)]
pub enum StepType {
    Forward,
    Reverse,
    Inclusive,
}

fn ui_channel(ui: &mut Ui, label: &str, channel: &mut ColorChannel, width: f32) {
    // Steps
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(
            DragValue::new(&mut channel.steps)
                .range(1..=24)
                .prefix("Steps: "),
        );
    });

    // Step type
    ui.horizontal(|ui| {
        ui.selectable_value(&mut channel.step_type, StepType::Forward, "Forward");
        ui.selectable_value(&mut channel.step_type, StepType::Reverse, "Backward");

        if channel.steps == 1 && channel.step_type == StepType::Inclusive {
            channel.step_type = StepType::Forward;
        } else {
            ui.selectable_value(&mut channel.step_type, StepType::Inclusive, "Inclusive");
        }
    });

    // Start/End slider
    let mut start = channel.start;
    let mut end = channel.end;

    ui.horizontal(|ui| {
        ui.add(
            DoubleSlider::new(&mut start, &mut end, 0.0..=1.0)
                .width(width)
                .separation_distance(0.0),
        );
    });

    channel.start = start;
    channel.end = end;
}