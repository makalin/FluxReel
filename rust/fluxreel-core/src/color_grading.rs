use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ColorWheel {
    pub lift: (f32, f32, f32),    // RGB lift (shadows)
    pub gamma: (f32, f32, f32),   // RGB gamma (midtones)
    pub gain: (f32, f32, f32),    // RGB gain (highlights)
}

#[pyclass]
#[derive(Clone)]
pub struct ColorGrading {
    pub wheels: ColorWheel,
    #[pyo3(get, set)]
    pub temperature: f32,  // Color temperature (-100 to 100)
    #[pyo3(get, set)]
    pub tint: f32,         // Green/Magenta tint (-100 to 100)
    #[pyo3(get, set)]
    pub exposure: f32,    // Exposure adjustment (-5.0 to 5.0)
    #[pyo3(get, set)]
    pub contrast: f32,    // Contrast (-100 to 100)
    #[pyo3(get, set)]
    pub saturation: f32,  // Saturation (0 to 200)
}

#[pymethods]
impl ColorGrading {
    #[new]
    fn new() -> Self {
        Self {
            wheels: ColorWheel {
                lift: (0.0, 0.0, 0.0),
                gamma: (0.0, 0.0, 0.0),
                gain: (0.0, 0.0, 0.0),
            },
            temperature: 0.0,
            tint: 0.0,
            exposure: 0.0,
            contrast: 0.0,
            saturation: 100.0,
        }
    }

    fn set_lift(&mut self, r: f32, g: f32, b: f32) {
        self.wheels.lift = (r, g, b);
    }

    fn set_gamma(&mut self, r: f32, g: f32, b: f32) {
        self.wheels.gamma = (r, g, b);
    }

    fn set_gain(&mut self, r: f32, g: f32, b: f32) {
        self.wheels.gain = (r, g, b);
    }

    fn reset(&mut self) {
        self.wheels.lift = (0.0, 0.0, 0.0);
        self.wheels.gamma = (0.0, 0.0, 0.0);
        self.wheels.gain = (0.0, 0.0, 0.0);
        self.temperature = 0.0;
        self.tint = 0.0;
        self.exposure = 0.0;
        self.contrast = 0.0;
        self.saturation = 100.0;
    }
}

#[derive(Debug, Clone)]
pub struct CurvePoint {
    pub x: f32,
    pub y: f32,
    pub handle_in: (f32, f32),
    pub handle_out: (f32, f32),
}

#[pyclass]
#[derive(Clone)]
pub struct ColorCurves {
    pub luma: Vec<CurvePoint>,
    pub red: Vec<CurvePoint>,
    pub green: Vec<CurvePoint>,
    pub blue: Vec<CurvePoint>,
}

#[pymethods]
impl ColorCurves {
    #[new]
    fn new() -> Self {
        Self {
            luma: vec![CurvePoint {
                x: 0.0,
                y: 0.0,
                handle_in: (0.0, 0.0),
                handle_out: (1.0, 1.0),
            }],
            red: Vec::new(),
            green: Vec::new(),
            blue: Vec::new(),
        }
    }

    fn add_point(&mut self, channel: &str, x: f32, y: f32) {
        let point = CurvePoint {
            x,
            y,
            handle_in: (x - 0.1, y),
            handle_out: (x + 0.1, y),
        };
        match channel {
            "luma" => self.luma.push(point),
            "red" => self.red.push(point),
            "green" => self.green.push(point),
            "blue" => self.blue.push(point),
            _ => {}
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct LUT {
    #[pyo3(get, set)]
    pub path: String,
    #[pyo3(get, set)]
    pub intensity: f32,  // 0.0 to 1.0
}

#[pymethods]
impl LUT {
    #[new]
    fn new(path: String) -> Self {
        Self {
            path,
            intensity: 1.0,
        }
    }

    fn load_cube(&mut self, path: &str) -> PyResult<()> {
        self.path = path.to_string();
        // LUT loading would be implemented here
        Ok(())
    }
}

