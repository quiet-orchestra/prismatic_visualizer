//Quiet Orchestra
//Prismatic Color Visualizer

use bevy::{prelude::*, render::view::NoIndirectDrawing};
use bevy_egui::{
    EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext
};

mod camera;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use camera::camera_controls;

mod ui;
use ui::{ui_overlay, VisualizationSettings};

mod three_dim_viz;
use three_dim_viz::{
    spawn_3d_visualization, 
    spawn_grid, 
    VisualizationMesh, 
    SCALE
};

mod two_dim_viz;
use two_dim_viz::TwoDimViz;

use bevy_pointcloud::{
    render::PointCloudRenderMode, 
    PointCloudPlugin, 
    point_cloud::{PointCloud}, 
    point_cloud_material::PointCloudMaterial,
};

use crate::ui::{ColorChannel, StepType};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window: Some(Window {
                title: "Prismatic Visualizer".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PointCloudPlugin)
        // .add_plugins(TwoDimViz)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_visualization, update_gizmo_config, update_grid))
        .add_systems(FixedUpdate, camera_controls)
        .add_systems(EguiPrimaryContextPass, ui_overlay)
        .run();
}
 
fn setup(
    gizmos: Gizmos,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    point_clouds: ResMut<Assets<PointCloud>>,
    point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
) {

    commands.spawn((
        PrimaryEguiContext,
        PanOrbitCamera::default(),
        Transform::from_xyz(SCALE*2., SCALE*2., SCALE*2.).looking_at(Vec3::new(0., 0., 0.), Vec3::Z),
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode {
            use_edl: false,
            edl_radius: 2.8,
            edl_strength: 0.4,
            edl_neighbour_count: 4,
            ..Default::default()
        },
    ));

    let channel_settings: (ColorChannel, ColorChannel, ColorChannel) = (
        ColorChannel { start: 0., end: 1., steps: 12, step_type: StepType::Forward },
        ColorChannel { start: 0., end: 1., steps: 8, step_type: StepType::Inclusive },
        ColorChannel { start: 0., end: 1., steps: 8, step_type: StepType::Inclusive},
    );

    let settings = VisualizationSettings {
        channel_settings,
        ..Default::default()
    };

    let settings_copy = settings.clone();

    commands.insert_resource(settings);

    spawn_3d_visualization(gizmos, commands, meshes, materials, point_clouds, point_cloud_materials, &settings_copy);

}
 
fn update_visualization(
    gizmos: Gizmos,
    mut commands: Commands,
    visualization_settings: ResMut<VisualizationSettings>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    point_clouds: ResMut<Assets<PointCloud>>,
    point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    entities: Query<Entity, With<VisualizationMesh>>,
) 
 {

     if visualization_settings.is_changed() {
 
         //Delete previous visualization 
         for mesh in entities.iter(){
             commands.entity(mesh).despawn();
         }
  
        spawn_3d_visualization(gizmos, commands, meshes, materials, point_clouds, point_cloud_materials, & *visualization_settings);
        
    }
 }

 fn update_grid(
    gizmos: Gizmos,
    visualization_settings: ResMut<VisualizationSettings>,
 ){
    spawn_grid(gizmos, &visualization_settings);
 }

fn update_gizmo_config(
    mut config_store: ResMut<bevy::prelude::GizmoConfigStore>,
    visualization_settings: Res<VisualizationSettings>,
) {
    if visualization_settings.is_changed() {
        let (config, _handle) = config_store.config_mut::<bevy::prelude::DefaultGizmoConfigGroup>();
        config.line.width = visualization_settings.line_width * 5. ;
        config.line.joints = GizmoLineJoint::Miter;

    }
}

