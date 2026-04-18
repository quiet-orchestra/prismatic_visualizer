use bevy::prelude::*;
use prismatic_color::Color as P_Color;

use crate::TwoDimCamera;

mod hue_wheel;
mod color_peaks;
mod gradients;

#[derive(Component, Clone)]
pub struct TwoDimMesh;

#[derive(Clone)]
pub enum VisualizerScene{
    HueWheel,
    ColorPeaks,
    Gradients,
}

trait ColorVisualizer{
    fn spawn(
        &self,
        width: f32,
        height: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        color_sets: Vec<Vec<P_Color>>,
    );
    fn despawn(
        commands: &mut Commands,
        query: Query<Entity, With<TwoDimMesh>>
    );
    fn generate_colors(
        &self,
        //Need to add color augmentation
    ) -> Vec<Vec<P_Color>>;
}

impl ColorVisualizer for VisualizerScene {
    fn spawn(
        &self,
        width: f32,
        height: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        color_sets: Vec<Vec<P_Color>>,
    ) {
        match self {
            VisualizerScene::HueWheel => hue_wheel::spawn(width, height, commands, materials, meshes, color_sets),
            VisualizerScene::ColorPeaks => color_peaks::spawn(width, height, commands, materials, meshes, color_sets),
            VisualizerScene::Gradients => gradients::spawn(width, height, commands, materials, meshes, images, color_sets),
        }
    }

    fn despawn(
        commands: &mut Commands,
        query: Query<Entity, With<TwoDimMesh>>
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }

    fn generate_colors(
        &self,
        //Need to add color augmentation
    ) -> Vec<Vec<P_Color>> {
        match self {
            VisualizerScene::HueWheel => hue_wheel::generate_hues(),
            VisualizerScene::ColorPeaks => color_peaks::generate_colors(),
            VisualizerScene::Gradients => gradients::generate_colors(),
        }
    }
}

#[derive(Resource)]
pub struct TwoDimSceneConfig {
    pos: usize,
    scenes: Vec<VisualizerScene>,
}

impl TwoDimSceneConfig {
    fn new() -> TwoDimSceneConfig {
        TwoDimSceneConfig { 
            pos: 0,
            scenes: vec![
                VisualizerScene::HueWheel,
                VisualizerScene::ColorPeaks,
                VisualizerScene::Gradients,
            ],
        }
    }
    pub fn advance(&mut self) {
        self.pos = 
            if self.pos + 1 < self.scenes.len() { self.pos + 1} else {0};
    }
    pub fn spawn_scene(
        &self,
        width: f32,
        height: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let scene = self.scenes.get(self.pos).expect("Scene out of range");
        let colors = scene.generate_colors();
        scene.spawn(width, height, commands, materials, meshes, images, colors);
    }
}

pub struct TwoDimViz;


impl Plugin for TwoDimViz {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, toggle_visualizers)
        .insert_resource(TwoDimSceneConfig::new());
    }
}

fn toggle_visualizers(
    camera: Single<&Camera, With<TwoDimCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    two_dim_mesh: Query<Entity, With<TwoDimMesh>>,
    mut scene_config: ResMut<TwoDimSceneConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {

    let size = camera.into_inner().viewport.as_ref().unwrap().physical_size;
    let (width , height) = (size.x as f32 , size.y as f32);

    if keyboard.just_pressed(KeyCode::KeyT) {
        scene_config.advance();
        println!("Advanced to {}", scene_config.pos);
        VisualizerScene::despawn(&mut commands, two_dim_mesh);
        scene_config.spawn_scene(width, height, &mut commands, &mut meshes, &mut materials, &mut images);
    }
}
 