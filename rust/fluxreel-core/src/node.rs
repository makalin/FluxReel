use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use nalgebra::{Point2, Vector2};

#[derive(Debug, Clone)]
pub struct NodeProperties {
    pub position: Point2<f32>,
    pub scale: Vector2<f32>,
    pub rotation: f32,
    pub opacity: f32,
    pub visible: bool,
}

impl Default for NodeProperties {
    fn default() -> Self {
        Self {
            position: Point2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            rotation: 0.0,
            opacity: 1.0,
            visible: true,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Node {
    #[pyo3(get, set)]
    pub id: String,
    pub properties: NodeProperties,
}

#[pymethods]
impl Node {
    #[new]
    fn new(id: String) -> Self {
        Self {
            id,
            properties: NodeProperties::default(),
        }
    }

    fn align(&mut self, alignment: &str) {
        // Simplified alignment logic
        match alignment {
            "center" => {
                self.properties.position.x = 0.0;
                self.properties.position.y = 0.0;
            }
            "left" => {
                self.properties.position.x = -0.5;
            }
            "right" => {
                self.properties.position.x = 0.5;
            }
            "top" => {
                self.properties.position.y = 0.5;
            }
            "bottom" => {
                self.properties.position.y = -0.5;
            }
            _ => {}
        }
    }

    fn fade_in(&mut self, _duration: f32) {
        self.properties.opacity = 0.0;
        // Animation would be handled by the renderer
    }

    fn scale(&mut self, start: f32, _end: f32, _ease: &str) {
        self.properties.scale = Vector2::new(start, start);
        // Animation would be handled by the renderer
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.properties.position = Point2::new(x, y);
    }

    fn move_by(&mut self, dx: f32, dy: f32) {
        self.properties.position.x += dx;
        self.properties.position.y += dy;
    }

    fn rotate(&mut self, angle: f32) {
        self.properties.rotation = angle;
    }

    fn rotate_by(&mut self, angle: f32) {
        self.properties.rotation += angle;
    }

    fn set_opacity(&mut self, opacity: f32) {
        self.properties.opacity = opacity.clamp(0.0, 1.0);
    }

    fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        self.properties.scale = Vector2::new(scale_x, scale_y);
    }

    fn set_visible(&mut self, visible: bool) {
        self.properties.visible = visible;
    }

    fn fade_out(&mut self, _duration: f32) {
        self.properties.opacity = 1.0;
        // Animation would be handled by the renderer
    }

    fn slide(&mut self, start_x: f32, start_y: f32, _end_x: f32, _end_y: f32, _ease: &str) {
        self.properties.position = Point2::new(start_x, start_y);
        // Animation would be handled by the renderer
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TextNode {
    #[pyo3(get)]
    pub node: Node,
    #[pyo3(get, set)]
    pub text: String,
    #[pyo3(get, set)]
    pub size: f32,
    #[pyo3(get, set)]
    pub color: String,
    #[pyo3(get, set)]
    pub font: Option<String>,
}

#[pymethods]
impl TextNode {
    #[new]
    fn new(text: String, size: f32, color: String) -> Self {
        Self {
            node: Node::new(format!("text_{}", text.len())),
            text,
            size,
            color,
            font: None,
        }
    }

    fn align(&mut self, alignment: &str) {
        self.node.align(alignment);
    }

    fn fade_in(&mut self, duration: f32) {
        self.node.fade_in(duration);
    }

    fn scale(&mut self, start: f32, end: f32, ease: &str) {
        self.node.scale(start, end, ease);
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ImageNode {
    #[pyo3(get)]
    pub node: Node,
    #[pyo3(get, set)]
    pub path: String,
}

#[pymethods]
impl ImageNode {
    #[new]
    fn new(path: String) -> Self {
        Self {
            node: Node::new(format!("image_{}", path.len())),
            path,
        }
    }

    fn align(&mut self, alignment: &str) {
        self.node.align(alignment);
    }

    fn fade_in(&mut self, duration: f32) {
        self.node.fade_in(duration);
    }
}

#[pyclass]
#[derive(Clone)]
pub struct VideoNode {
    #[pyo3(get)]
    pub node: Node,
    #[pyo3(get, set)]
    pub path: String,
}

#[pymethods]
impl VideoNode {
    #[new]
    fn new(path: String) -> Self {
        Self {
            node: Node::new(format!("video_{}", path.len())),
            path,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ShapeNode {
    #[pyo3(get)]
    pub node: Node,
    #[pyo3(get, set)]
    pub shape_type: String,
    #[pyo3(get, set)]
    pub color: String,
}

#[pymethods]
impl ShapeNode {
    #[new]
    fn new(shape_type: String, color: String) -> Self {
        Self {
            node: Node::new(format!("shape_{}", shape_type)),
            shape_type,
            color,
        }
    }
}

