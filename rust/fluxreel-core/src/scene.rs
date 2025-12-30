use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneAnimation {
    pub property: String,
    pub start_value: f32,
    pub end_value: f32,
    pub duration: f32,
    pub ease: String,
    pub start_time: f32,
}

#[pyclass]
#[derive(Clone)]
pub struct Scene {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub duration: f32,
    pub animations: Vec<SceneAnimation>,
}

#[pymethods]
impl Scene {
    #[new]
    fn new(name: String) -> Self {
        Self {
            name,
            duration: 0.0,
            animations: Vec::new(),
        }
    }

    fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }
}

