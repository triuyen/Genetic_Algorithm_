use bevy::prelude::*;
use camera::CameraPlugin;
use simulation::InitPlugin;
use world::WorldPlugin;
use bevy_fly_cam::FlyCamPlugin;
use bevy_debug_grid::*;

mod camera;
mod simulation;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
                        WorldPlugin, 
                        CameraPlugin, 
                        FlyCamPlugin, 
                        InitPlugin,
                        DebugGridPlugin::with_floor_grid()))
        .run();
}