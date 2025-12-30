# FluxReel Python Package

Python bindings for FluxReel video rendering engine.

## Installation

```bash
pip install fluxreel
```

Or install from source:

```bash
cd python
pip install -e .
```

## Usage

```python
from fluxreel import Scene, Text, Image, Audio

setup(res="1080p", fps=60)

scene = Scene("Intro")
title = Text("Hello FluxReel", size=100, color="#FFFFFF")
title.align("center")
title.fade_in(duration=1.0)
```

## Development

Build the Rust extension:

```bash
maturin develop
```

Run tests:

```bash
pytest tests/
```

