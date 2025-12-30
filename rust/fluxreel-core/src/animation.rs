use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub easing: String,
}

#[pyclass]
#[derive(Clone)]
pub struct Animation {
    #[pyo3(get)]
    pub property: String,
    pub keyframes: Vec<Keyframe>,
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub loop_count: i32, // -1 for infinite
}

#[pymethods]
impl Animation {
    #[new]
    fn new(property: String, duration: f32) -> Self {
        Self {
            property,
            keyframes: Vec::new(),
            duration,
            loop_count: 1,
        }
    }

    fn add_keyframe(&mut self, time: f32, value: f32, easing: String) {
        self.keyframes.push(Keyframe {
            time,
            value,
            easing,
        });
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    fn set_loop(&mut self, count: i32) {
        self.loop_count = count;
    }

    fn infinite(&mut self) {
        self.loop_count = -1;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AnimationSequence {
    pub animations: Vec<Animation>,
    #[pyo3(get, set)]
    pub duration: f32,
}

#[pymethods]
impl AnimationSequence {
    #[new]
    fn new() -> Self {
        Self {
            animations: Vec::new(),
            duration: 0.0,
        }
    }

    fn add(&mut self, _animation: Animation) {
        // Note: This would need to handle Python objects properly
        // For now, this is a placeholder
    }

    fn parallel(&mut self, _animations: Vec<Animation>) {
        // Note: This would need to handle Python objects properly
        // For now, this is a placeholder
    }

    fn calculate_duration(&mut self) {
        self.duration = self
            .animations
            .iter()
            .map(|a| a.duration)
            .fold(0.0, f32::max);
    }
}

/// Interpolate value at given time using keyframes
pub fn interpolate_keyframes(keyframes: &[Keyframe], time: f32) -> f32 {
    if keyframes.is_empty() {
        return 0.0;
    }
    
    if time <= keyframes[0].time {
        return keyframes[0].value;
    }
    
    if time >= keyframes[keyframes.len() - 1].time {
        return keyframes[keyframes.len() - 1].value;
    }
    
    for i in 0..keyframes.len() - 1 {
        let k1 = &keyframes[i];
        let k2 = &keyframes[i + 1];
        
        if time >= k1.time && time <= k2.time {
            let t = (time - k1.time) / (k2.time - k1.time);
            // Apply easing from k1
            let eased_t = crate::utils::ease_function(&k1.easing, t);
            return k1.value + (k2.value - k1.value) * eased_t;
        }
    }
    
    keyframes[keyframes.len() - 1].value
}

