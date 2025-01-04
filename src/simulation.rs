use bevy::{scene::ron::value::Float, transform::commands};
use rand::Rng;
use bevy::prelude::Color;
use std::time::Duration;
use std::thread;
use bevy::{prelude::*, transform};
pub mod data_genom;

// let's determine that we want at the end of the simulation yellow cubes sets only

const POPULATION_SIZE:usize = 10;
const TARGET:usize = 80;
const MAX_GENERATIONS:usize = 1000;

// All components
#[derive(Component,Debug)]
pub struct CubeAttributes {
    color_group: u8, // Attribute to determine the color group
}

#[derive(Component)]
struct Mover {
    velocity: Vec3,
}


pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_gen)
            .add_systems(Update, move_cubes);
            //.add_systems(Update, run_simulation);
            //.add_systems(Update, reproduction);
    }
}

#[derive(Component)]
struct ParentCube;
#[derive(Component)]
struct ChildCube;

fn spawn_first_gen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..POPULATION_SIZE {
        // Spawn the parent cube
        let random_x = rng.gen_range(-3.0..5.0);
        let random_y = rng.gen_range(-5.0..5.0);
        let random_z = rng.gen_range(-3.0..5.0);

        let velocity = Vec3::new(
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
        );

        let parent_position = Vec3::new(random_x, random_y, random_z);

        // Generate the material for the parent (cube 1)
        let parent_color_group = rng.gen_range(0..5); // Random color group for the parent cube
        let parent_material = generate_material(parent_color_group, &mut materials);

        // Spawn the parent cube
        let parent_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: parent_material,
                transform: Transform {
                    translation: parent_position,
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            })
            .insert(Mover { velocity })
            .insert(ParentCube)
            .id(); // Save the entity ID to use as a parent

        // Generate the material for the child (cube 2)
        let child_color_group = rng.gen_range(0..5); // Random color group for the child cube
        let child_material = generate_material(child_color_group, &mut materials);

        // Spawn the second mesh slightly offset from the first
        let children = commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: child_material,
            transform: Transform {
                translation: Vec3::new(1.0, 0.0, 0.0), //+ Vec3::new(0.01, 0.0, 0.0), // Offset to the side
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(ChildCube)
        .id();

        // Parent the child to the parent cube
        commands.entity(parent_entity).add_child(children);
    }
}

// generate color for the cubes.
fn generate_material(
    color_group: u8,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    // Generate a color based on the color group
    let color = match color_group {
        0 => Color::srgb(1.0, 0.0, 0.0), // Red
        1 => Color::srgb(0.0, 1.0, 0.0), // Green
        2 => Color::srgb(0.0, 0.0, 1.0), // Blue
        3 => Color::srgb(1.0, 1.0, 0.0), // Yellow
        _ => Color::srgb(1.0, 1.0, 1.0), // Default White
    };

    // Create and return a new material
    materials.add(StandardMaterial {
        base_color: color,
        ..default()
    })
}

// Update the cubes' positions
fn move_cubes(mut query: Query<(&Mover, &mut Transform)>, time: Res<Time>) {
    for (mover, mut transform) in query.iter_mut() {
        transform.translation += mover.velocity * time.delta_seconds();

        // Keep the cubes within bounds (-5, 5)
        if transform.translation.x.abs() > 5.0 {
            transform.translation.x = transform.translation.x.signum() * 20.0;
        }
        if transform.translation.y.abs() > 5.0 {
            transform.translation.y = transform.translation.y.signum() * 20.0;
        }
        if transform.translation.z.abs() > 5.0 {
            transform.translation.z = transform.translation.z.signum() * 20.0;
        }
        // Limit the movement to within a 30x30x30 space
        transform.translation.x = transform.translation.x.clamp(-30.0, 30.0);
        transform.translation.y = transform.translation.y.clamp(-30.0, 30.0);
        transform.translation.z = transform.translation.z.clamp(-30.0, 30.0);
    }
}

// evaluate weither the gene is yellow or not __________________________________________________________
pub fn evaluate_fitness(
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let yellow = Color::srgb(1.0, 1.0, 0.0); // Yellow color definition
    println!("Running Evaluation fitness !");
    for (entity, material_handle, parent, children) in query.iter() {
        // Access material's base color
        let is_entity_yellow = if let Some(material) = materials.get(material_handle) {
            material.base_color == yellow
        } else {
            false
        };

        // Parent-Child logic
        if let Some(parent) = parent {
            // If the entity is a child, handle based on the parent's status
            if let Ok((_, parent_material_handle, _, _)) = query.get(parent.get()) {
                let is_parent_yellow = if let Some(parent_material) = materials.get(parent_material_handle) {
                    parent_material.base_color == yellow
                } else {
                    false
                };

                // If neither parent nor this child is yellow, despawn the child
                if !is_entity_yellow && !is_parent_yellow {
                    commands.entity(entity).despawn();
                }
            }
        } else if let Some(children) = children {
            // If the entity is a parent, evaluate children
            let mut has_yellow_child = false;

            for &child in children.iter() {
                if let Ok((_, child_material_handle, _, _)) = query.get(child) {
                    if let Some(child_material) = materials.get(child_material_handle) {
                        if child_material.base_color == yellow {
                            has_yellow_child = true;
                            break;
                        }
                    }
                }
            }

            // If neither the parent nor any children are yellow, despawn all
            if !is_entity_yellow && !has_yellow_child {
                commands.entity(entity).despawn_recursive(); // Despawn parent and all children
            }
        }
    }
}


// Perform reproduction using arithmetic crossover __________________________________________________________
pub fn reproduction(
    mut commands: Commands,
    mut query: Query<(&CubeAttributes, &Transform), With<ParentCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut rng = rand::thread_rng();
    let mut new_cubes = Vec::new();
    let parents: Vec<_> = query.iter().collect();

    // Pair up parents for crossover
    for i in (0..parents.len()).step_by(2) {
        if i + 1 < parents.len() {
            let (parent1, transform1) = parents[i];
            let (parent2, transform2) = parents[i + 1];

            // Arithmetic crossover for color groups
            let alpha = rng.gen_range(0.0..=1.0);
            let child_color_group = (alpha * parent1.color_group as f32
                + (1.0 - alpha) * parent2.color_group as f32)
                .round() as u8;

            let child_position = (transform1.translation + transform2.translation) / 2.0;

            new_cubes.push((child_color_group, child_position));
        }
    }

    // Spawn new cubes
    for (color_group, position) in new_cubes {
        let material = generate_material(color_group, &mut materials);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material,
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        })
        .insert(CubeAttributes { color_group })
        .insert(Mover {
            velocity: Vec3::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)),
        });
    }
}
