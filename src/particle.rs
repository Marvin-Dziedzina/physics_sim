use std::fmt::Display;

use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::vector::Vector2;

const GRAVITY: f64 = 5.;
const BOUNCYNESS: f64 = 0.20;
const DRAG: f64 = 0.075;

const REPULSIVENESS: f64 = 2.5;
const REPULSE_RADIUS: u32 = 16;

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Vector2,
    pub radius: u32,
    pub mass: f64,
    pub velocity: Vector2,
    color: Color,
}
impl Particle {
    pub fn new(position: Vector2, radius: u32, mass: f64, rgba: (u8, u8, u8, u8)) -> Particle {
        Particle {
            position,
            radius,
            mass,
            velocity: Vector2::new(),
            color: Color::from_rgba(rgba.0, rgba.1, rgba.2, rgba.3),
        }
    }

    pub fn calculate(&mut self, delta_frame_time: &f64) {
        // movement
        // self.gravity(delta_frame_time);
        self.drag(&delta_frame_time);

        // update
        self.update_position();
        self.update_color();

        // check for rules
        self.check_boundaries(&delta_frame_time);
        self.calculate_border_repulsiveness(&delta_frame_time);
    }

    fn update_position(&mut self) {
        self.position.add(&self.velocity);
    }

    fn update_color(&mut self) {
        let speed = self.velocity.get_magnitude() / 7.5;
        let r = speed * 255.;
        let b = (1. - speed) * 255.;
        self.color = Color::from_rgba(r as u8, 0, b as u8, 255);
    }

    fn gravity(&mut self, delta_frame_time: f64) {
        self.velocity.add(&Vector2::from_components(
            0.,
            self.mass * GRAVITY * delta_frame_time,
        ));
    }

    fn drag(&mut self, delta_frame_time: &f64) {
        let mut drag_vector = self.velocity.reverse_vector();
        drag_vector
            .set_magnitude(&self.velocity.get_magnitude() * self.mass * DRAG * delta_frame_time);

        self.velocity.add(&drag_vector);
    }

    fn check_boundaries(&mut self, delta_frame_time: &f64) {
        let screen_size = screen_size();
        let cmpnt = self.position.get_components();

        // check x
        if (cmpnt.x - self.radius as f64) < 0. {
            self.position.set_x(self.radius as f64);
            self.velocity
                .multiply_scalar_x(BOUNCYNESS * -1. * delta_frame_time);
        } else if (cmpnt.x + self.radius as f64) > screen_size.0 as f64 {
            self.position
                .set_x(screen_size.0 as f64 - self.radius as f64);
            self.velocity
                .multiply_scalar_x(BOUNCYNESS * -1. * delta_frame_time);
        }

        // check y
        if (cmpnt.y - self.radius as f64) < 0. {
            self.position.set_y(self.radius as f64);
            self.velocity
                .multiply_scalar_y(BOUNCYNESS * -1. * delta_frame_time);
        } else if (cmpnt.y + self.radius as f64) > screen_size.1 as f64 {
            self.position
                .set_y(screen_size.1 as f64 - self.radius as f64);
            self.velocity
                .multiply_scalar_y(BOUNCYNESS * -1. * delta_frame_time);
        }
    }

    fn calculate_border_repulsiveness(&mut self, delta_frame_time: &f64) {
        let screen_size = screen_size();
        let cmpnt = self.position.get_components();

        // x
        let distance_left = Vector2::from_components(self.radius as f64, cmpnt.y)
            .get_distance_scalar(&self.position);
        let distance_right =
            Vector2::from_components(screen_size.0 as f64 - self.radius as f64, cmpnt.y)
                .get_distance_scalar(&self.position);
        if distance_left <= REPULSE_RADIUS as f64 + self.radius as f64 {
            self.velocity.add(&Vector2::from_components(
                REPULSIVENESS * (REPULSE_RADIUS as f64 - distance_left) * delta_frame_time,
                0.,
            ))
        } else if distance_right <= (REPULSE_RADIUS + self.radius) as f64 {
            self.velocity.add(
                &Vector2::from_components(
                    REPULSIVENESS * (REPULSE_RADIUS as f64 - distance_right) * delta_frame_time,
                    0.,
                )
                .reverse_vector(),
            )
        }

        // y
        let distance_top = Vector2::from_components(cmpnt.x, self.radius as f64)
            .get_distance_scalar(&self.position);
        let distance_bottom =
            Vector2::from_components(cmpnt.x, screen_size.1 as f64 - self.radius as f64)
                .get_distance_scalar(&self.position);
        if distance_top <= REPULSE_RADIUS as f64 + self.radius as f64 {
            self.velocity.add(&Vector2::from_components(
                0.,
                REPULSIVENESS * (REPULSE_RADIUS as f64 - distance_top) * delta_frame_time,
            ));
        } else if distance_bottom <= (REPULSE_RADIUS + self.radius) as f64 {
            self.velocity.add(
                &Vector2::from_components(
                    0.,
                    REPULSIVENESS * (REPULSE_RADIUS as f64 - distance_bottom) * delta_frame_time,
                )
                .reverse_vector(),
            )
        }
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.get_x() as f32,
            self.position.get_y() as f32,
            self.radius as f32,
            self.color,
        );
        self.draw_velocity();
    }

    fn draw_velocity(&self) {
        draw_line(
            self.position.get_x() as f32,
            self.position.get_y() as f32,
            (self.position.get_x() + self.velocity.get_x()) as f32,
            (self.position.get_y() + self.velocity.get_y()) as f32,
            2.,
            Color::from_rgba(0, 255, 0, 180),
        )
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: {} velocity: {} radius: {}; mass: {}; color: [{}, {}, {}, {}];",
            self.position,
            self.velocity,
            self.radius,
            self.mass,
            (self.color.r * 255.) as u8,
            (self.color.g * 255.) as u8,
            (self.color.b * 255.) as u8,
            (self.color.a * 255.) as u8,
        )
    }
}
