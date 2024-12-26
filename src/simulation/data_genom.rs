// use bevy::prelude::*;
// use bevy::render::render_resource::ShaderType;
// use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub fn get_color_map() -> HashMap<char, (f32, f32, f32)> {
    HashMap::from([
        ('A', (0.0, 0.0, 1.0)), // Blue
        ('B', (0.0, 1.0, 0.0)), // Green
        ('C', (1.0, 0.0, 0.0)), // Red
        ('D', (1.0, 1.0, 0.0)), // Yellow
        ('E', (0.0, 1.0, 1.0)), // Cyan
    ])
}

// #[derive(Resource, ShaderType, Default, Debug, Clone, Copy)]
// pub struct GpuSimulationParams {
//     pub time: f32,
//     pub delta_time: f32,
//     pub growth_rate: f32,
//     pub kernel_radius: f32,
//     pub grid_size: UVec2,
//     pub pause: u32,
// }

// pub mod genome {
//     // Declare the list of genomes
//     pub fn get_list_genome() -> Vec<&'static str> {
//         vec!["tail", "fines", "head", "body", "limb"]
//     }
// }

// { 'A' : (0.0, 0.0, 1.0) , 'B': (0.0, 1.0, 0.0), 'C': (1.0, 0.0, 0.0), 'D':(1.0, 1.0, 0.0), 'E':(0.0, 1.0, 1.0),}


// #[derive(Resource, Serialize, Deserialize, Debug, Clone)]
// pub struct SimulationSettings {
//     pub grid_size: UVec2,
//     pub growth_rate: f32,
//     pub kernel_radius: f32,
//     pub time_scale: f32,
//     pub pause: bool,
// }

// impl Default for SimulationSettings {
//     fn default() -> Self {
//         Self {
//             grid_size: UVec2::new(256, 256),
//             growth_rate: 0.1,
//             kernel_radius: 10.0,
//             time_scale: 1.0,
//             pause: false,
//         }
//     }
// }