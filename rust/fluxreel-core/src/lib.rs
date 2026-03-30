use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

pub mod ai_assets;
pub mod animation;
pub mod audio;
pub mod audio_pro;
pub mod blend_modes;
pub mod color_grading;
pub mod effects;
pub mod geometry;
pub mod masking;
pub mod motion_tracking;
pub mod multicam;
pub mod node;
pub mod renderer;
pub mod scene;
pub mod speed_ramping;
pub mod streaming;
pub mod time;
pub mod transitions;
pub mod utils;

pub use ai_assets::*;
pub use animation::*;
pub use audio::*;
pub use audio_pro::*;
pub use blend_modes::*;
pub use color_grading::*;
pub use effects::*;
pub use geometry::*;
pub use masking::*;
pub use motion_tracking::*;
pub use multicam::*;
pub use node::*;
pub use renderer::*;
pub use scene::*;
pub use speed_ramping::*;
pub use streaming::*;
pub use time::*;
pub use transitions::*;
pub use utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub resolution: Resolution,
    pub fps: u32,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Resolution {
    #[serde(rename = "480p")]
    SD,
    #[serde(rename = "2K")]
    TwoK,
    #[serde(rename = "4K")]
    FourK,
    #[serde(rename = "8K")]
    EightK,
    #[serde(rename = "1080p")]
    FullHD,
    #[serde(rename = "1440p")]
    QuadHD,
    #[serde(rename = "720p")]
    HD,
    #[serde(rename = "1:1")]
    Square,
    #[serde(rename = "4:5")]
    Portrait45,
    #[serde(rename = "9:16")]
    Vertical,
    Custom {
        width: u32,
        height: u32,
    },
}

impl Resolution {
    pub fn width(&self) -> u32 {
        match self {
            Resolution::SD => 854,
            Resolution::TwoK => 2048,
            Resolution::FourK => 3840,
            Resolution::EightK => 7680,
            Resolution::FullHD => 1920,
            Resolution::QuadHD => 2560,
            Resolution::HD => 1280,
            Resolution::Square => 1080,
            Resolution::Portrait45 => 1080,
            Resolution::Vertical => 1080,
            Resolution::Custom { width, .. } => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Resolution::SD => 480,
            Resolution::TwoK => 1080,
            Resolution::FourK => 2160,
            Resolution::EightK => 4320,
            Resolution::FullHD => 1080,
            Resolution::QuadHD => 1440,
            Resolution::HD => 720,
            Resolution::Square => 1080,
            Resolution::Portrait45 => 1350,
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

    // Professional Color Grading
    m.add_class::<ColorGrading>()?;
    m.add_class::<ColorCurves>()?;
    m.add_class::<LUT>()?;

    // Masking & Rotoscoping
    m.add_class::<Mask>()?;
    m.add_class::<RectangleMask>()?;
    m.add_class::<EllipseMask>()?;

    // Motion Tracking
    m.add_class::<MotionTracker>()?;
    m.add_class::<Stabilizer>()?;

    // Professional Audio
    m.add_class::<Equalizer>()?;
    m.add_class::<Compressor>()?;
    m.add_class::<Reverb>()?;
    m.add_class::<Limiter>()?;

    // Speed Ramping
    m.add_class::<SpeedRamp>()?;
    m.add_class::<TimeRemap>()?;

    // Blend Modes
    m.add_class::<BlendModeEffect>()?;

    // Live Streaming
    m.add_class::<StreamingConfig>()?;
    m.add_class::<Streamer>()?;
    m.add_class::<RTMPStreamer>()?;

    // Multi-Camera
    m.add_class::<CameraAngle>()?;
    m.add_class::<MultiCamSequence>()?;
    m.add_class::<MultiCamEditor>()?;

    // AI Asset Generation
    m.add_class::<AIGenerator>()?;
    m.add_class::<ImageGenerator>()?;
    m.add_class::<VideoGenerator>()?;
    m.add_class::<AudioGenerator>()?;
    m.add_class::<TextGenerator>()?;

    // Functions
    m.add_function(wrap_pyfunction!(create_rtmp_url, m)?)?;
    m.add_function(wrap_pyfunction!(validate_rtmp_url, m)?)?;
    m.add_function(wrap_pyfunction!(generate_asset, m)?)?;
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
        "480p" => Resolution::SD,
        "2K" | "2k" => Resolution::TwoK,
        "4K" => Resolution::FourK,
        "8K" => Resolution::EightK,
        "1080p" => Resolution::FullHD,
        "1440p" => Resolution::QuadHD,
        "720p" => Resolution::HD,
        "1:1" | "square" => Resolution::Square,
        "4:5" | "portrait" | "instagram_portrait" => Resolution::Portrait45,
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
