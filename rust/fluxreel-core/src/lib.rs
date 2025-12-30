use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

pub mod node;
pub mod scene;
pub mod renderer;
pub mod audio;
pub mod utils;
pub mod transitions;
pub mod effects;
pub mod geometry;
pub mod animation;
pub mod time;

pub use node::*;
pub use scene::*;
pub use renderer::*;
pub use audio::*;
pub use utils::*;
pub use transitions::*;
pub use effects::*;
pub use geometry::*;
pub use animation::*;
pub use time::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub resolution: Resolution,
    pub fps: u32,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Resolution {
    #[serde(rename = "4K")]
    FourK,
    #[serde(rename = "1080p")]
    FullHD,
    #[serde(rename = "720p")]
    HD,
    #[serde(rename = "9:16")]
    Vertical,
    Custom { width: u32, height: u32 },
}

impl Resolution {
    pub fn width(&self) -> u32 {
        match self {
            Resolution::FourK => 3840,
            Resolution::FullHD => 1920,
            Resolution::HD => 1280,
            Resolution::Vertical => 1080,
            Resolution::Custom { width, .. } => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Resolution::FourK => 2160,
            Resolution::FullHD => 1080,
            Resolution::HD => 720,
            Resolution::Vertical => 1920,
            Resolution::Custom { height, .. } => *height,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    MP4,
    MOV,
    GIF,
    WEBP,
    PNGSequence,
}

#[pymodule]
fn fluxreel_core(_py: Python, m: &PyModule) -> PyResult<()> {
    // Core classes
    m.add_class::<Node>()?;
    m.add_class::<TextNode>()?;
    m.add_class::<ImageNode>()?;
    m.add_class::<VideoNode>()?;
    m.add_class::<ShapeNode>()?;
    m.add_class::<Scene>()?;
    m.add_class::<Renderer>()?;
    m.add_class::<AudioTrack>()?;
    
    // Transitions
    m.add_class::<Transition>()?;
    
    // Effects
    m.add_class::<BlurEffect>()?;
    m.add_class::<GlowEffect>()?;
    m.add_class::<ShadowEffect>()?;
    m.add_class::<ColorAdjustEffect>()?;
    m.add_class::<ChromaKeyEffect>()?;
    m.add_class::<NoiseEffect>()?;
    
    // Geometry
    m.add_class::<Rectangle>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Polygon>()?;
    
    // Animation
    m.add_class::<Animation>()?;
    m.add_class::<AnimationSequence>()?;
    
    // Audio utilities
    m.add_class::<AudioMixer>()?;
    
    // Functions
    m.add_function(wrap_pyfunction!(setup_project, m)?)?;
    m.add_function(wrap_pyfunction!(point_distance, m)?)?;
    m.add_function(wrap_pyfunction!(point_angle, m)?)?;
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(detect_beats_energy, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_bpm, m)?)?;
    m.add_function(wrap_pyfunction!(frames_to_seconds, m)?)?;
    m.add_function(wrap_pyfunction!(seconds_to_frames, m)?)?;
    m.add_function(wrap_pyfunction!(format_time, m)?)?;
    m.add_function(wrap_pyfunction!(parse_time, m)?)?;
    
    Ok(())
}

#[pyfunction]
fn setup_project(res: &str, fps: u32) -> PyResult<PyObject> {
    use pyo3::types::PyDict;
    let resolution = match res {
        "4K" => Resolution::FourK,
        "1080p" => Resolution::FullHD,
        "720p" => Resolution::HD,
        "9:16" | "tiktok" | "reels" => Resolution::Vertical,
        _ => {
            let parts: Vec<&str> = res.split('x').collect();
            if parts.len() == 2 {
                Resolution::Custom {
                    width: parts[0].parse().unwrap_or(1920),
                    height: parts[1].parse().unwrap_or(1080),
                }
            } else {
                Resolution::FullHD
            }
        }
    };

    let width = resolution.width();
    let height = resolution.height();
    
    // Return as Python dict
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("fps", fps)?;
        dict.set_item("width", width)?;
        dict.set_item("height", height)?;
        Ok(dict.to_object(py))
    })
}

