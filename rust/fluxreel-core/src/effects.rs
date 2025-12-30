use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone)]
pub struct BlurEffect {
    #[pyo3(get, set)]
    pub radius: f32,
}

#[pymethods]
impl BlurEffect {
    #[new]
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct GlowEffect {
    #[pyo3(get, set)]
    pub intensity: f32,
    #[pyo3(get, set)]
    pub color: String,
    #[pyo3(get, set)]
    pub radius: f32,
}

#[pymethods]
impl GlowEffect {
    #[new]
    fn new(intensity: f32, color: String, radius: f32) -> Self {
        Self {
            intensity,
            color,
            radius,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ShadowEffect {
    #[pyo3(get, set)]
    pub offset_x: f32,
    #[pyo3(get, set)]
    pub offset_y: f32,
    #[pyo3(get, set)]
    pub blur: f32,
    #[pyo3(get, set)]
    pub color: String,
    #[pyo3(get, set)]
    pub opacity: f32,
}

#[pymethods]
impl ShadowEffect {
    #[new]
    fn new(offset_x: f32, offset_y: f32, blur: f32, color: String) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            color,
            opacity: 1.0,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ColorAdjustEffect {
    #[pyo3(get, set)]
    pub brightness: f32,
    #[pyo3(get, set)]
    pub contrast: f32,
    #[pyo3(get, set)]
    pub saturation: f32,
    #[pyo3(get, set)]
    pub hue: f32,
}

#[pymethods]
impl ColorAdjustEffect {
    #[new]
    fn new() -> Self {
        Self {
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            hue: 0.0,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ChromaKeyEffect {
    #[pyo3(get, set)]
    pub color: String,
    #[pyo3(get, set)]
    pub threshold: f32,
    #[pyo3(get, set)]
    pub smoothness: f32,
}

#[pymethods]
impl ChromaKeyEffect {
    #[new]
    fn new(color: String, threshold: f32) -> Self {
        Self {
            color,
            threshold,
            smoothness: 0.1,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct NoiseEffect {
    #[pyo3(get, set)]
    pub amount: f32,
    #[pyo3(get, set)]
    pub seed: u32,
}

#[pymethods]
impl NoiseEffect {
    #[new]
    fn new(amount: f32) -> Self {
        Self {
            amount,
            seed: 0,
        }
    }
}

