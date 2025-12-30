# FluxReel Features & Functions

## 🎨 Easing Functions

Comprehensive easing functions for smooth animations:

- **Linear**: `linear`
- **Quadratic**: `ease_in`, `ease_out`, `ease_in_out`, `quad_in`, `quad_out`, `quad_in_out`
- **Cubic**: `cubic_in`, `cubic_out`, `cubic_in_out`
- **Quartic**: `quart_in`, `quart_out`, `quart_in_out`
- **Quintic**: `quint_in`, `quint_out`, `quint_in_out`
- **Sine**: `sine_in`, `sine_out`, `sine_in_out`
- **Exponential**: `expo_in`, `expo_out`, `expo_in_out`
- **Circular**: `circ_in`, `circ_out`, `circ_in_out`
- **Elastic**: `elastic_in`, `elastic_out`, `elastic_in_out`
- **Back**: `back_in`, `back_out`, `back_in_out`
- **Bounce**: `bounce_in`, `bounce_out`, `bounce_in_out`

## 🎬 Transitions

Smooth scene transitions:

- `fade` - Crossfade between scenes
- `slide_left`, `slide_right`, `slide_up`, `slide_down` - Slide transitions
- `zoom_in`, `zoom_out` - Zoom transitions
- `wipe_left`, `wipe_right`, `wipe_up`, `wipe_down` - Wipe transitions
- `rotate` - Rotation transition
- `blur` - Blur transition
- `glitch` - Glitch effect transition
- `pixelate` - Pixelation transition

## ✨ Visual Effects

### BlurEffect
Apply blur to nodes:
```python
blur = BlurEffect(radius=10.0)
```

### GlowEffect
Add glow effect:
```python
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)
```

### ShadowEffect
Add drop shadow:
```python
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")
```

### ColorAdjustEffect
Adjust color properties:
```python
color_adj = ColorAdjustEffect()
color_adj.brightness = 0.2
color_adj.contrast = 1.2
color_adj.saturation = 1.5
color_adj.hue = 30.0
```

### ChromaKeyEffect
Green screen / chroma key:
```python
chroma = ChromaKeyEffect(color="#00FF00", threshold=0.5, smoothness=0.1)
```

### NoiseEffect
Add noise/grain:
```python
noise = NoiseEffect(amount=0.1, seed=42)
```

## 📐 Geometry Utilities

### Shapes
- **Rectangle**: `Rectangle(x, y, width, height)`
  - `contains(px, py)` - Check if point is inside
  - `intersects(other)` - Check intersection with another rectangle
  - `center()` - Get center point

- **Circle**: `Circle(x, y, radius)`
  - `contains(px, py)` - Check if point is inside
  - `intersects(other)` - Check intersection with another circle

- **Polygon**: `Polygon(points)`
  - `contains(px, py)` - Check if point is inside polygon

### Geometry Functions
- `point_distance(x1, y1, x2, y2)` - Calculate distance between points
- `point_angle(x1, y1, x2, y2)` - Calculate angle between points (degrees)
- `rotate_point(px, py, cx, cy, angle_degrees)` - Rotate point around center

## 🎞️ Animation System

### Animation
Keyframe-based animations:
```python
anim = Animation("opacity", duration=2.0)
anim.add_keyframe(0.0, 0.0, "ease_in")
anim.add_keyframe(1.0, 1.0, "ease_out")
anim.set_loop(3)  # Loop 3 times
anim.infinite()   # Loop infinitely
```

### AnimationSequence
Sequence multiple animations:
```python
seq = AnimationSequence()
seq.add(anim1)
seq.add(anim2)
seq.parallel([anim3, anim4])  # Run in parallel
```

## 🎵 Audio Processing

### AudioTrack
Enhanced audio functionality:
- `play()` - Play audio
- `detect_beats()` - Detect beats in audio
- `get_waveform(samples)` - Get waveform data
- `get_amplitude_at(time)` - Get amplitude at specific time
- `normalize()` - Normalize audio levels
- `fade_in(duration)` - Fade in effect
- `fade_out(duration)` - Fade out effect

### AudioMixer
Mix multiple audio tracks:
```python
mixer = AudioMixer()
mixer.add_track(track1)
mixer.add_track(track2)
mixer.set_volume(0, 0.8)  # Set volume for track 0
mixer.master_volume = 0.9
```

### Audio Analysis Functions
- `detect_beats_energy(audio_data, sample_rate, threshold)` - Energy-based beat detection
- `calculate_bpm(beat_times)` - Calculate BPM from beat times

## ⏱️ Time Utilities

- `frames_to_seconds(frames, fps)` - Convert frames to seconds
- `seconds_to_frames(seconds, fps)` - Convert seconds to frames
- `format_time(seconds)` - Format time as "HH:MM:SS.mmm"
- `parse_time(time_str)` - Parse time string to seconds

## 🎯 Node Enhancements

Additional node methods:
- `move_to(x, y)` - Move to absolute position
- `move_by(dx, dy)` - Move by relative offset
- `rotate(angle)` - Set rotation angle
- `rotate_by(angle)` - Rotate by relative angle
- `set_opacity(opacity)` - Set opacity (0.0 to 1.0)
- `set_scale(scale_x, scale_y)` - Set scale
- `set_visible(visible)` - Show/hide node
- `fade_out(duration)` - Fade out animation
- `slide(start_x, start_y, end_x, end_y, ease)` - Slide animation

## 🛠️ Utility Functions

### Math Utilities
- `lerp(start, end, t)` - Linear interpolation
- `clamp(value, min, max)` - Clamp value
- `map_range(value, in_min, in_max, out_min, out_max)` - Map to range
- `smoothstep(edge0, edge1, x)` - Smooth step interpolation
- `normalize(value, min, max)` - Normalize to 0-1
- `denormalize(value, min, max)` - Denormalize from 0-1
- `bezier(t, p0, p1, p2, p3)` - Cubic Bezier interpolation

### Color Utilities
- `hex_to_rgb(hex_color)` - Convert hex to RGB
- `rgb_to_hex(r, g, b)` - Convert RGB to hex
- `rgb_to_hsl(r, g, b)` - Convert RGB to HSL
- `hsl_to_rgb(h, s, l)` - Convert HSL to RGB

## 💻 CLI Commands

### New Commands
- `fluxreel new <name> [--template <template>]` - Create project with optional template
- `fluxreel render <input> [--output <path>] [--resolution <res>] [--fps <fps>] [--quality <quality>]` - Render with options
- `fluxreel extract-audio <input> [--output <path>]` - Extract audio from video
- `fluxreel convert <input> <output> [--format <format>]` - Convert video format
- `fluxreel analyze-audio <input> [--beats] [--bpm]` - Analyze audio file
- `fluxreel templates` - List available templates
- `fluxreel validate <input>` - Validate script syntax

### Templates
- `default` - Full-featured template
- `minimal` - Minimal template
- `vertical` - 9:16 format template

## 📚 Example Usage

```python
from fluxreel import (
    setup, Scene, Text, Image, Audio,
    Transition, BlurEffect, GlowEffect,
    Animation, AnimationSequence,
    point_distance, format_time
)

# Setup
setup(res="1080p", fps=60)

# Create scene with effects
scene = Scene("Intro")
title = Text("FluxReel", size=100, color="#FFFFFF")
title.align("center")

# Add effects
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")

# Animation
anim = Animation("opacity", duration=2.0)
anim.add_keyframe(0.0, 0.0, "ease_in")
anim.add_keyframe(1.0, 1.0, "ease_out")

# Transition
trans = Transition("fade", duration=0.5, easing="ease_in_out")

# Audio analysis
audio = Audio("track.mp3")
beats = audio.detect_beats()
bpm = calculate_bpm(beats)

# Time formatting
duration = 125.5
time_str = format_time(duration)  # "00:02:05.500"
```

## 🚀 Performance Features

- Native Rust core for maximum performance
- GPU acceleration support (planned)
- Multi-threaded rendering
- Efficient memory management
- Optimized audio processing

