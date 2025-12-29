<div align="center">

# FluxReel 🎬
### Code Less. Render Faster.

**FluxReel** is a high-performance, programmable video engine designed for developers who find existing tools too complex. It combines a native Rust core with a simplified Pythonic scripting layer, offering the perfect balance between terminal automation and GUI visualization.

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Author](#author)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)
![License](https://img.shields.io/badge/license-MIT-orange)

</div>

---

## 🚀 Why FluxReel?

Existing programmatic video tools (like Motion Canvas or Manim) are powerful but often suffer from heavy web-based architectures, complex generator syntax, or slow rendering times.

**FluxReel** strips away the complexity:
* **Native Speed:** Built on a Rust/C++ rendering engine.
* **Dual Workflow:** Compile from the CLI *or* edit visually in the Studio GUI.
* **Simple Syntax:** No complex async/await or generator functions. Just linear, readable scripts.
* **Live Ready:** Supports webcam, screen capture, and RTMP streams as inputs.

## ✨ Key Features

### 🎞️ Flexible Rendering
* **Multi-Format:** Output to `.mp4`, `.mov` (ProRes), `.gif`, `.webp`, or `PNG Sequence`.
* **Resolutions:** Built-in presets for 4K, 1080p (YouTube), and 9:16 (TikTok/Reels).
* **High FPS:** Support for 120+ FPS for high-quality slow-motion editing.

### 🎛️ Architecture
* **Unified Layer System:** Text, Images, Videos, and Shapes are all `Nodes` with uniform properties.
* **Smart Transitions:** Procedural transitions that interpret motion context.
* **Audio-Reactive:** Built-in beat detection to drive animation properties automatically.

### 🤖 AI-Powered
* **Copilot:** Built-in LLM support to generate scene code from natural language prompts.
* **TTS Sync:** Auto-align animation duration to AI-generated voiceovers.

## 📦 Installation

**FluxReel** is available as a single binary or a Python package.

### via PIP (Python)
```bash
pip install fluxreel

```

### via Cargo (Rust)

```bash
cargo install fluxreel-cli

```

## ⚡ Quick Start

Create a new project:

```bash
fluxreel new my_first_video
cd my_first_video

```

Edit `main.flux` (or `main.py`) with your script:

```python
from fluxreel import Scene, Text, Image, Audio

# Global Settings
setup(res="1080p", fps=60)

scene "Intro":
    # Assets
    bg = Image("assets/background.jpg")
    title = Text("Hello FluxReel", size=100, color="#FFFFFF")
    
    # Declarative Animation
    title.align("center")
    title.fade_in(duration=1.0)
    title.scale(start=0.5, end=1.0, ease="elastic_out")
    
    # Audio Sync
    voice = Audio("assets/intro.mp3")
    voice.play()
    
    # Wait for audio to finish automatically
    wait(voice.duration)

scene "Main":
    transition(effect="slide_left", duration=0.5)
    # ... your main content here

```

**Run the CLI Renderer:**

```bash
fluxreel render main.flux -o output.mp4

```

**Launch the GUI Studio:**

```bash
fluxreel studio

```

## 🗺️ Roadmap

* [x] Core Rendering Engine (Rust)
* [x] Basic Python Bindings
* [ ] **FluxReel Studio** (GUI Editor with Timeline)
* [ ] GPU Shader Support (GLSL)
* [ ] Live Streaming Output (RTMP)
* [ ] AI Asset Generation Integration

## 🤝 Contributing

Contributions are welcome! Please read `CONTRIBUTING.md` for details on our code of conduct and the process for submitting pull requests.

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
