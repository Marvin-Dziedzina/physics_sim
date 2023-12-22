use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::vector::Vector2;

const GRAVITY: f64 = 9.8;
const BOUNCYNESS: f64 = 0.20;

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Vector2,
    radius: u32,
    mass: f64,
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

    pub fn calculate(&mut self, particles: Vec<Particle>) {
        // movement
        self.gravity();

        // update
        self.update_position();

        // check for rules
        self.check_particle_collisions(&particles);
        self.check_boundaries();
    }

    fn update_position(&mut self) {
        self.position.add(&self.velocity);
    }

    fn gravity(&mut self) {
        self.velocity.add(&Vector2::from_components(
            0.,
            self.mass * GRAVITY * get_frame_time() as f64,
        ));
    }

    fn check_boundaries(&mut self) {
        let screen_size = screen_size();
        let cmpnt = self.position.get_components();

        // check x
        if (cmpnt.x - self.radius as f64) < 0. {
            self.position.set_x(self.radius as f64);
            self.velocity.multiply_scalar_x(BOUNCYNESS * -1.);
        } else if (cmpnt.x + self.radius as f64) > screen_size.0 as f64 {
            self.position
                .set_x(screen_size.0 as f64 - self.radius as f64);
            self.velocity.multiply_scalar_x(BOUNCYNESS * -1.);
        }

        // check y
        if (cmpnt.y - self.radius as f64) < 0. {
            self.position.set_y(self.radius as f64);
            self.velocity.multiply_scalar_y(BOUNCYNESS * -1.);
        } else if (cmpnt.y + self.radius as f64) > screen_size.1 as f64 {
            self.position
                .set_y(screen_size.1 as f64 - self.radius as f64);
            self.velocity.multiply_scalar_y(BOUNCYNESS * -1.);
        }
    }

    fn check_particle_collisions(&mut self, particles: &Vec<Particle>) {
        for particle in particles {
            let mut distance_vector = particle.position.subtract_vector(&self.position);
            let distance = distance_vector.get_magnitude();

            if distance < (self.radius + particle.radius) as f64 {
                // stop me
                let offset = (self.radius + particle.radius) as f64 - distance;
                // shorten the vector to the offset so the particle just gets out of the other particle
                distance_vector.set_magnitude(offset);
                // it should not stop so here we preserve the velocity that isnt dirctly directed toward the other particle
                self.velocity.subtract(&distance_vector);
                // we reverse it so it goes away and not into the other particle
                distance_vector.multiply_scalar(-1.);
                //self.position.add(&distance_vector);

                // set own velocity
                //self.velocity.add(&particle.velocity.multiply_scalar_vector(BOUNCYNESS))
            }
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

    #[cfg(debug_assertions)]
    fn draw_velocity(&self) {
        draw_line(
            self.position.get_x() as f32,
            self.position.get_y() as f32,
            (self.position.get_x() + self.velocity.get_x()) as f32,
            (self.position.get_y() + self.velocity.get_y()) as f32,
            2.,
            Color::from_rgba(255, 0, 0, 180),
        )
    }

    fn draw_vector(&self, v: &Vector2) {
        draw_line(
            self.position.get_x() as f32,
            self.position.get_y() as f32,
            (v.get_x() + self.position.get_x()) as f32,
            (v.get_y() + self.position.get_y()) as f32,
            2.,
            Color::from_rgba(0, 0, 255, 180),
        )
    }
}
