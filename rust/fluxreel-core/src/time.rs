use pyo3::prelude::*;

/// Convert frames to seconds
#[pyfunction]
pub fn frames_to_seconds(frames: u32, fps: u32) -> f32 {
    frames as f32 / fps as f32
}

/// Convert seconds to frames
#[pyfunction]
pub fn seconds_to_frames(seconds: f32, fps: u32) -> u32 {
    (seconds * fps as f32) as u32
}

/// Format time as HH:MM:SS.mmm
#[pyfunction]
pub fn format_time(seconds: f32) -> String {
    let hours = (seconds as u32) / 3600;
    let minutes = ((seconds as u32) % 3600) / 60;
    let secs = (seconds as u32) % 60;
    let millis = ((seconds - seconds.floor()) * 1000.0) as u32;
    
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
}

/// Parse time string (HH:MM:SS or MM:SS) to seconds
#[pyfunction]
pub fn parse_time(time_str: &str) -> PyResult<f32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        // MM:SS
        let minutes: f32 = parts[0].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format")
        })?;
        let seconds: f32 = parts[1].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format")
        })?;
        Ok(minutes * 60.0 + seconds)
    } else if parts.len() == 3 {
        // HH:MM:SS
        let hours: f32 = parts[0].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format")
        })?;
        let minutes: f32 = parts[1].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format")
        })?;
        let seconds: f32 = parts[2].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format")
        })?;
        Ok(hours * 3600.0 + minutes * 60.0 + seconds)
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid time format"))
    }
}

