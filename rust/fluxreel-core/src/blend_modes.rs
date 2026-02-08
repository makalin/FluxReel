use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    Add,
    Subtract,
    Divide,
}

#[pyclass]
#[derive(Clone)]
pub struct BlendModeEffect {
    pub mode: String,
    #[pyo3(get, set)]
    pub opacity: f32,
}

#[pymethods]
impl BlendModeEffect {
    #[new]
    fn new(mode: String) -> Self {
        Self {
            mode,
            opacity: 1.0,
        }
    }

    fn set_mode(&mut self, mode: String) {
        self.mode = mode;
    }
}

/// Apply blend mode to two colors
pub fn apply_blend_mode(
    base: (f32, f32, f32),
    blend: (f32, f32, f32),
    mode: &str,
    opacity: f32,
) -> (f32, f32, f32) {
    let result = match mode {
        "multiply" => multiply(base, blend),
        "screen" => screen(base, blend),
        "overlay" => overlay(base, blend),
        "soft_light" => soft_light(base, blend),
        "hard_light" => hard_light(base, blend),
        "color_dodge" => color_dodge(base, blend),
        "color_burn" => color_burn(base, blend),
        "darken" => darken(base, blend),
        "lighten" => lighten(base, blend),
        "difference" => difference(base, blend),
        "exclusion" => exclusion(base, blend),
        "add" => add(base, blend),
        "subtract" => subtract(base, blend),
        _ => blend, // Normal
    };
    
    // Blend with opacity
    (
        base.0 + (result.0 - base.0) * opacity,
        base.1 + (result.1 - base.1) * opacity,
        base.2 + (result.2 - base.2) * opacity,
    )
}

fn multiply(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (base.0 * blend.0, base.1 * blend.1, base.2 * blend.2)
}

fn screen(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (1.0 - (1.0 - base.0) * (1.0 - blend.0),
     1.0 - (1.0 - base.1) * (1.0 - blend.1),
     1.0 - (1.0 - base.2) * (1.0 - blend.2))
}

fn overlay(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        if base.0 < 0.5 { 2.0 * base.0 * blend.0 } else { 1.0 - 2.0 * (1.0 - base.0) * (1.0 - blend.0) },
        if base.1 < 0.5 { 2.0 * base.1 * blend.1 } else { 1.0 - 2.0 * (1.0 - base.1) * (1.0 - blend.1) },
        if base.2 < 0.5 { 2.0 * base.2 * blend.2 } else { 1.0 - 2.0 * (1.0 - base.2) * (1.0 - blend.2) },
    )
}

fn soft_light(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        if blend.0 < 0.5 {
            base.0 - (1.0 - 2.0 * blend.0) * base.0 * (1.0 - base.0)
        } else {
            base.0 + (2.0 * blend.0 - 1.0) * ((base.0.sqrt() - base.0))
        },
        if blend.1 < 0.5 {
            base.1 - (1.0 - 2.0 * blend.1) * base.1 * (1.0 - base.1)
        } else {
            base.1 + (2.0 * blend.1 - 1.0) * ((base.1.sqrt() - base.1))
        },
        if blend.2 < 0.5 {
            base.2 - (1.0 - 2.0 * blend.2) * base.2 * (1.0 - base.2)
        } else {
            base.2 + (2.0 * blend.2 - 1.0) * ((base.2.sqrt() - base.2))
        },
    )
}

fn hard_light(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    overlay(blend, base)
}

fn color_dodge(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        if blend.0 == 1.0 { 1.0 } else { (base.0 / (1.0 - blend.0)).min(1.0) },
        if blend.1 == 1.0 { 1.0 } else { (base.1 / (1.0 - blend.1)).min(1.0) },
        if blend.2 == 1.0 { 1.0 } else { (base.2 / (1.0 - blend.2)).min(1.0) },
    )
}

fn color_burn(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        if blend.0 == 0.0 { 0.0 } else { (1.0 - (1.0 - base.0) / blend.0).max(0.0) },
        if blend.1 == 0.0 { 0.0 } else { (1.0 - (1.0 - base.1) / blend.1).max(0.0) },
        if blend.2 == 0.0 { 0.0 } else { (1.0 - (1.0 - base.2) / blend.2).max(0.0) },
    )
}

fn darken(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (base.0.min(blend.0), base.1.min(blend.1), base.2.min(blend.2))
}

fn lighten(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (base.0.max(blend.0), base.1.max(blend.1), base.2.max(blend.2))
}

fn difference(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    ((base.0 - blend.0).abs(), (base.1 - blend.1).abs(), (base.2 - blend.2).abs())
}

fn exclusion(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    (base.0 + blend.0 - 2.0 * base.0 * blend.0,
     base.1 + blend.1 - 2.0 * base.1 * blend.1,
     base.2 + blend.2 - 2.0 * base.2 * blend.2)
}

fn add(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    ((base.0 + blend.0).min(1.0), (base.1 + blend.1).min(1.0), (base.2 + blend.2).min(1.0))
}

fn subtract(base: (f32, f32, f32), blend: (f32, f32, f32)) -> (f32, f32, f32) {
    ((base.0 - blend.0).max(0.0), (base.1 - blend.1).max(0.0), (base.2 - blend.2).max(0.0))
}

