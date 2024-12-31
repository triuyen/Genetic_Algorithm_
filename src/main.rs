use bevy::{prelude::*, render::render_resource::encase::internal::SizeValue};
use simulation::InitPlugin;
use world::WorldPlugin;
use bevy_fly_cam::FlyCamPlugin;
use bevy_debug_grid::*;
//use main_ui::ButtonPlugin;
//mod main_ui;
// mod camera;
mod simulation;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
                        //ButtonPlugin,
                        WorldPlugin, 
                        FlyCamPlugin, 
                        InitPlugin,
                        DebugGridPlugin::with_floor_grid()))
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_ui)
        .add_systems(Update, button_interaction_system)
        .run();
}


const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
// use bevy::winit::WinitSettings;


/// UI setup: adds a button to start the simulation
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera for UI
    commands.spawn(Camera2dBundle::default());

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|button| {
                    // Button text
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Start Simulation",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
        });
}

/// System to handle button interaction
fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                println!("Simulation started!");
                // Add logic to start simulation here
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}