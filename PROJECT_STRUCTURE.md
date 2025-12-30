# FluxReel Project Structure

```
FluxReel/
├── .gitignore                 # Git ignore rules
├── .editorconfig             # Editor configuration
├── .pre-commit-config.yaml   # Pre-commit hooks
├── Cargo.toml                # Rust workspace configuration
├── Makefile                  # Build automation
├── LICENSE                   # MIT License
├── README.md                 # Main documentation
├── CONTRIBUTING.md           # Contribution guidelines
│
├── rust/                     # Rust core components
│   ├── fluxreel-core/        # Core rendering library
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── src/
│   │       ├── lib.rs        # Main library entry point
│   │       ├── node.rs       # Node system (Text, Image, Video, Shape)
│   │       ├── scene.rs      # Scene management
│   │       ├── renderer.rs   # Rendering engine
│   │       ├── audio.rs      # Audio processing
│   │       └── utils.rs      # Utility functions
│   │
│   └── fluxreel-cli/         # CLI tool
│       ├── Cargo.toml
│       └── src/
│           └── main.rs       # CLI entry point
│
├── python/                   # Python bindings
│   ├── pyproject.toml        # Python package configuration
│   ├── requirements.txt      # Python dependencies
│   ├── README.md            # Python package docs
│   └── fluxreel/            # Python package
│       ├── __init__.py       # Package initialization
│       └── __main__.py       # Package entry point
│
├── studio/                   # GUI Studio application
│   ├── package.json          # Node.js dependencies
│   └── src/
│       ├── main.js           # Electron main process
│       └── index.html        # Studio UI
│
├── examples/                 # Example scripts
│   ├── hello_world.flux      # Basic example
│   └── tiktok_vertical.flux  # Vertical format example
│
├── tests/                    # Test suite
│   ├── __init__.py
│   └── test_basic.py         # Basic tests
│
├── docs/                     # Documentation
│   └── API.md                # API reference
│
└── .github/                  # GitHub workflows
    └── workflows/
        └── ci.yml            # CI/CD pipeline
```

## Key Components

### Rust Core (`rust/fluxreel-core`)
- High-performance rendering engine
- Node system for unified layer management
- Scene composition
- Audio processing
- Python bindings via PyO3

### CLI Tool (`rust/fluxreel-cli`)
- Command-line interface
- Project creation
- Script rendering
- Studio launcher
- AI Copilot integration

### Python Package (`python/fluxreel`)
- Pythonic API for FluxReel
- Simplified scripting interface
- Wraps Rust core functionality

### Studio GUI (`studio/`)
- Electron-based GUI editor
- Timeline interface
- Visual editing capabilities
- Real-time preview

## Build & Development

See `README.md` and `Makefile` for build instructions.

