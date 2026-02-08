use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone)]
pub struct Equalizer {
    pub bands: Vec<EQBand>,
    #[pyo3(get, set)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EQBand {
    pub frequency: f32,  // Hz
    pub gain: f32,       // dB
    pub q: f32,          // Quality factor
    pub band_type: String, // "low_shelf", "high_shelf", "peak", "notch"
}

#[pymethods]
impl Equalizer {
    #[new]
    fn new() -> Self {
        Self {
            bands: Vec::new(),
            enabled: true,
        }
    }

    fn add_band(&mut self, frequency: f32, gain: f32, q: f32, band_type: String) {
        self.bands.push(EQBand {
            frequency,
            gain,
            q,
            band_type,
        });
    }

    fn remove_band(&mut self, index: usize) {
        if index < self.bands.len() {
            self.bands.remove(index);
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Compressor {
    #[pyo3(get, set)]
    pub threshold: f32,  // dB
    #[pyo3(get, set)]
    pub ratio: f32,     // 1:1 to 20:1
    #[pyo3(get, set)]
    pub attack: f32,    // ms
    #[pyo3(get, set)]
    pub release: f32,   // ms
    #[pyo3(get, set)]
    pub knee: f32,      // dB
    #[pyo3(get, set)]
    pub makeup_gain: f32, // dB
}

#[pymethods]
impl Compressor {
    #[new]
    fn new() -> Self {
        Self {
            threshold: -12.0,
            ratio: 4.0,
            attack: 3.0,
            release: 100.0,
            knee: 2.0,
            makeup_gain: 0.0,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Reverb {
    #[pyo3(get, set)]
    pub room_size: f32,  // 0.0 to 1.0
    #[pyo3(get, set)]
    pub damping: f32,    // 0.0 to 1.0
    #[pyo3(get, set)]
    pub wet_level: f32,  // 0.0 to 1.0
    #[pyo3(get, set)]
    pub dry_level: f32,  // 0.0 to 1.0
    #[pyo3(get, set)]
    pub width: f32,      // 0.0 to 1.0
}

#[pymethods]
impl Reverb {
    #[new]
    fn new() -> Self {
        Self {
            room_size: 0.5,
            damping: 0.5,
            wet_level: 0.3,
            dry_level: 0.7,
            width: 1.0,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Limiter {
    #[pyo3(get, set)]
    pub ceiling: f32,    // dB
    #[pyo3(get, set)]
    pub release: f32,    // ms
    #[pyo3(get, set)]
    pub lookahead: f32,  // ms
}

#[pymethods]
impl Limiter {
    #[new]
    fn new() -> Self {
        Self {
            ceiling: 0.0,
            release: 50.0,
            lookahead: 5.0,
        }
    }
}

