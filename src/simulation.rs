use bevy::prelude::Color;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::time::Timer;
use rand::Rng;

const POPULATION_SIZE:usize = 350;
const MUTATION_RATE :f32 = 0.001;

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
            .add_systems(Update, (move_cubes,despawn_cubes));
    }
}

#[derive(Component,Debug)]
struct ParentCube{
    position:Vec3,
    color_group: u8
}
#[derive(Component,Debug)]
struct ChildCube;
#[derive(Component)]
struct Lifetime {
    timer: Timer,
}
#[derive(Resource)]
pub struct GenerationNumber{
    pub current_gen: u32,
}
// A custom component for the color group
#[derive(Component,Debug, Clone, Copy)]
pub struct ColorGroup(pub u8);


impl Default for GenerationNumber{
    fn default() -> Self{
        GenerationNumber{ current_gen: 1}
    } 
}


pub fn process_generation_system(
    mut commands: Commands,
    query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>, Option<&Transform>, Option<&ColorGroup>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut generate_counter: ResMut<GenerationNumber>,
) {
    process_generation(&mut commands, query, &mut materials, &mut meshes, &mut generate_counter, 0.1);
}

fn process_generation(
    commands: &mut Commands,
    mut query: Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>, Option<&Transform>, Option<&ColorGroup>)>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    generate_counter: &mut ResMut<GenerationNumber>,
    base_mutation_rate: f32, 
) {
    // 1. Determine parent entities and relationshipsa
    let (parent_entities, parent_to_children) = determine_parents(&query);
    // 2. Perform crossover and mutation to create child genes
    let new_cubes = perform_crossover(&parent_entities, &query, MUTATION_RATE, generate_counter);
    // 3. Spawn the new generation of cubes
    spawn_child_cubes(commands, materials, meshes, new_cubes);

    // 4. Increment generation counter
    generate_counter.current_gen += 1;

    // re-evaluate_fitness
    evaluate_fitness(commands, query, materials);

}


fn spawn_first_gen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..POPULATION_SIZE {
        // Spawn the parent cube
        let random_x = rng.gen_range(-3.0..7.0);
        let random_y = rng.gen_range(-5.0..7.0);
        let random_z = rng.gen_range(-3.0..7.0);

        let velocity = Vec3::new(
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
        );

        let parent_position = Vec3::new(random_x, random_y, random_z);

        // Generate the material for the parent (cube 1)
        let parent_color_group = rng.gen_range(0..6); // Random color group for the parent cube
        let parent_material = generate_material(parent_color_group, &mut materials);

        // Spawn the parent cube with Lifetime
        let parent_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: parent_material,
                transform: Transform {
                    translation: parent_position,
                    scale: Vec3::splat(0.3),
                    ..default()
                },
                ..default()
            })
            .insert(Mover { velocity })
            .insert(ParentCube {
                position: parent_position,
                color_group: parent_color_group,
            })
            .insert(ColorGroup(parent_color_group))
            .id(); // Save the entity ID to use as a parent

        // Spawn the specified number of child cubes
        for i in 0..5 {
            // Generate a random color group for each child
            let child_color_group = rng.gen_range(0..5);
            let child_material = generate_material(child_color_group, &mut materials);

            let offset = Vec3::new(i as f32 * 1.0, 0.0, 0.0); // Apply a 1.5 unit offset to each child

            let child_entity = commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: child_material,
                    transform: Transform {
                        translation: Vec3::new(1.0, 0.0, 0.0) + offset, // Apply offset to parent position
                        scale: Vec3::splat(0.5),
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
    
    let speed_multiplier = 3.0;

    for (mut mover, mut transform) in query.iter_mut() {
        let scaled_velocity = mover.velocity * speed_multiplier;
        // Update position based on velocity
        transform.translation += scaled_velocity * time.delta_seconds();

        // Check for boundary collisions and reverse velocity if necessary
        if transform.translation.x <= -9.0 || transform.translation.x >= 9.0 {
            mover.velocity.x = -mover.velocity.x; // Reverse X velocity
            transform.translation.x = transform.translation.x.clamp(-9.0, 9.0); // Ensure within bounds
        }
        if transform.translation.y <= -9.0 || transform.translation.y >= 9.0 {
            mover.velocity.y = -mover.velocity.y; // Reverse Y velocity
            transform.translation.y = transform.translation.y.clamp(-9.0, 9.0); // Ensure within bounds
        }
        if transform.translation.z <= -9.0 || transform.translation.z >= 9.0 {
            mover.velocity.z = -mover.velocity.z; // Reverse Z velocity
            transform.translation.z = transform.translation.z.clamp(-9.0, 9.0); // Ensure within bounds
        }
    }
}

//Helper function to compare colors with a tolerance
fn colors_are_equal(color1: Color, color2: Color, tolerance: f32) -> bool {
    let [r1, g1, b1, _] = color1.to_linear().to_u8_array();
    let [r2, g2, b2, _] = color2.to_linear().to_u8_array();

    // Convert to f32 values for comparison
    let r1 = r1 as f32 / 255.0;
    let g1 = g1 as f32 / 255.0;
    let b1 = b1 as f32 / 255.0;

    let r2 = r2 as f32 / 255.0;
    let g2 = g2 as f32 / 255.0;
    let b2 = b2 as f32 / 255.0;

    // Compare each component's difference against the tolerance
    (r1 - r2).abs() < tolerance
        && (g1 - g2).abs() < tolerance
        && (b1 - b2).abs() < tolerance
}

pub fn evaluate_fitness(
    commands: &mut Commands,
    mut query: Query<(
        Entity,
        &Handle<StandardMaterial>,
        Option<&Parent>,
        Option<&Children>,
        Option<&Transform>,
        Option<&ColorGroup>,
    )>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let yellow = Color::srgb(1.0, 1.0, 0.0); // Yellow color definition
    let tolerance = 0.01; // Tolerance for color comparison

    for (entity, material_handle, parent, children, _, color_group) in query.iter() {
        // Get the color of the entity
        let is_entity_yellow = if let Some(material) = materials.get(material_handle) {
            colors_are_equal(material.base_color, yellow, tolerance)
        } else {
            false
        };

        // Calculate fitness score for the cube based on its distance to yellow
        let fitness_score = if let Some(material) = materials.get(material_handle) {
            calculate_fitness_score(material.base_color, yellow)
        } else {
            0.0
        };

        // Handle parent-child logic
        if let Some(parent) = parent {
            // This is a child entity
            if let Ok((_, parent_material_handle, _, _, _, _)) = query.get(parent.get()) {
                let parent_fitness_score = if let Some(parent_material) = materials.get(parent_material_handle) {
                    calculate_fitness_score(parent_material.base_color, yellow)
                } else {
                    0.0
                };

                // If neither parent nor child is yellow and has low fitness, despawn child
                if !is_entity_yellow && fitness_score < 0.9 {
                    commands.entity(entity).despawn_recursive(); // Eliminate weaker non-yellow cubes
                }
            }

        } else if let Some(children) = children {
            // This is a parent entity
            let mut has_yellow_child = false;

            for &child in children.iter() {
                if let Ok((_, child_material_handle, _, _, _, _)) = query.get(child) {
                    if let Some(child_material) = materials.get(child_material_handle) {
                        if colors_are_equal(child_material.base_color, yellow, tolerance) {
                            has_yellow_child = true;
                            break;
                        }
                    }
                }
            }

            // Despawn parent and children if no yellow exists or fitness is low
            if !is_entity_yellow && !has_yellow_child && fitness_score < 0.2 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Helper function to calculate fitness score based on color distance
fn calculate_fitness_score(color: Color, target: Color) -> f32 {
    let [r1, g1, b1, _] = color.to_linear().to_u8_array(); // Decompose color1 into components
    let [r2, g2, b2, _] = target.to_linear().to_u8_array(); // Decompose color2 into components

    // Convert to f32 values for comparison
    let r1 = r1 as f32 / 255.0;
    let g1 = g1 as f32 / 255.0;
    let b1 = b1 as f32 / 255.0;

    let r2 = r2 as f32 / 255.0;
    let g2 = g2 as f32 / 255.0;
    let b2 = b2 as f32 / 255.0;

    // Calculate Euclidean distance between colors
    let distance = ((r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2)).sqrt();

    // The closer to yellow (0 distance), the higher the fitness score
    if distance < 0.1 {
        2.0 // Perfect match to yellow
    } else {
        (1.0 / (1.0 + distance)).powi(3)
    }
}

fn determine_parents(
    query: &Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>, Option<&Transform>, Option<&ColorGroup>)>,
) -> (Vec<Entity>, HashMap<Entity, Vec<Entity>>) {
    let mut parent_to_children: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut parent_entities: Vec<Entity> = Vec::new();

    for (entity, _, parent, children, _, _) in query.iter() {
        if let Some(parent) = parent {
            parent_to_children.entry(parent.get()).or_insert_with(Vec::new).push(entity);
        } else {
            parent_entities.push(entity);
        }

        if let Some(children) = children {
            for &child in children.iter() {
                parent_to_children.entry(entity).or_insert_with(Vec::new).push(child);
            }
        }
    }

    (parent_entities, parent_to_children)
}


fn perform_crossover(
    parent_entities: &[Entity],
    query: &Query<(Entity, &Handle<StandardMaterial>, Option<&Parent>, Option<&Children>, Option<&Transform>, Option<&ColorGroup>)>,
    mutation_rate: f32,
    generate_counter: &mut ResMut<GenerationNumber>,
) -> Vec<(u8, Vec3)> {

    let mut rng = rand::thread_rng();
    let mut new_cubes: Vec<(u8, Vec3)> = Vec::new();

    for i in (0..parent_entities.len()).step_by(2) {
        if i + 1 < parent_entities.len() {
            let parent1 = parent_entities[i];
            let parent2 = parent_entities[i + 1];

            if let (
                Ok((_, _, _, _, Some(transform1), Some(color_group1))),
                Ok((_, _, _, _, Some(transform2), Some(color_group2))),
            ) = (query.get(parent1), query.get(parent2))
            {
                // Gene slicing using arithmetic crossover
                let slice_point = rng.gen_range(0.0..=1.0);
                // Sliced color group
                let child_color_group = if color_group1.0 == 3 || color_group2.0 == 3 {
                    3 // Prioritize yellow parents
                } else {
                    if slice_point < 0.5 {
                        color_group1.0
                    } else {
                        color_group2.0
                    }
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
                    mutate(child_position, child_color_group, mutation_rate,generate_counter );

                // Add new cube data
                new_cubes.push((mutated_color_group, mutated_position));
            }
        }
    }

    new_cubes
}

fn spawn_child_cubes(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    new_cubes: Vec<(u8, Vec3)>,
) {
    // Precreate the mesh to avoid recreating it every time
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    // Spawn the parent entity
    let parent_entity = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            Name::new("ParentEntity"),
        ))
        .id();

    // Iterate over each new cube spawn request
    for (color_group, position) in new_cubes {
        // Generate or reuse the material based on the color group
        let child_material = generate_material(color_group, materials);

        // Spawn a new cube as a child of the parent entity
        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(), // Reuse the mesh
                    material: child_material,
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(0.4), // Set the scale for the cubes
                        ..default()
                    },
                    ..default()
                },
                Lifetime {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                },
            ));
        });
    }
}

fn despawn_cubes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        // Tick the timer
        lifetime.timer.tick(time.delta());

        // Despawn the entity if the timer has finished
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn mutate(position: Vec3, color_group: u8, mutation_rate: f32, generate_counter: &mut ResMut<GenerationNumber>) -> (Vec3, u8) {
    let mut rng = rand::thread_rng();
    let generation = generate_counter.current_gen;

    let yellow_bias = 0.0005 * generation as f32; 
    let mutation_probability = (mutation_rate + yellow_bias).min(1.0);

    // Mutate position: add small random changes within a range
    let mutated_position = Vec3::new(
        position.x + rng.gen_range(-mutation_rate..mutation_rate),
        position.y + rng.gen_range(-mutation_rate..mutation_rate),
        position.z + rng.gen_range(-mutation_rate..mutation_rate),
    );

    // Mutate color group: increase chance of yellow (color_group = 3) over time
    let mutated_color_group = if rng.gen_bool(mutation_probability as f64) {
        3  // Prefer yellow as generations increase
    } else {
        color_group
    };

    (mutated_position, mutated_color_group)
}

