extern crate piston_window;

use crate::{vector::Vec3, EPSILON};
use rand::Rng;

use piston_window::*;

const MAX_SPEED: f64 = 2.0;
const MAX_FORCE: f64 = 0.1;
const RADIUS: f64 = 50.0;
const MASS: f64 = 1.0;

const ALIGNMENT_WEIGHT: f64 = 1.0;
const COHESION_WEIGHT: f64 = 1.0;
const SEPARATION_WEIGHT: f64 = 1.0;

#[derive(Clone, Copy)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_speed: f64,
    pub max_force: f64,
    pub mass: f64,
    pub radius: f64,
}

impl Boid {
    pub fn new(width: f64, height: f64) -> Boid {
        let mut rng = rand::thread_rng();
        let position = Vec3::new(
            rng.gen_range((0.0)..width),
            rng.gen_range((0.0)..height),
            0.0,
        );
        let velocity = Vec3::new(EPSILON, 0.0, 0.0);
        let acceleration = Vec3::zero();
        let max_speed = MAX_SPEED;
        let max_force = MAX_FORCE;
        let mass = MASS;
        let radius = RADIUS;
        Boid {
            position,
            velocity,
            acceleration,
            max_speed,
            max_force,
            mass,
            radius,
        }
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d, col: Option<[f32; 4]>) {
        let color = col.unwrap_or([1.0; 4]);
        ellipse(
            color,
            [self.position.x, self.position.y, 5.0, 5.0],
            c.transform,
            g,
        );
    }

    pub fn r#move(&mut self, width: f64, height: f64) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity.limit(self.max_speed);
        self.acceleration *= 0.0;

        if self.position.x > width {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = width;
        }
        if self.position.y > height {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = height;
        }
    }

    pub fn get_forces(&mut self, boids: &Vec<Boid>) -> (Vec3, Vec3, Vec3) {
        let mut separation = Vec3::zero();
        let mut alignment = Vec3::zero();
        let mut cohesion = Vec3::zero();
        let mut total = 0;

        for b in boids.iter() {
            let d = self.position.dist(b.position);

            if b.position != self.position && d < self.radius {
                alignment += b.velocity;
                cohesion += b.position;

                let diff = (self.position - b.position) / (1.0 + d * d);
                separation += diff;

                total += 1;
            }
        }

        if total >= 1 {
            alignment /= total as f64;
            alignment.normalize();
            alignment *= self.max_speed;
            alignment -= self.velocity;
            alignment.limit(self.max_force);

            cohesion /= total as f64;
            cohesion -= self.position;
            cohesion.normalize();
            cohesion *= self.max_speed;
            cohesion -= self.velocity;
            cohesion.limit(self.max_force);

            separation /= total as f64;
            separation.normalize();
            separation *= self.max_speed;
            separation -= self.velocity;
            separation.limit(self.max_force);
        }
        (alignment, cohesion, separation)
    }

    pub fn flock(&mut self, others: &Vec<Boid>) {
        let forces = self.get_forces(others);
        let alignment = forces.0 * ALIGNMENT_WEIGHT;
        let cohesion = forces.1 * COHESION_WEIGHT;
        let separation = forces.2 * SEPARATION_WEIGHT;
        self.acceleration += alignment + cohesion + separation;
    }
}
