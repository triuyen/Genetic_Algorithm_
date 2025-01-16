use bevy::prelude::*;
use simulation::InitPlugin;
use world::WorldPlugin;
use crate::simulation::ColorGroup;
use crate::simulation::GenerationNumber;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_fly_cam::FlyCamPlugin;
use bevy_debug_grid::*;
use bevy::prelude::Resource;

mod simulation;
mod world;



#[derive(Resource)]
struct SimulationState {
    running: bool,
}

fn main() {
    App::new()
        .insert_resource(SimulationState { running: false })
        .add_plugins((
            DefaultPlugins,
            WorldPlugin,
            InitPlugin,
            FlyCamPlugin,
            DebugGridPlugin::with_floor_grid(),
            EguiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (control_window_system, simulation_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GenerationNumber { current_gen: 0 });
}

fn control_window_system(mut contexts: EguiContexts, mut state: ResMut<SimulationState>, generate_counter: Res<GenerationNumber>) {
    egui::Window::new("Control Window").show(contexts.ctx_mut(), |ui| {
        // Display the current generation number
        ui.label(format!("Current Generation: {}", generate_counter.current_gen));

        // Add a button to start/stop the simulation
        if ui.button(if state.running { "Stop Simulation" } else { "Run Simulation" }).clicked() {
            state.running = !state.running;
        }
    });
}

fn simulation_system(
    state: Res<SimulationState>,
    mut commands: Commands,
    query: Query<(
        Entity,
        &Handle<StandardMaterial>,
        Option<&Parent>,
        Option<&Children>,
        Option<&Transform>,
        Option<&ColorGroup>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut generate_counter: ResMut<GenerationNumber>,
) {
    if state.running {
        simulation::process_generation_system(
            commands,
            query,
            materials,
            meshes,
            generate_counter,
        );
    }
}
