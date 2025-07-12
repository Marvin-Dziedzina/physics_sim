use bevy::prelude::*;

use rand::{self, Rng};

const PARTICLE_COUNT: u32 = 10000;

const DRAG_MULTIPLIER: f32 = 0.9;

const PARTICLE_RADIUS: f32 = 5.0;
const PARTICLE_DIAMETER: f32 = PARTICLE_RADIUS * 2.0;
const PARTICLE_DIAMETER_SQUARED: f32 = PARTICLE_DIAMETER * PARTICLE_DIAMETER;
const MASS: f32 = 0.05;
const BOUNCE_VELOCITY_MULTIPLIER: f32 = 0.6;
const REPULSIVENESS: f32 = 0.49;

const GRAVITY: f32 = 9.81;

#[derive(Debug, Component)]
struct Particle;

#[derive(Debug, Component)]
struct Velocity(Vec2);

#[derive(Debug, Component)]
struct Mass(f32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_gravity, apply_drag, repulse_particles))
        .add_systems(
            FixedPostUpdate,
            (
                update_particle_position,
                check_border_collision.after(update_particle_position),
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&mut Window>,
) {
    commands.spawn(Camera2d);

    let mut thread_rng = rand::rng();

    let window_size = window.size();

    let circle = meshes.add(Circle::new(PARTICLE_RADIUS));
    let color = materials.add(Color::linear_rgb(1.0, 1.0, 1.0));

    let x_range = -(window_size.x / 2.0) + PARTICLE_RADIUS..(window_size.x / 2.0) - PARTICLE_RADIUS;
    let y_range = -(window_size.y / 2.0) + PARTICLE_RADIUS..(window_size.y / 2.0) - PARTICLE_RADIUS;

    for _ in 0..PARTICLE_COUNT {
        commands.spawn((
            Particle,
            Transform::from_xyz(
                thread_rng.random_range(x_range.clone()),
                thread_rng.random_range(y_range.clone()),
                0.0,
            ),
            Velocity(Vec2::new(0.0, 0.0)),
            Mesh2d(circle.clone()),
            MeshMaterial2d(color.clone()),
            Mass(MASS),
        ));
    }
}

fn check_border_collision(
    window: Single<&mut Window>,
    mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    let window_size = window.size();
    let coordinate_window_size = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);

    for (mut particle_transform, mut particle_velocity) in particles.iter_mut() {
        if particle_transform.translation.x - PARTICLE_RADIUS < -coordinate_window_size.x {
            particle_transform.translation.x = (coordinate_window_size.x - PARTICLE_RADIUS) * -1.0;

            particle_velocity.0.x = particle_velocity.0.x * BOUNCE_VELOCITY_MULTIPLIER * -1.0;
        } else if particle_transform.translation.x + PARTICLE_RADIUS > coordinate_window_size.x {
            particle_transform.translation.x = coordinate_window_size.x - PARTICLE_RADIUS;

            particle_velocity.0.x = particle_velocity.0.x * BOUNCE_VELOCITY_MULTIPLIER * -1.0;
        };

        if particle_transform.translation.y - PARTICLE_RADIUS < -coordinate_window_size.y {
            particle_transform.translation.y = (coordinate_window_size.y - PARTICLE_RADIUS) * -1.0;

            particle_velocity.0.y = particle_velocity.0.y * BOUNCE_VELOCITY_MULTIPLIER * -1.0;
        } else if particle_transform.translation.y + PARTICLE_RADIUS > coordinate_window_size.y {
            particle_transform.translation.y = coordinate_window_size.y - PARTICLE_RADIUS;

            particle_velocity.0.y = particle_velocity.0.y * BOUNCE_VELOCITY_MULTIPLIER * -1.0;
        };
    }
}

fn update_particle_position(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    particles
        .par_iter_mut()
        .for_each(|(mut particle_transform, particle_velocity)| {
            particle_transform.translation.x += particle_velocity.0.x * time.delta_secs();
            particle_transform.translation.y += particle_velocity.0.y * time.delta_secs();
        });
}

fn apply_gravity(mut particles: Query<(&mut Velocity, &Mass), With<Particle>>) {
    for (mut velocity, mass) in particles.iter_mut() {
        velocity.0.y -= GRAVITY * mass.0;
    }
}

fn apply_drag(mut particles: Query<&mut Velocity, With<Particle>>) {
    for mut velocity in particles.iter_mut() {
        velocity.0.x *= DRAG_MULTIPLIER;
        velocity.0.y *= DRAG_MULTIPLIER;
    }
}

fn repulse_particles(
    mut query_1: Query<(&Transform, &mut Velocity), With<Particle>>,
    query_2: Query<&Transform, With<Particle>>,
) {
    query_1
        .par_iter_mut()
        .for_each(|(particle_1_transform, mut particle_1_velocity)| {
            for particle_2_transform in query_2.iter() {
                let direction = particle_1_transform.translation - particle_2_transform.translation;
                let distance_squared = direction.length_squared();

                if distance_squared > PARTICLE_DIAMETER_SQUARED {
                    continue;
                };

                let distance = distance_squared.sqrt().max(0.0000001);
                let normalized_direction = direction / distance;
                particle_1_velocity.0 = particle_1_velocity.0
                    + Vec2::new(normalized_direction.x, normalized_direction.y)
                        * (PARTICLE_DIAMETER - distance)
                        * REPULSIVENESS;
            }
        });
}
