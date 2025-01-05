use bevy::prelude::*;
use simulation::InitPlugin;
use world::WorldPlugin;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_fly_cam::FlyCamPlugin;
use bevy_debug_grid::*;
mod simulation;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
                        WorldPlugin,           
                        InitPlugin,
                        FlyCamPlugin,
                        DebugGridPlugin::with_floor_grid()))
        .add_plugins(EguiPlugin)
        .add_systems(Update, e_gui)
        .run();
}

fn e_gui(
    mut contexts: EguiContexts,
    commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>)>,
    ) {

    egui::Window::new("Example Window").show(contexts.ctx_mut(), |ui| {
        if ui.button("Run_simulation").clicked() {
            println!("Test");
            //simulation::evaluate_fitness(commands, query, materials);
            //simulation::selective_reproduction(commands, query, materials, meshes);
        }
    });
}
