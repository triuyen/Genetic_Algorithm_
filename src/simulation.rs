use bevy::prelude::Color;
use bevy::{color, prelude::*};
use std::time::Duration;
use std::collections::HashMap;
pub mod data_genom;
use rand::Rng;

// let's determine that we want at the end of the simulation yellow cubes sets only

const POPULATION_SIZE:usize = 25;
// const TARGET:usize = 80;
// const MAX_GENERATIONS:usize = 1000;

// All components
#[derive(Component)]
struct Mover {
    velocity: Vec3,
}
pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenerationNumber { current_gen: 1 })
            .add_systems(Startup, spawn_first_gen)
            //.add_systems(Update, run_simulation)
            .add_systems(Update, evaluate_fitness)
            .add_systems(Update, move_cubes)
            .add_systems(Update, selective_reproduction);
            //.add_systems(Update, get_generation_num);
    }
}

#[derive(Component,Debug)]
struct ParentCube;
#[derive(Component,Debug)]
struct ChildCube;

#[derive(Component,Debug)]
pub struct CubeAttributes {
    color_group: u8, // Attribute to determine the color group
}

#[derive(Resource)]
struct GenerationNumber{
    current_gen: usize,
}

impl Default for GenerationNumber{
    fn default() -> Self{
        GenerationNumber{ current_gen: 1}
    } 
}

#[derive(Resource)]
struct SimulationState {
    running: bool,
    max_generations: usize,
    current_generation: usize,
}

impl Default for SimulationState {
    fn default() -> Self {
        SimulationState {
            running: true,
            max_generations: 10,
            current_generation: 1,
        }
    }
}

// fn run_simulation(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut generation_number: ResMut<GenerationNumber>,
//     mut query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>, Option<&Transform>, Option<&ColorGroup>)>,
// ) {
//     const MAX_GENERATIONS: usize = 30;

//     while generation_number.current_gen <= MAX_GENERATIONS {
//         println!("Running Generation {}", generation_number.current_gen);

//         // Step 1: Evaluate Fitness 
//         let mut value = evaluate_fitness(&mut commands, &mut query, &mut materials);
//         println!("Fitness evaluation completed for Generation {}", generation_number.current_gen);
//         value;

//         // Step 2: Perform Selective Reproduction
//         let mut value = selective_reproduction(
//             &mut commands,
//             &mut query,
//             &mut materials,
//             &mut meshes,
//          &mut generation_number,
//         );

//         value;
//         println!("Selective reproduction completed for Generation {}", generation_number.current_gen);

//         // Increment generation counter
//         generation_number.current_gen += 1;

//         // std::thread::sleep(Duration::from_secs(2)); // Pauses for 1 second

//         // Break the loop if the termination condition is met
//         if generation_number.current_gen > MAX_GENERATIONS {
//             println!("Simulation completed after {} generations.", MAX_GENERATIONS);
//             break;
//         }
//     }
// }


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
        let parent_color_group = rng.gen_range(0..6); // Random color group for the parent cube
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
            .insert(ColorGroup(parent_color_group))
            .id(); // Save the entity ID to use as a parent
        
        // Spawn the specified number of child cubes
        for i in 0..5 {
            // Generate a random color group for each child
            let child_color_group = rng.gen_range(0..5);
            let child_material = generate_material(child_color_group, &mut materials);
            
            let offset = Vec3::new(i as f32 * 1.0, 0.0, 0.0); // Apply a 1.5 unit offset to each child
            
            let child_entity = commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: child_material,
                transform: Transform {
                    translation: Vec3::new(1.0, 0.0, 0.0) + offset, // Apply offset to parent position
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            })
            .insert(ChildCube)
            .insert(ColorGroup(child_color_group))
            .id();

        // Parent the child to the parent cube
        commands.entity(parent_entity).add_child(child_entity);
    }
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


fn move_cubes(mut query: Query<(&mut Mover, &mut Transform)>, time: Res<Time>) {
    for (mut mover, mut transform) in query.iter_mut() {
        // Update position based on velocity
        transform.translation += mover.velocity * time.delta_seconds();

        // Check for boundary collisions and reverse velocity if necessary
        if transform.translation.x <= -7.0 || transform.translation.x >= 7.0 {
            mover.velocity.x = -mover.velocity.x; // Reverse X velocity
            transform.translation.x = transform.translation.x.clamp(-7.0, 7.0); // Ensure within bounds
        }
        if transform.translation.y <= -7.0 || transform.translation.y >= 7.0 {
            mover.velocity.y = -mover.velocity.y; // Reverse Y velocity
            transform.translation.y = transform.translation.y.clamp(-7.0, 7.0); // Ensure within bounds
        }
        if transform.translation.z <= -7.0 || transform.translation.z >= 7.0 {
            mover.velocity.z = -mover.velocity.z; // Reverse Z velocity
            transform.translation.z = transform.translation.z.clamp(-7.0, 7.0); // Ensure within bounds
        }
    }
}

// evaluate weither the gene is yellow or not __________________________________________________________
pub fn evaluate_fitness(
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<StandardMaterial>, 
                        Option<&Parent>, Option<&Children>, 
                        Option<&Transform>, Option<&ColorGroup>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let yellow = Color::srgb(1.0, 1.0, 0.0); // Yellow color definition

    for (entity, material_handle, parent, children,_,_) in query.iter() {
        // Access material's base color
        let is_entity_yellow = if let Some(material) = materials.get(material_handle) {
            material.base_color == yellow
        } else {
            false
        };

        // Parent-Child logic
        if let Some(parent) = parent {
            // If the entity is a child, handle based on the parent's status
            if let Ok((_, parent_material_handle, _, _,_,_)) = query.get(parent.get()) {
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
                if let Ok((_, child_material_handle, _, _,_,_)) = query.get(child) {
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

// A custom component for the color group
#[derive(Component,Debug, Clone, Copy)]
pub struct ColorGroup(pub u8);

// TODO: the problem is not the looping it's inside the selective reproduction function
// Perform reproduction using arithmetic crossover __________________________________________________________
pub fn selective_reproduction(
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<StandardMaterial>, 
                        Option<&Parent>, Option<&Children>, 
                        Option<&Transform>, Option<&ColorGroup>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut generate_counter: ResMut<GenerationNumber>,
) {
    // Map to store parent-child relationships
    let mut parent_to_children: HashMap<Entity, Vec<Entity>> = HashMap::new();
    //Creates an empty vector parent_entities to hold parent entities.
    let mut parent_entities: Vec<Entity> = Vec::new();


    for (entity, _material_handle, parent, children, _,_colorgroup) in query.iter() {
        if let Some(parent) = parent {
            let parent_entity = parent.get(); // Get the parent entity

            // Associate this entity as a child to the parent
            parent_to_children
                .entry(parent_entity)
                .or_insert_with(Vec::new);
        } else {
            // If the entity does not have a parent, it's a potential root (parent entity)
            parent_entities.push(entity);
        }

        if let Some(children) = children {
            for &child in children.iter() {
                // Add this entity to the parent-child map
                parent_to_children
                    .entry(entity)
                    .or_insert_with(Vec::new)
                    .push(child);
            }
        }
    }
    
  
    // Perform crossover for each pair of parents
    let mut rng = rand::thread_rng();
    let mut new_cubes: Vec<(u8, Vec3)> = Vec::new();

    //println!("parent entities  ----- :{:?}",parent_entities);

    for i in (0..parent_entities.len()).step_by(2) {
        if i + 1 < parent_entities.len() {
            let parent1 = parent_entities[i];
            let parent2 = parent_entities[i + 1];

            if let (
                Ok((_, _, _, _, Some(transform1),Some(color_group1))), //, Some(color_group1))),
                Ok((_, _, _, _, Some(transform2),Some(color_group2))),  //, Some(color_group2))),
            ) = (query.get(parent1), query.get(parent2))
            {

                // Gene slicing using arithmetic crossover
                let slice_point = rng.gen_range(0.0..=1.0);
                // Sliced color group
                let child_color_group = if slice_point < 0.5 {
                    color_group1.0
                } else {
                    color_group2.0
                };

                // Sliced position (weighted average based on slice point)
                let child_position = Vec3::new(
                    transform1.translation.x * slice_point
                        + transform2.translation.x * (1.0 - slice_point),
                    transform1.translation.y * slice_point
                        + transform2.translation.y * (1.0 - slice_point),
                    transform1.translation.z * slice_point
                        + transform2.translation.z * (1.0 - slice_point),
                );

                // Apply mutation
                let (mutated_position, mutated_color_group) =
                mutate(child_position, child_color_group, 0.1); // Mutation rate: 10%

                // // Add new cube data
                println!("Child color group : {:?} ", child_color_group);
                println!("Child position : {:?}", child_position);
                new_cubes.push((child_color_group, child_position));
            }
        }
    }

    // Spawn new child entities based on the recomposed genes
    for (color_group, position) in new_cubes {
        // Generate a material based on the color group
        let child_material = generate_material(color_group, &mut materials);

        // Spawn new child cube
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: child_material,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(1.5),
                ..default()
            },
            ..default()
        });
    }   
    generate_counter.current_gen += 1;
    println!("Generation number : {:?}", generate_counter.current_gen);
}



fn mutate(position: Vec3, color_group: u8, mutation_rate: f32) -> (Vec3, u8) {
    let mut rng = rand::thread_rng();
    println!("Mutation rate : {:?}", mutation_rate);    
    // Mutate position: add small random changes within a range
    let mutated_position = Vec3::new(
        position.x + rng.gen_range(-mutation_rate..mutation_rate),
        position.y + rng.gen_range(-mutation_rate..mutation_rate),
        position.z + rng.gen_range(-mutation_rate..mutation_rate),
    );

    // Mutate color group: randomly increment the value with some probability
    let mutated_color_group = if rng.gen_bool(mutation_rate as f64) {
        color_group.wrapping_add(1) // Use wrapping_add to prevent overflow
    } else {
        color_group
    };

    (mutated_position, mutated_color_group)
}

