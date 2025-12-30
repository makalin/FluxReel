use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Comprehensive easing functions for animations
pub fn ease_function(ease_type: &str, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match ease_type {
        "linear" => t,
        
        // Quadratic
        "ease_in" | "quad_in" => t * t,
        "ease_out" | "quad_out" => 1.0 - (1.0 - t) * (1.0 - t),
        "ease_in_out" | "quad_in_out" => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - 2.0 * (1.0 - t) * (1.0 - t)
            }
        }
        
        // Cubic
        "cubic_in" => t * t * t,
        "cubic_out" => 1.0 - (1.0 - t).powi(3),
        "cubic_in_out" => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - 4.0 * (1.0 - t).powi(3)
            }
        }
        
        // Quartic
        "quart_in" => t * t * t * t,
        "quart_out" => 1.0 - (1.0 - t).powi(4),
        "quart_in_out" => {
            if t < 0.5 {
                8.0 * t * t * t * t
            } else {
                1.0 - 8.0 * (1.0 - t).powi(4)
            }
        }
        
        // Quintic
        "quint_in" => t * t * t * t * t,
        "quint_out" => 1.0 - (1.0 - t).powi(5),
        "quint_in_out" => {
            if t < 0.5 {
                16.0 * t * t * t * t * t
            } else {
                1.0 - 16.0 * (1.0 - t).powi(5)
            }
        }
        
        // Sine
        "sine_in" => 1.0 - (t * PI / 2.0).cos(),
        "sine_out" => (t * PI / 2.0).sin(),
        "sine_in_out" => -((PI * t).cos() - 1.0) / 2.0,
        
        // Exponential
        "expo_in" => if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * (t - 1.0)) },
        "expo_out" => if t == 1.0 { 1.0 } else { 1.0 - 2.0_f32.powf(-10.0 * t) },
        "expo_in_out" => {
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else if t < 0.5 {
                2.0_f32.powf(20.0 * t - 10.0) / 2.0
            } else {
                (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0
            }
        }
        
        // Circular
        "circ_in" => 1.0 - (1.0 - t * t).sqrt(),
        "circ_out" => (1.0 - (t - 1.0) * (t - 1.0)).sqrt(),
        "circ_in_out" => {
            if t < 0.5 {
                (1.0 - (1.0 - 2.0 * t) * (1.0 - 2.0 * t)).sqrt() / 2.0
            } else {
                (1.0 + (2.0 * t - 1.0) * (2.0 * t - 1.0)).sqrt() / 2.0
            }
        }
        
        // Elastic
        "elastic_in" => {
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else {
                -2.0_f32.powf(10.0 * (t - 1.0)) * ((t - 1.0 - 0.075) * (2.0 * PI) / 0.3).sin()
            }
        }
        "elastic_out" | "elastic" => {
            let c4 = (2.0 * PI) / 3.0;
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else {
                2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
            }
        }
        "elastic_in_out" => {
            let c5 = (2.0 * PI) / 4.5;
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else if t < 0.5 {
                -(2.0_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
            } else {
                (2.0_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
            }
        }
        
        // Back
        "back_in" => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            c3 * t * t * t - c1 * t * t
        }
        "back_out" => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0) * (t - 1.0)
        }
        "back_in_out" => {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;
            if t < 0.5 {
                ((2.0 * t) * (2.0 * t) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
            } else {
                ((2.0 * t - 2.0) * (2.0 * t - 2.0) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
            }
        }
        
        // Bounce
        "bounce_in" => 1.0 - bounce_out(1.0 - t),
        "bounce_out" => bounce_out(t),
        "bounce_in_out" => {
            if t < 0.5 {
                (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
            } else {
                (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
            }
        }
        
        _ => t,
    }
}

fn bounce_out(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

/// Parse color from hex string to RGBA tuple
pub fn parse_color(color: &str) -> (u8, u8, u8, u8) {
    let color = color.trim_start_matches('#');
    if color.len() == 6 {
        let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(255);
        (r, g, b, 255)
    } else if color.len() == 8 {
        let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(255);
        let a = u8::from_str_radix(&color[6..8], 16).unwrap_or(255);
        (r, g, b, a)
    } else {
        (255, 255, 255, 255)
    }
}

/// Convert RGB to HSL
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    let mut h = 0.0;
    if delta != 0.0 {
        if max == r {
            h = 60.0 * (((g - b) / delta) % 6.0);
        } else if max == g {
            h = 60.0 * (((b - r) / delta) + 2.0);
        } else {
            h = 60.0 * (((r - g) / delta) + 4.0);
        }
    }
    if h < 0.0 {
        h += 360.0;
    }
    
    let l = (max + min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };
    
    (h, s, l)
}

/// Convert HSL to RGB
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    (
        ((r + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).clamp(0.0, 255.0) as u8,
    )
}

/// Interpolate between two values
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

/// Clamp value between min and max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Map value from one range to another
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Smooth step interpolation
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Calculate distance between two 2D points
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

/// Convert degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Convert radians to degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / PI
}

/// Normalize angle to 0-360 range
pub fn normalize_angle(angle: f32) -> f32 {
    let mut angle = angle % 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }
    angle
}

