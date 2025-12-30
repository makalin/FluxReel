# FluxReel Quick Start Guide

## Installation

### Option 1: Python Package (Recommended for Development)

```bash
cd python
pip install -e .
```

### Option 2: Rust CLI

```bash
cd rust/fluxreel-cli
cargo install --path .
```

## Creating Your First Project

```bash
fluxreel new my_video
cd my_video
```

This creates a new project with:
- `main.flux` - Your main script file
- `assets/` - Directory for media files

## Writing Your Script

Edit `main.flux`:

```python
from fluxreel import Scene, Text, Image, Audio

# Global Settings
setup(res="1080p", fps=60)

scene "Intro":
    title = Text("Hello FluxReel", size=100, color="#FFFFFF")
    title.align("center")
    title.fade_in(duration=1.0)
    title.scale(start=0.5, end=1.0, ease="elastic_out")
```

## Rendering

### Via CLI

```bash
fluxreel render main.flux -o output.mp4
```

### Via Python

```python
from fluxreel import Renderer, setup

config = setup(res="1080p", fps=60)
renderer = Renderer(config)
# Add scenes...
renderer.render("output.mp4")
```

## Launching Studio

```bash
fluxreel studio
```

Or with a project:

```bash
fluxreel studio my_video
```

## Example Projects

Check the `examples/` directory for:
- `hello_world.flux` - Basic introduction
- `tiktok_vertical.flux` - Vertical format example

## Next Steps

- Read the [API Documentation](docs/API.md)
- Check out [Project Structure](PROJECT_STRUCTURE.md)
- See [Contributing Guidelines](CONTRIBUTING.md)

