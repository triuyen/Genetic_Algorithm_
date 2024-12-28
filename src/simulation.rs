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
struct CubeAttributes {
    color_group: u8, // Attribute to determine the color group
}

#[derive(Component)]
struct Mover {
    velocity: Vec3,
}

#[derive(Default)]
struct Individual {
    genes: String,
    fitness: usize,
    age: usize,
    color: Color,
}


pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_gen)
            .add_systems(Update, move_cubes)
            .add_systems(Update, evaluate_fitness);
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


fn evaluate_fitness(
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let yellow = Color::srgb(1.0, 1.0, 0.0); // Yellow color definition

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


// // Evaluation fo fitness // in this case if there is Yellow ? // is it all Yellow ?
// fn evaluate_fitness(
//     mut commands: Commands,
//     mut query: Query<(Entity, &Handle<StandardMaterial>, Option<&ParentCube>, Option<&ChildCube>)>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut yellow_count = 0;
//     let mut total_cubes = 0;
//     let mut is_yellow = false;

//     let yellow = Color::srgb(1.0, 1.0, 0.0); // Yellow color

//     for (entity, materialhandle,parent, children) in query.iter() {
//         total_cubes += 1;

//         // Access the material from the handle
//         if let Some(material) = materials.get(materialhandle)
//         {
//             let color = material.base_color;
//             // Check if the cube's color is close to yellow
//             if color == yellow {
//                 let is_yellow = entity;
//                 yellow_count += 1;
//             }

//             // If the current entity is part of a parent-child relationship
//             if let Some(parent) = parent{
//                 // if parent cube is not yellow and Children is not yellow delete the children
//                 if !is_yellow{
//                     commands.entity(entity).despawn(); // Despawn parent;
//                     for &child in children.iter() {
                        
//                         if &child != is_yellow {
//                             commands.entity(&child).despawn(); // Despawn children;
//                         }
//                     }
//                 }
//             // If this is a child entity (not a parent)
//             else if let Some(child) = children {
//                 // If the child cube is not yellow , delete the child
//                 if !is_yellow {
//                     commands.entity(entity).despawn(); // Despawn child
//                 }
//             }
//         }
//     }

// }
// }





// fn create_individual(
//     mut genes: &str,
// ) -> Individual {
//     Individual {
//         genes: genes.to_string(), // Example genes
//         fitness: 100,                // Initial fitness
//         age: 100,                    // Initial age
//         color: Color::srgb(0.5, 0.2, 0.8), // Example color (purple-like)
//     }
// }

// fn slicing_genes(gene1:&str, gene2:&str) -> String {
//     let mut rng = rand::thread_rng();
//     let slice_point = rng.gen_range(0..gene1.len());
//     let new_gene = gene1[0..slice_point].to_string() + &gene2[slice_point..];
//     return new_gene;
// }

// fn reproduction_Individual(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // gene slicing
//     let current_Individual = create_individual("ABDE");
//     println!(" genes : {}",current_Individual.genes);
//     // Set the position for the cube
//     let position = Vec3::new(0.0, 0.0, 0.0);
//     // Check if the fitness is greater than 80
//     if current_Individual.fitness > 80{
//         // Create a PbrBundle for the cube
//         commands.spawn(PbrBundle {
//             mesh: asset_server.load("models/cube.glb#Mesh0/Primitive0"), // Specify GLTF mesh path
//             material: materials.add(StandardMaterial {
//                 base_color: Color::rgb(0.0, 0.0, 1.0),
//                 ..Default::default()
//             }),
//             transform: Transform {
//                 translation: position,
//                 rotation: Quat::IDENTITY,
//                 scale: Vec3::splat(0.1),
//             },
//         ..Default::default()
//         });
//     }
// }

