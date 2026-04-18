use bevy::{
    prelude::*,
};

use crate::two_dim_viz::TwoDimMesh;

use std::f32::consts::PI;
use prismatic_color::{Color as P_Color, constants as Color_Names};


trait BevyColorConvert {
    fn to_bevy_color(&self) -> Color;
}

impl BevyColorConvert for P_Color {
    fn to_bevy_color(&self) -> Color {
        let color = self.to_rgb().to_array();
        Color::srgba(color[0], color[1], color[2], color[3])
    }
}

pub fn generate_hues() -> Vec<Vec<P_Color>>{

    let protoary_color = vec![Color_Names::WHITE];
    let primary_colors = vec![
        Color_Names::RED,
        Color_Names::GREEN,
        Color_Names::BLUE,
    ];
    let secondary_colors = vec![
        Color_Names::RED,
        Color_Names::YELLOW,
        Color_Names::GREEN,
        Color_Names::CYAN,
        Color_Names::BLUE,
        Color_Names::MAGENTA,
    ];
    let tertiary_colors = vec![
        Color_Names::RED,
        Color_Names::ORANGE,
        Color_Names::YELLOW,
        Color_Names::CHARTREUSE,
        Color_Names::GREEN,
        Color_Names::MINT,
        Color_Names::CYAN,
        Color_Names::AZURE,
        Color_Names::BLUE,
        Color_Names::VIOLET,
        Color_Names::MAGENTA,
        Color_Names::ROSE,
    ];
    let quaternary_colors = vec![
        Color_Names::RED,
        Color_Names::VERMILLION,
        Color_Names::ORANGE,
        Color_Names::AMBER,
        Color_Names::YELLOW,
        Color_Names::BECQUEREL,
        Color_Names::CHARTREUSE,
        Color_Names::LIME,
        Color_Names::GREEN,
        Color_Names::EMERALD,
        Color_Names::MINT,
        Color_Names::TURQUOISE,
        Color_Names::CYAN,
        Color_Names::CAPRI,
        Color_Names::AZURE,
        Color_Names::CERULEAN,
        Color_Names::BLUE,
        Color_Names::INDIGO,
        Color_Names::VIOLET,
        Color_Names::PURPLE,
        Color_Names::MAGENTA,
        Color_Names::FUSCHIA,
        Color_Names::ROSE,
        Color_Names::RUBY,
    ];

    return vec![protoary_color, primary_colors, secondary_colors, tertiary_colors, quaternary_colors];
}


pub fn spawn(
    width: f32,
    height: f32,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_sets: Vec<Vec<P_Color>>,

) {

    let segment_ratio = 1.;
    let protoary_ratio = 1./8.;
    let start_radius = width.min(height) * protoary_ratio / 2.;

    for (stage, colors) in color_sets.into_iter().enumerate() {
        // Calculate number of segments for the current stage
        let num_segments = colors.len();
        // Set mesh size for the current stage
        let radius = start_radius + stage as f32 * segment_ratio * start_radius;
        let arc_angle =  PI / num_segments as f32;

        // Create the mesh for this stage
        let arc_mesh = Mesh2d(meshes.add(CircularSector::new(radius, arc_angle)));

        for (i, color) in colors.into_iter().enumerate() {
            let rotation_angle = -2. * i as f32 * arc_angle;
            commands.spawn((
                arc_mesh.clone(),
                MeshMaterial2d(materials.add(color.to_linear_rgb().to_bevy_color())),
                Transform::from_xyz(0.0, 0.0, 0.0 - stage as f32)
                    .with_rotation(Quat::from_rotation_z(rotation_angle)),
            )).insert(TwoDimMesh{});
        }
    }
}

