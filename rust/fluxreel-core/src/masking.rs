use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use nalgebra::Point2;

#[derive(Debug, Clone)]
pub struct BezierPoint {
    pub anchor: Point2<f32>,
    pub handle_in: Point2<f32>,
    pub handle_out: Point2<f32>,
}

#[pyclass]
#[derive(Clone)]
pub struct Mask {
    pub points: Vec<BezierPoint>,
    #[pyo3(get, set)]
    pub feather: f32,
    #[pyo3(get, set)]
    pub expansion: f32,
    #[pyo3(get, set)]
    pub inverted: bool,
    #[pyo3(get, set)]
    pub opacity: f32,
}

#[pymethods]
impl Mask {
    #[new]
    fn new() -> Self {
        Self {
            points: Vec::new(),
            feather: 0.0,
            expansion: 0.0,
            inverted: false,
            opacity: 1.0,
        }
    }

    fn add_point(&mut self, x: f32, y: f32) {
        let point = BezierPoint {
            anchor: Point2::new(x, y),
            handle_in: Point2::new(x - 10.0, y),
            handle_out: Point2::new(x + 10.0, y),
        };
        self.points.push(point);
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        // Simplified point-in-polygon test
        // Full implementation would use bezier curve math
        false
    }
}

#[pyclass]
#[derive(Clone)]
pub struct RectangleMask {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub width: f32,
    #[pyo3(get, set)]
    pub height: f32,
    #[pyo3(get, set)]
    pub feather: f32,
    #[pyo3(get, set)]
    pub inverted: bool,
}

#[pymethods]
impl RectangleMask {
    #[new]
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            feather: 0.0,
            inverted: false,
        }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        let inside = px >= self.x && px <= self.x + self.width &&
                     py >= self.y && py <= self.y + self.height;
        if self.inverted {
            !inside
        } else {
            inside
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct EllipseMask {
    #[pyo3(get, set)]
    pub center_x: f32,
    #[pyo3(get, set)]
    pub center_y: f32,
    #[pyo3(get, set)]
    pub radius_x: f32,
    #[pyo3(get, set)]
    pub radius_y: f32,
    #[pyo3(get, set)]
    pub feather: f32,
    #[pyo3(get, set)]
    pub inverted: bool,
}

#[pymethods]
impl EllipseMask {
    #[new]
    fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center_x,
            center_y,
            radius_x,
            radius_y,
            feather: 0.0,
            inverted: false,
        }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        let dx = (px - self.center_x) / self.radius_x;
        let dy = (py - self.center_y) / self.radius_y;
        let inside = (dx * dx + dy * dy) <= 1.0;
        if self.inverted {
            !inside
        } else {
            inside
        }
    }
}

