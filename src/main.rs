use std::thread;
use std::time;

use ::rand;
use macroquad::miniquad::window;
use macroquad::prelude::*;
use particle::Particle;
use vector::Vector2;

mod particle;
mod vector;

const PARTICLE_COUNT: u32 = 200;
const RADIUS: u32 = 15;
const MASS: f64 = 1.;

const REPULSIVENESS: f64 = 2.;
const REPULSE_RADIUS: u32 = 32;
const ATTRACTION_RADIUS: u32 = 45;

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
        let time_since_last_frame = time::Instant::now()
            .duration_since(last_frame)
            .as_secs_f32();
        let fps: f32 = 1. / time_since_last_frame;
        last_frame = time::Instant::now();

        let sleep_until = time::Instant::now() + time::Duration::from_secs_f64(1. / 25.);
        while time::Instant::now() < sleep_until {
            thread::sleep(sleep_zero);
        }

        println!("FPS: {}", fps);

        // calculate
        for i in 0..particles.len() {
            particles[i].calculate();

            for ii in 0..particles.len() {
                if i == ii {
                    continue;
                }

                let mut distance_vector = particles[ii]
                    .position
                    .get_distance_vector(&particles[i].position);
                let distance = distance_vector.get_magnitude();
                if distance as u32 <= REPULSE_RADIUS {
                    let strength = (REPULSE_RADIUS as f64 - distance) / REPULSE_RADIUS as f64;
                    distance_vector.reverse();
                    distance_vector.set_magnitude(strength * REPULSIVENESS * particles[i].mass);
                    particles[i].velocity.add(&distance_vector);
                }
            }
        }

        // fun input
        const MOUSE_RADIUS: f64 = 128.;
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
                    distance_vector.set_magnitude(strength * 2.);
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
                    distance_vector.set_magnitude(strength * 2.);
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
    }
}
