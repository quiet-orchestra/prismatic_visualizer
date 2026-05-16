use std::ops::IndexMut;

use bevy::{prelude::*};
use bevy_egui::{
    EguiContexts,
    egui::{
        self, Ui, global_theme_preference_buttons
    }
};

pub(crate) mod ui_traits;

use crate::{
    UiCamera, ViewportState, three_dim_viz::{
        Attribution,
        ColorChannelSettings,
        ColorModelSettings,
        ControlSettings,
        DimensionalitySettings,
        GridSettings,
        PerceptualOffsetSettings,
        ScaleSettings,
    }, ui::ui_traits::{Setting, SettingsMenu}
};

#[derive(Resource, Clone, Copy)]
pub struct Settings{

    pub viewport_state: ViewportState,

    pub scale_settings: ScaleSettings,

    pub grid_settings: GridSettings,

    pub perceptual_offset_settings: PerceptualOffsetSettings,

    pub color_channel_settings: ColorChannelSettings,

    pub color_model_settings: ColorModelSettings,

    pub dimensionality_settings: DimensionalitySettings,

    pub controls_settings: ControlSettings,

    pub attribution: Attribution,

    pub active_setting: SettingsOption,

}

#[derive(PartialEq, Clone, Copy)]
enum SettingsOption {
    Grid,
    Offset,
    Channels,
    Model,
    Dim,

    Gradient,
    Picker,

    Controls,
    Attr,

}


impl Settings {

    fn three_dim_settings() -> Vec<SettingsOption> {
        vec![
            SettingsOption::Grid,
            SettingsOption::Dim,
        ]
    }

    fn two_dim_settings() -> Vec<SettingsOption> {
        vec![
            SettingsOption::Gradient,
            SettingsOption::Picker,
        ]
    }

    fn shared_settings() -> Vec<SettingsOption> {
        vec![
            SettingsOption::Offset,
            SettingsOption::Channels,
            SettingsOption::Model,
        ]
    }

    fn bottom_settings() -> Vec<SettingsOption> {
        vec![
            SettingsOption::Controls,
            SettingsOption::Attr,
        ]
    }


    pub fn display_mode_setting(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
            ui.selectable_value(&mut self.viewport_state, ViewportState::ThreeDimOnly, "3D");
            ui.selectable_value(&mut self.viewport_state, ViewportState::TwoDimOnly, "2D");
            ui.selectable_value(&mut self.viewport_state, ViewportState::SplitDim, "Split");
            ui.separator();
                
        });
    }

   

    pub fn settings_ribbon_ui(&mut self, ui: &mut Ui){

        let options: Vec<SettingsOption> = match self.viewport_state {
            ViewportState::ThreeDimOnly => {
                [
                Settings::three_dim_settings(),
                Settings::shared_settings(),
                ].concat()
            },
            ViewportState::TwoDimOnly => {
                [
                Settings::shared_settings(),
                Settings::two_dim_settings(),
                ].concat()
            },
            ViewportState::SplitDim => {
                [
                Settings::three_dim_settings(),
                Settings::shared_settings(),
                Settings::two_dim_settings(),
                ].concat()
            },
        };

        // ui.horizontal(|ui| {

            // ui.label(self.settings_menu.heading);

            // let min_text = if self.settings_menu.minimized {"Ʌ"} else {"V"};
            // let is_minimized = self.settings_menu.minimized;
            // ui.selectable_value(&mut self.settings_menu.minimized, !is_minimized , min_text);
            
            // ui.separator();

            // if !self.settings_menu.minimized {
        ui.horizontal_wrapped(|ui|{
            for option in options {
                ui.selectable_value( &mut self.active_setting, option, Settings::setting_heading(option));
            }  
        });

            // }

        // });

    }

    pub fn display_bottom_settings(&mut self, ui: &mut Ui) {
        let options = Settings::bottom_settings();

        ui.horizontal_wrapped(|ui|{
            for option in options {
                ui.selectable_value( &mut self.active_setting, option, Settings::setting_heading(option));
            }  
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui|{
                global_theme_preference_buttons(ui);
            });
            
        });
    }

    fn display_selected_setting(&mut self, ui: &mut Ui) {
        match self.active_setting {
            SettingsOption::Grid => self.grid_settings.ui(ui),
            SettingsOption::Offset => self.perceptual_offset_settings.ui(ui),
            SettingsOption::Channels => self.color_channel_settings.ui(ui),
            SettingsOption::Model => self.color_model_settings.ui(ui),
            SettingsOption::Dim => self.dimensionality_settings.ui(ui),
            SettingsOption::Gradient => {},
            SettingsOption::Picker => {},
            SettingsOption::Controls => self.controls_settings.ui(ui),
            SettingsOption::Attr => self.attribution.ui(ui),
        }
    }

    fn setting_heading(option: SettingsOption) -> &'static str {
        match option {
            SettingsOption::Grid => GridSettings::heading(),
            SettingsOption::Offset => PerceptualOffsetSettings::heading(),
            SettingsOption::Channels => ColorChannelSettings::heading(),
            SettingsOption::Model => ColorModelSettings::heading(),
            SettingsOption::Dim => DimensionalitySettings::heading(),
            SettingsOption::Gradient => "Gradient",
            SettingsOption::Picker => "Picker",
            SettingsOption::Controls => ControlSettings::heading(),
            SettingsOption::Attr => Attribution::heading(),
        }
    }

}

impl Default for Settings{
    fn default() -> Self {
        Self {

            viewport_state: ViewportState::default(),
            
            scale_settings: ScaleSettings::default(),

            grid_settings: GridSettings::default(),

            perceptual_offset_settings: PerceptualOffsetSettings::default(),

            color_channel_settings: ColorChannelSettings::default(),

            color_model_settings: ColorModelSettings::default(),

            dimensionality_settings: DimensionalitySettings::default(),

            controls_settings: ControlSettings::default(),

            attribution: Attribution::default(), 

            active_setting: SettingsOption::Grid,


        }
    }
}

pub fn ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    camera: Single<&Camera, With<UiCamera>>,
) {

    let size = camera.into_inner().viewport.as_ref().unwrap().physical_size;
    let (_ , height) = (size.x as f32 , size.y as f32);


    egui::TopBottomPanel::top("Settings")
        .max_height(height)
        .show(contexts.ctx_mut().unwrap(), | ui|{
            egui::Sense::hover();

            settings.display_mode_setting(ui);
            ui.separator();
            settings.settings_ribbon_ui(ui);

        });

        egui::TopBottomPanel::bottom("bottom")
        .show(contexts.ctx_mut().unwrap(), | ui|{

            settings.display_bottom_settings(ui);
            
        });

        egui::CentralPanel::default()
        .show(contexts.ctx_mut().unwrap(), | ui | {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            
                settings.display_selected_setting(ui);

            });
        });




    

}
