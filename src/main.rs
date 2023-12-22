use std::thread;
use std::time;

use ::rand;
use macroquad::miniquad::window;
use macroquad::prelude::*;
use particle::Particle;
use rand::Rng;
use vector::Vector2;

mod particle;
mod vector;

const PARTICLE_COUNT: u32 = 100;
const RADIUS: u32 = 15;

#[macroquad::main("Particles")]
async fn main() {
    let screen_size = window::screen_size();

    let mut particles: Vec<particle::Particle> = Vec::new();
    for _ in 0..PARTICLE_COUNT {
        let x =
            (rand::random::<f64>() * (screen_size.0 - (RADIUS * 2) as f32) as f64) + RADIUS as f64;
        let y =
            (rand::random::<f64>() * (screen_size.1 - (RADIUS * 2) as f32) as f64) + RADIUS as f64;

        particles.push(Particle::new(
            Vector2::from_components(x, y),
            RADIUS,
            1.0,
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
        }

        // fun input
        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();

            let mut is_empty = true;
            let mut particle_i = 1;
            for i in 0..=particles.len() - 1 {
                if (particles[i].position.get_x() - particles[i].radius as f64)
                    <= mouse_pos.0 as f64
                    && (particles[i].position.get_y() - particles[i].radius as f64)
                        <= mouse_pos.1 as f64
                    && (particles[i].position.get_x() + particles[i].radius as f64)
                        >= mouse_pos.0 as f64
                    && (particles[i].position.get_y() + particles[i].radius as f64)
                        >= mouse_pos.1 as f64
                {
                    is_empty = false;
                    particle_i = i;
                }
            }
            if is_empty {
                particles.push(Particle::new(
                    Vector2::from_components(mouse_pos.0 as f64, mouse_pos.1 as f64),
                    15,
                    1.,
                    (255, 255, 255, 255),
                ))
            } else {
                particles.remove(particle_i);
            }
        } else if is_key_pressed(KeyCode::Space) {
            let mut i = rand::thread_rng();
            let i = i.gen_range(0..=particles.len() - 1);

            particles[i]
                .velocity
                .add(&Vector2::from_components(0., -10.));
        }

        // draw
        clear_background(BLACK);

        for i in 0..particles.len() {
            particles[i].draw();
        }

        next_frame().await;
    }
}
