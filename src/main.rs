use bevy::prelude::*;
use simulation::InitPlugin;
use world::WorldPlugin;
use bevy_fly_cam::FlyCamPlugin;
use bevy_debug_grid::*;

// mod camera;
mod simulation;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
                        WorldPlugin, 
                        FlyCamPlugin, 
                        InitPlugin,
                        DebugGridPlugin::with_floor_grid()))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
use bevy::winit::WinitSettings;

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                println!("Button clicked!");
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        // width: Val::Px(150.0),
                        // height: Val::Px(65.0),
                        // border: UiRect::all(Val::Px(5.0)),
                        // // horizontally center child text
                        // justify_content: JustifyContent::Center,
                        // // vertically center child text
                        // align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                // .with_child((
                //     Text::new("Button"),
                //     TextFont {
                //         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                //         font_size: 33.0,
                //         ..default()
                //     },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}