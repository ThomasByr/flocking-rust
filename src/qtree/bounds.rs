extern crate piston_window;

use super::Boid;

use math_vector::Vector;
use piston_window::*;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Point {
    pub pos: Vector<f64>,
    pub data: Option<Boid>,
}

#[derive(Debug)]
pub struct Rect {
    pub pos: Vector<f64>,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains(&self, point: &Point) -> bool {
        let x = point.pos.x;
        let y = point.pos.y;
        let x_min = self.pos.x - self.width;
        let x_max = self.pos.x + self.width;
        let y_min = self.pos.y - self.height;
        let y_max = self.pos.y + self.height;
        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }

    pub fn intersects_r(&self, other: &Rect) -> bool {
        !(self.pos.x - self.width > other.pos.x + other.width
            || self.pos.x + self.width < other.pos.x - other.width
            || self.pos.y - self.height > other.pos.y + other.height
            || self.pos.y + self.height < other.pos.y - other.height)
    }

    pub fn draw(&self, c: Context, g: &mut G2d, col: Option<[f32; 4]>) {
        let color = col.unwrap_or([1.0; 4]);
        let l0 = [
            self.pos.x - self.width,
            self.pos.y - self.height,
            self.pos.x + self.width,
            self.pos.y - self.height,
        ];
        let l1 = [
            self.pos.x + self.width,
            self.pos.y - self.height,
            self.pos.x + self.width,
            self.pos.y + self.height,
        ];
        let l2 = [
            self.pos.x + self.width,
            self.pos.y + self.height,
            self.pos.x - self.width,
            self.pos.y + self.height,
        ];
        let l3 = [
            self.pos.x - self.width,
            self.pos.y + self.height,
            self.pos.x - self.width,
            self.pos.y - self.height,
        ];
        line(color, 1.0, l0, c.transform, g);
        line(color, 1.0, l1, c.transform, g);
        line(color, 1.0, l2, c.transform, g);
        line(color, 1.0, l3, c.transform, g);
    }
}
