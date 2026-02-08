use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedKeyframe {
    pub time: f32,
    pub speed: f32,  // 0.0 to 10.0 (1.0 = normal speed)
    pub ease: String,
}

#[pyclass]
#[derive(Clone)]
pub struct SpeedRamp {
    pub keyframes: Vec<SpeedKeyframe>,
    #[pyo3(get, set)]
    pub frame_blending: bool,
    #[pyo3(get, set)]
    pub optical_flow: bool,
}

#[pymethods]
impl SpeedRamp {
    #[new]
    fn new() -> Self {
        Self {
            keyframes: vec![SpeedKeyframe {
                time: 0.0,
                speed: 1.0,
                ease: "linear".to_string(),
            }],
            frame_blending: false,
            optical_flow: false,
        }
    }

    fn add_keyframe(&mut self, time: f32, speed: f32, ease: String) {
        self.keyframes.push(SpeedKeyframe { time, speed, ease });
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    fn get_speed_at_time(&self, time: f32) -> f32 {
        if self.keyframes.is_empty() {
            return 1.0;
        }
        
        if time <= self.keyframes[0].time {
            return self.keyframes[0].speed;
        }
        
        if time >= self.keyframes[self.keyframes.len() - 1].time {
            return self.keyframes[self.keyframes.len() - 1].speed;
        }
        
        for i in 0..self.keyframes.len() - 1 {
            let k1 = &self.keyframes[i];
            let k2 = &self.keyframes[i + 1];
            
            if time >= k1.time && time <= k2.time {
                let t = (time - k1.time) / (k2.time - k1.time);
                // Apply easing
                let eased_t = crate::utils::ease_function(&k1.ease, t);
                return k1.speed + (k2.speed - k1.speed) * eased_t;
            }
        }
        
        self.keyframes[self.keyframes.len() - 1].speed
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TimeRemap {
    pub speed_ramp: SpeedRamp,
    #[pyo3(get, set)]
    pub maintain_pitch: bool,
}

#[pymethods]
impl TimeRemap {
    #[new]
    fn new() -> Self {
        Self {
            speed_ramp: SpeedRamp::new(),
            maintain_pitch: false,
        }
    }

    fn set_speed(&mut self, time: f32, speed: f32) {
        self.speed_ramp.add_keyframe(time, speed, "linear".to_string());
    }
}

