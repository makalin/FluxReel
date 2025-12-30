use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    ZoomIn,
    ZoomOut,
    Rotate,
    WipeLeft,
    WipeRight,
    WipeUp,
    WipeDown,
    Blur,
    Glitch,
    Pixelate,
    Custom(String),
}

#[pyclass]
#[derive(Clone)]
pub struct Transition {
    #[pyo3(get, set)]
    pub effect: String,
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub easing: String,
}

#[pymethods]
impl Transition {
    #[new]
    fn new(effect: String, duration: f32) -> Self {
        Self {
            effect,
            duration,
            easing: "ease_in_out".to_string(),
        }
    }

    fn with_easing(&mut self, easing: String) {
        self.easing = easing;
    }
}

/// Apply transition effect between two scenes
pub fn apply_transition(
    transition_type: &str,
    progress: f32,
    from_frame: &[u8],
    to_frame: &[u8],
    width: u32,
    height: u32,
) -> Vec<u8> {
    match transition_type {
        "fade" => fade_transition(progress, from_frame, to_frame),
        "slide_left" => slide_transition(progress, from_frame, to_frame, width, height, true, false),
        "slide_right" => slide_transition(progress, from_frame, to_frame, width, height, false, false),
        "slide_up" => slide_transition(progress, from_frame, to_frame, width, height, false, true),
        "slide_down" => slide_transition(progress, from_frame, to_frame, width, height, false, false),
        "zoom_in" => zoom_transition(progress, from_frame, to_frame, width, height, true),
        "zoom_out" => zoom_transition(progress, from_frame, to_frame, width, height, false),
        "wipe_left" => wipe_transition(progress, from_frame, to_frame, width, height, true, false),
        "wipe_right" => wipe_transition(progress, from_frame, to_frame, width, height, false, false),
        _ => fade_transition(progress, from_frame, to_frame),
    }
}

fn fade_transition(progress: f32, from: &[u8], to: &[u8]) -> Vec<u8> {
    let alpha = progress.clamp(0.0, 1.0);
    from.iter()
        .zip(to.iter())
        .map(|(f, t)| {
            ((*f as f32) * (1.0 - alpha) + (*t as f32) * alpha) as u8
        })
        .collect()
}

fn slide_transition(
    progress: f32,
    from: &[u8],
    to: &[u8],
    _width: u32,
    _height: u32,
    _left: bool,
    _vertical: bool,
) -> Vec<u8> {
    // Simplified implementation - actual would handle pixel positioning
    if progress < 0.5 {
        from.to_vec()
    } else {
        to.to_vec()
    }
}

fn zoom_transition(
    progress: f32,
    from: &[u8],
    to: &[u8],
    _width: u32,
    _height: u32,
    _zoom_in: bool,
) -> Vec<u8> {
    // Simplified implementation
    if progress < 0.5 {
        from.to_vec()
    } else {
        to.to_vec()
    }
}

fn wipe_transition(
    progress: f32,
    from: &[u8],
    to: &[u8],
    _width: u32,
    _height: u32,
    _left: bool,
    _vertical: bool,
) -> Vec<u8> {
    // Simplified implementation
    if progress < 0.5 {
        from.to_vec()
    } else {
        to.to_vec()
    }
}

