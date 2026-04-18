use bevy::prelude::{*};

use crate::ThreeDimCamera;

pub fn camera_controls(
    mut camera_query: Query<&mut Transform, With<ThreeDimCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut contexts: bevy_egui::EguiContexts,
){
    if !contexts.ctx_mut().unwrap().is_pointer_over_area() && !contexts.ctx_mut().unwrap().wants_keyboard_input(){
        for mut camera_transform in &mut camera_query {

            let boost = if keyboard.pressed(KeyCode::ShiftLeft) {2.} else {0.};
            let speed = 2. + boost;
    
            // Define the camera's rotation speed in radians per second
            let camera_rotation_speed_horizontal = 
                if keyboard.pressed(KeyCode::KeyQ)||keyboard.pressed(KeyCode::ArrowLeft){
                    speed
                }
                else if keyboard.pressed(KeyCode::KeyE)||keyboard.pressed(KeyCode::ArrowRight) {
                    -speed
                }
                else {
                    0.0
            };
    
            let camera_rotation_speed_vertical = 
                if keyboard.pressed(KeyCode::KeyR)||keyboard.pressed(KeyCode::ArrowUp){
                    speed
                }
                else if keyboard.pressed(KeyCode::KeyF)||keyboard.pressed(KeyCode::ArrowDown) {
                    -speed
                }
                else {
                    0.0
            };
    
            let camera_speed_horizontal = 
                if keyboard.pressed(KeyCode::KeyD){
                    speed
                }
                else if keyboard.pressed(KeyCode::KeyA) {
                    -speed
                }
                else {
                    0.0
            };
    
            let camera_speed_forward = 
                if keyboard.pressed(KeyCode::KeyW){
                    speed
                }
                else if keyboard.pressed(KeyCode::KeyS) {
                    -speed
                }
                else {
                    0.0
            };
    
            let camera_speed_vertical =
            if keyboard.pressed(KeyCode::Space){
                speed
            }
            else if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::KeyC) {
                -speed
            }
            else {
                0.0
            };
    
            let time_delta = time.delta_secs();
    
            // Calculate the camera's rotation angle based on time and speed
            let camera_rotation_angle_horizontal = time_delta * camera_rotation_speed_horizontal;
            let camera_rotation_angle_vertical = time_delta * camera_rotation_speed_vertical;
            let camera_vertical = time_delta * camera_speed_vertical;
            let camera_horizontal = time_delta * camera_speed_horizontal;
            let camera_forward = time_delta * camera_speed_forward;
    
            let side_movement = camera_transform.local_x().as_vec3();
            let forward_movement = -camera_transform.local_z().as_vec3();
    
            camera_transform.rotate(Quat::from_rotation_y(camera_rotation_angle_horizontal) * Quat::from_axis_angle(side_movement, camera_rotation_angle_vertical));
            camera_transform.translation.y += camera_vertical;
            camera_transform.translation +=  (Vec3::new(forward_movement.x,0.,forward_movement.z) * camera_forward) + (side_movement * camera_horizontal);
        }
    }

}