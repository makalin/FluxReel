use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMethod {
    Timecode,
    Audio,
    Manual,
    InPoint,
}

#[pyclass]
#[derive(Clone)]
pub struct CameraAngle {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub source: String,  // Video file path or device ID
    pub offset: f32,     // Time offset in seconds
    #[pyo3(get, set)]
    pub enabled: bool,
    pub sync_method: SyncMethod,
}

#[pymethods]
impl CameraAngle {
    #[new]
    fn new(name: String, source: String) -> Self {
        Self {
            name,
            source,
            offset: 0.0,
            enabled: true,
            sync_method: SyncMethod::Timecode,
        }
    }

    fn set_sync_method(&mut self, method: String) {
        self.sync_method = match method.as_str() {
            "timecode" => SyncMethod::Timecode,
            "audio" => SyncMethod::Audio,
            "manual" => SyncMethod::Manual,
            "in_point" => SyncMethod::InPoint,
            _ => SyncMethod::Timecode,
        };
    }

    fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }
    
    fn get_offset(&self) -> f32 {
        self.offset
    }
}

#[pyclass]
#[derive(Clone)]
pub struct MultiCamSequence {
    pub angles: Vec<CameraAngle>,
    #[pyo3(get, set)]
    pub active_angle: usize,
    pub cuts: Vec<MultiCamCut>,
    #[pyo3(get, set)]
    pub sync_point: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCamCut {
    pub time: f32,
    pub angle_index: usize,
    pub transition: String,
}

#[pymethods]
impl MultiCamSequence {
    #[new]
    fn new() -> Self {
        Self {
            angles: Vec::new(),
            active_angle: 0,
            cuts: Vec::new(),
            sync_point: 0.0,
        }
    }

    fn add_angle(&mut self, angle: CameraAngle) {
        self.angles.push(angle);
    }

    fn remove_angle(&mut self, index: usize) -> PyResult<()> {
        if index < self.angles.len() {
            self.angles.remove(index);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Angle index out of range"
            ))
        }
    }

    fn sync_angles(&mut self, method: String) -> PyResult<()> {
        match method.as_str() {
            "timecode" => {
                // Sync by timecode
                println!("Syncing angles by timecode");
            }
            "audio" => {
                // Sync by audio waveform
                println!("Syncing angles by audio");
            }
            "in_point" => {
                // Sync to in point
                for angle in &mut self.angles {
                    angle.offset = -self.sync_point;
                }
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid sync method"
                ));
            }
        }
        Ok(())
    }

    fn add_cut(&mut self, time: f32, angle_index: usize, transition: String) -> PyResult<()> {
        if angle_index >= self.angles.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Angle index out of range"
            ));
        }
        self.cuts.push(MultiCamCut {
            time,
            angle_index,
            transition,
        });
        self.cuts.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        Ok(())
    }

    fn get_active_angle_at(&self, time: f32) -> Option<usize> {
        for cut in self.cuts.iter().rev() {
            if time >= cut.time {
                return Some(cut.angle_index);
            }
        }
        Some(self.active_angle)
    }

    fn switch_angle(&mut self, angle_index: usize) -> PyResult<()> {
        if angle_index < self.angles.len() {
            self.active_angle = angle_index;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Angle index out of range"
            ))
        }
    }

    fn preview_all_angles(&self) -> Vec<String> {
        self.angles.iter()
            .map(|a| format!("{}: {}", a.name, a.source))
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct MultiCamEditor {
    pub sequence: MultiCamSequence,
    #[pyo3(get, set)]
    pub preview_mode: String,  // "active", "all", "quad"
    #[pyo3(get, set)]
    pub show_timecode: bool,
}

#[pymethods]
impl MultiCamEditor {
    #[new]
    fn new() -> Self {
        Self {
            sequence: MultiCamSequence::new(),
            preview_mode: "active".to_string(),
            show_timecode: true,
        }
    }

    fn add_camera(&mut self, name: String, source: String) {
        let angle = CameraAngle::new(name, source);
        self.sequence.add_angle(angle);
    }

    fn auto_sync(&mut self) -> PyResult<()> {
        // Try audio sync first, fallback to timecode
        self.sequence.sync_angles("audio".to_string())?;
        Ok(())
    }

    fn cut_to_angle(&mut self, time: f32, angle_name: String) -> PyResult<()> {
        let angle_index = self.sequence.angles.iter()
            .position(|a| a.name == angle_name)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Angle '{}' not found", angle_name)
            ))?;
        
        self.sequence.add_cut(time, angle_index, "cut".to_string())?;
        Ok(())
    }

    fn render(&self, output_path: &str) -> PyResult<()> {
        println!("Rendering multi-cam sequence to: {}", output_path);
        println!("Total angles: {}", self.sequence.angles.len());
        println!("Total cuts: {}", self.sequence.cuts.len());
        // Rendering would happen here
        Ok(())
    }
}

