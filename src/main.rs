use std::thread;
use std::time;

use ::rand;
use macroquad::miniquad::window;
use macroquad::prelude::*;
use particle::Particle;
use vector::Vector2;

mod particle;
mod vector;

const FPS: f64 = 25.;

const PARTICLE_COUNT: u32 = 1200;
const RADIUS: u32 = 5;
const MASS: f64 = 1.;

const REPULSIVENESS: f64 = 1.5;
const REPULSE_RADIUS: u32 = 32;

const MOUSE_RADIUS: f64 = 128.;
const MOUSE_STRENGTH: f64 = 18.;

#[macroquad::main("Particles")]
async fn main() {
    let screen_size = window::screen_size();

    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..PARTICLE_COUNT {
        let x =
            (rand::random::<f64>() * (screen_size.0 - (RADIUS * 2) as f32) as f64) + RADIUS as f64;
        let y =
            (rand::random::<f64>() * (screen_size.1 - (RADIUS * 2) as f32) as f64) + RADIUS as f64;

        particles.push(Particle::new(
            Vector2::from_components(x, y),
            RADIUS,
            MASS,
            (255, 255, 255, 255),
        ));
    }

    let sleep_zero = time::Duration::from_secs(0);
    let mut last_frame: time::Instant = time::Instant::now();

    loop {
        let delta_frame_time = time::Instant::now()
            .duration_since(last_frame)
            .as_secs_f64();

        last_frame = time::Instant::now();

        // calculate
        for i in 0..particles.len() {
            particles[i].calculate(&delta_frame_time);

            for ii in 0..particles.len() {
                if i == ii {
                    continue;
                }

                let distance_vector = particles[ii]
                    .position
                    .get_distance_vector(&particles[i].position);
                let distance = distance_vector.get_magnitude();
                if distance as u32 <= REPULSE_RADIUS {
                    let strength = (REPULSE_RADIUS as f64 - distance) / REPULSE_RADIUS as f64;
                    let mut repulse_vector = distance_vector.reverse_vector();
                    repulse_vector.set_magnitude(
                        strength * REPULSIVENESS * particles[i].mass * delta_frame_time,
                    );
                    particles[i].velocity.add(&repulse_vector);
                }
            }
        }

        // fun input
        if is_mouse_button_down(MouseButton::Left) {
            // slice
            let mouse_pos = mouse_position();
            for i in 0..particles.len() {
                let mut distance_vector =
                    particles[i]
                        .position
                        .get_distance_vector(&Vector2::from_components(
                            mouse_pos.0 as f64,
                            mouse_pos.1 as f64,
                        ));
                let distance = distance_vector.get_magnitude();
                if distance <= MOUSE_RADIUS {
                    let strength = (MOUSE_RADIUS - distance) / MOUSE_RADIUS;
                    distance_vector.set_magnitude(strength * MOUSE_STRENGTH * delta_frame_time);
                    particles[i].velocity.add(&distance_vector);
                }
            }
        } else if is_mouse_button_down(MouseButton::Right) {
            // pull
            let mouse_pos = mouse_position();
            for i in 0..particles.len() {
                let mut distance_vector =
                    particles[i]
                        .position
                        .get_distance_vector(&Vector2::from_components(
                            mouse_pos.0 as f64,
                            mouse_pos.1 as f64,
                        ));
                let distance = distance_vector.get_magnitude();
                if distance <= MOUSE_RADIUS {
                    let strength = (MOUSE_RADIUS - distance) / MOUSE_RADIUS;
                    distance_vector.reverse();
                    distance_vector.set_magnitude(strength * MOUSE_STRENGTH * delta_frame_time);
                    particles[i].velocity.add(&distance_vector);
                }
            }
        }

        // draw
        clear_background(BLACK);

        for i in 0..particles.len() {
            particles[i].draw();
        }

        next_frame().await;

        let fps: f64 = 1. / delta_frame_time;
        println!("FPS: {}", fps.round());

        // let sleep_until = time::Instant::now()
        //     + time::Duration::from_secs_f64(((1. / FPS) - delta_frame_time).max(0.));
        while time::Instant::now()
            .duration_since(last_frame)
            .as_secs_f64()
            <= 1. / FPS
        {
            thread::sleep(sleep_zero);
        }
    }
}
