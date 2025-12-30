<div align="center">

<img src="fluxreel_logo.svg" alt="FluxReel Logo" width="100%">

# FluxReel 🎬
### Code Less. Render Faster.

**FluxReel** is a high-performance, programmable video engine designed for developers who find existing tools too complex. It combines a native Rust core with a simplified Pythonic scripting layer, offering the perfect balance between terminal automation and GUI visualization.

[Features](#-key-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [CLI Commands](#-cli-commands) • [Author](#-author)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)
![License](https://img.shields.io/badge/license-MIT-orange)

</div>

---

## 🚀 Why FluxReel?

Existing programmatic video tools (like Motion Canvas or Manim) are powerful but often suffer from heavy web-based architectures, complex generator syntax, or slow rendering times.

**FluxReel** strips away the complexity:
* **Native Speed:** Built on a Rust rendering engine with Python bindings.
* **Dual Workflow:** Compile from the CLI *or* edit visually in the Studio GUI.
* **Simple Syntax:** No complex async/await or generator functions. Just linear, readable scripts.
* **Rich Feature Set:** 30+ easing functions, transitions, effects, and audio processing tools.

## ✨ Key Features

### 🎞️ Flexible Rendering
* **Multi-Format:** Output to `.mp4`, `.mov` (ProRes), `.gif`, `.webp`, or `PNG Sequence`.
* **Resolutions:** Built-in presets for 4K, 1080p (YouTube), 720p, and 9:16 (TikTok/Reels).
* **High FPS:** Support for 120+ FPS for high-quality slow-motion editing.
* **Custom Resolutions:** Support for any custom width×height resolution.

### 🎛️ Architecture
* **Unified Layer System:** Text, Images, Videos, and Shapes are all `Nodes` with uniform properties.
* **Smart Transitions:** 12+ transition effects (fade, slide, zoom, wipe, blur, glitch, etc.).
* **Audio-Reactive:** Built-in beat detection and BPM calculation to drive animation properties automatically.
* **Keyframe Animation:** Advanced keyframe-based animation system with easing support.

### 🎨 Visual Effects
* **Blur & Glow:** Apply blur and glow effects to any node.
* **Shadows:** Drop shadows with customizable offset, blur, and color.
* **Color Adjustment:** Brightness, contrast, saturation, and hue controls.
* **Chroma Key:** Green screen / chroma key support.
* **Noise Effects:** Add grain and noise effects.

### 📐 Geometry & Math
* **Shape Primitives:** Rectangle, Circle, and Polygon classes with collision detection.
* **Point Utilities:** Distance, angle calculation, and rotation functions.
* **Easing Functions:** 30+ easing functions (linear, quadratic, cubic, elastic, bounce, etc.).

### 🎵 Audio Processing
* **Audio Tracks:** Load and manipulate audio files.
* **Beat Detection:** Energy-based beat detection algorithm.
* **BPM Calculation:** Automatic BPM calculation from beat times.
* **Audio Mixer:** Multi-track audio mixing with volume control.
* **Waveform Analysis:** Extract waveform data and amplitude at specific times.

### ⏱️ Time Utilities
* **Frame Conversion:** Convert between frames and seconds.
* **Time Formatting:** Format time as HH:MM:SS.mmm.
* **Time Parsing:** Parse time strings to seconds.

## 📦 Installation

**FluxReel** is available as a single binary or a Python package.

### Building from Source

#### Prerequisites
- Rust (latest stable)
- Python 3.8+
- Cargo

#### Build Rust Components
```bash
cd rust
cargo build --release
```

#### Install Python Package
```bash
cd python
pip install -e .
```

#### Install CLI Tool
```bash
cd rust/fluxreel-cli
cargo install --path .
```

### via PIP (Python) - Coming Soon
```bash
pip install fluxreel
```

### via Cargo (Rust) - Coming Soon
```bash
cargo install fluxreel-cli
```

## ⚡ Quick Start

### Create a New Project

```bash
fluxreel new my_first_video
cd my_first_video
```

Available templates:
- `default` - Full-featured template with intro scene
- `minimal` - Minimal template with basic setup
- `vertical` - Template optimized for 9:16 format

```bash
fluxreel new my_video --template minimal
```

### Basic Example

Edit `main.flux` (or `main.py`) with your script:

```python
from fluxreel import Scene, Text, Image, Audio, setup

# Global Settings
setup(res="1080p", fps=60)

# Create intro scene
intro_scene = Scene("Intro")
bg = Image("assets/background.jpg")
title = Text("Hello FluxReel", size=100, color="#FFFFFF")

# Declarative Animation
title.align("center")
title.fade_in(duration=1.0)
title.scale(start=0.5, end=1.0, ease="elastic_out")

# Audio Sync
voice = Audio("assets/intro.mp3")
voice.play()

# Create main scene
main_scene = Scene("Main")
transition(effect="slide_left", duration=0.5)
```

### Advanced Example with Effects

```python
from fluxreel import (
    Scene, Text, Image, 
    GlowEffect, ShadowEffect, BlurEffect,
    Animation, AnimationSequence
)

setup(res="1080p", fps=60)

scene = Scene("Advanced")
title = Text("FluxReel", size=120, color="#FF6B6B")

# Add visual effects
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")

# Keyframe animation
anim = Animation("opacity", duration=2.0)
anim.add_keyframe(0.0, 0.0, "ease_in")
anim.add_keyframe(1.0, 1.0, "ease_out")
anim.infinite()  # Loop infinitely
```

### Render Your Project

```bash
# Basic render
fluxreel render main.flux -o output.mp4

# With options
fluxreel render main.flux -o output.mp4 --resolution 4K --fps 120 --quality high
```

### Launch Studio GUI

```bash
fluxreel studio
# Or with a project
fluxreel studio my_first_video
```

## 💻 CLI Commands

### `fluxreel new <name> [--template <template>]`
Create a new FluxReel project.

```bash
fluxreel new my_video
fluxreel new my_video --template vertical
```

### `fluxreel render <input> [options]`
Render a script to video.

**Options:**
- `-o, --output <path>` - Output file path
- `-r, --resolution <res>` - Resolution override (4K, 1080p, 720p, 9:16)
- `-f, --fps <fps>` - FPS override
- `-q, --quality <quality>` - Quality preset (low, medium, high, ultra)

```bash
fluxreel render main.flux -o output.mp4 --resolution 4K --fps 60
```

### `fluxreel studio [project]`
Launch FluxReel Studio GUI.

```bash
fluxreel studio
fluxreel studio my_project
```

### `fluxreel copilot <prompt> [--output <file>]`
Generate code from natural language prompt.

```bash
fluxreel copilot "Create a scene with a title that fades in"
```

### `fluxreel extract-audio <input> [--output <path>]`
Extract audio from video file.

```bash
fluxreel extract-audio video.mp4 -o audio.mp3
```

### `fluxreel convert <input> <output> [--format <format>]`
Convert video format.

```bash
fluxreel convert input.mp4 output.mov --format mov
```

### `fluxreel analyze-audio <input> [--beats] [--bpm]`
Analyze audio file.

```bash
fluxreel analyze-audio track.mp3 --beats --bpm
```

### `fluxreel templates`
List available project templates.

### `fluxreel validate <input>`
Validate script syntax.

```bash
fluxreel validate main.flux
```

## 📚 Documentation

### Easing Functions

FluxReel supports 30+ easing functions for smooth animations:

**Basic:**
- `linear` - No easing

**Quadratic:**
- `ease_in`, `ease_out`, `ease_in_out`
- `quad_in`, `quad_out`, `quad_in_out`

**Cubic:**
- `cubic_in`, `cubic_out`, `cubic_in_out`

**Advanced:**
- `elastic_in`, `elastic_out`, `elastic_in_out`
- `bounce_in`, `bounce_out`, `bounce_in_out`
- `back_in`, `back_out`, `back_in_out`
- `sine_in`, `sine_out`, `sine_in_out`
- `expo_in`, `expo_out`, `expo_in_out`
- `circ_in`, `circ_out`, `circ_in_out`

### Transitions

Available transition effects:
- `fade` - Crossfade between scenes
- `slide_left`, `slide_right`, `slide_up`, `slide_down`
- `zoom_in`, `zoom_out`
- `wipe_left`, `wipe_right`, `wipe_up`, `wipe_down`
- `rotate`, `blur`, `glitch`, `pixelate`

### Node Methods

All nodes support these methods:
- `align(alignment)` - Align node (center, left, right, top, bottom)
- `move_to(x, y)` - Move to absolute position
- `move_by(dx, dy)` - Move by relative offset
- `rotate(angle)` - Set rotation angle
- `rotate_by(angle)` - Rotate by relative angle
- `set_opacity(opacity)` - Set opacity (0.0 to 1.0)
- `set_scale(scale_x, scale_y)` - Set scale
- `set_visible(visible)` - Show/hide node
- `fade_in(duration)` - Fade in animation
- `fade_out(duration)` - Fade out animation
- `scale(start, end, ease)` - Scale animation
- `slide(start_x, start_y, end_x, end_y, ease)` - Slide animation

### Visual Effects

```python
from fluxreel import BlurEffect, GlowEffect, ShadowEffect, ColorAdjustEffect

# Blur
blur = BlurEffect(radius=10.0)

# Glow
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)

# Shadow
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")

# Color adjustment
color_adj = ColorAdjustEffect()
color_adj.brightness = 0.2
color_adj.contrast = 1.2
color_adj.saturation = 1.5
color_adj.hue = 30.0
```

### Audio Processing

```python
from fluxreel import Audio, AudioMixer, detect_beats_energy, calculate_bpm

# Load audio
audio = Audio("track.mp3")

# Detect beats
beats = audio.detect_beats()
bpm = calculate_bpm(beats)

# Audio mixer
mixer = AudioMixer()
mixer.add_track(audio)
mixer.set_volume(0, 0.8)
mixer.master_volume = 0.9
```

### Geometry Utilities

```python
from fluxreel import Rectangle, Circle, Polygon, point_distance, rotate_point

# Create shapes
rect = Rectangle(0, 0, 100, 100)
circle = Circle(50, 50, 25)
polygon = Polygon([(0, 0), (100, 0), (50, 100)])

# Check collisions
if rect.contains(50, 50):
    print("Point is inside rectangle")

# Calculate distance
dist = point_distance(0, 0, 100, 100)

# Rotate point
x, y = rotate_point(10, 10, 0, 0, 45)  # Rotate 45 degrees
```

### Time Utilities

```python
from fluxreel import frames_to_seconds, seconds_to_frames, format_time, parse_time

# Convert frames to seconds
seconds = frames_to_seconds(1800, 60)  # 30 seconds at 60fps

# Convert seconds to frames
frames = seconds_to_frames(30.0, 60)  # 1800 frames

# Format time
time_str = format_time(125.5)  # "00:02:05.500"

# Parse time
seconds = parse_time("02:05.500")  # 125.5 seconds
```

## 🗺️ Roadmap

### 🚧 In Progress
- [ ] **FluxReel Studio** (GUI Editor with Timeline)
- [ ] GPU Shader Support (GLSL)
- [ ] Actual Video Rendering Implementation

### 📋 Planned
- [ ] Live Streaming Output (RTMP)
- [ ] AI Asset Generation Integration
- [ ] Webcam/Screen Capture Support
- [ ] Advanced Particle System
- [ ] 3D Transformations
- [ ] Multi-threaded Rendering

## 🤝 Contributing

Contributions are welcome! Please read `CONTRIBUTING.md` for details on our code of conduct and the process for submitting pull requests.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/makalin/FluxReel.git
cd FluxReel

# Build Rust components
cd rust && cargo build

# Install Python package
cd ../python && pip install -e .

# Run tests
pytest tests/
cargo test
```

## 📖 Additional Resources

- [API Documentation](docs/API.md)
- [Features Reference](FEATURES.md)
- [Project Structure](PROJECT_STRUCTURE.md)
- [Quick Start Guide](QUICKSTART.md)

## ✍️ Author

**Mehmet T. AKALIN** *Digital Vision*

* **GitHub:** [makalin](https://github.com/makalin)
* **Website:** [dv.com.tr](https://dv.com.tr)
* **LinkedIn:** [Mehmet T. AKALIN](https://www.linkedin.com/in/makalin/)
* **X (Twitter):** [@makalin](https://x.com/makalin)

---

<div align="center">
Made with ❤️ in Rust & Python
</div>
