use pyo3::prelude::*;
use nalgebra::{Point2, Vector2};

#[pyclass]
#[derive(Clone)]
pub struct Rectangle {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub width: f32,
    #[pyo3(get, set)]
    pub height: f32,
}

#[pymethods]
impl Rectangle {
    #[new]
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Circle {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub radius: f32,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        let dx = px - self.x;
        let dy = py - self.y;
        (dx * dx + dy * dy).sqrt() <= self.radius
    }

    fn intersects(&self, other: &Circle) -> bool {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < (self.radius + other.radius)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Polygon {
    pub points: Vec<Point2<f32>>,
}

#[pymethods]
impl Polygon {
    #[new]
    fn new(points: Vec<(f32, f32)>) -> Self {
        Self {
            points: points.iter().map(|(x, y)| Point2::new(*x, *y)).collect(),
        }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        let point = Point2::new(px, py);
        let mut inside = false;
        let mut j = self.points.len() - 1;
        
        for i in 0..self.points.len() {
            let pi = &self.points[i];
            let pj = &self.points[j];
            
            if ((pi.y > point.y) != (pj.y > point.y))
                && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        
        inside
    }
}

/// Calculate distance between two points
#[pyfunction]
pub fn point_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

/// Calculate angle between two points in degrees
#[pyfunction]
pub fn point_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((y2 - y1).atan2(x2 - x1) * 180.0 / std::f32::consts::PI + 360.0) % 360.0
}

/// Rotate point around center
#[pyfunction]
pub fn rotate_point(px: f32, py: f32, cx: f32, cy: f32, angle_degrees: f32) -> (f32, f32) {
    let angle = angle_degrees * std::f32::consts::PI / 180.0;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let dx = px - cx;
    let dy = py - cy;
    (
        cx + dx * cos_a - dy * sin_a,
        cy + dx * sin_a + dy * cos_a,
    )
}

