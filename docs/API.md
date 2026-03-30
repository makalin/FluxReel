# FluxReel API Documentation

## Core Functions

### `setup(res, fps)`

Configure global project settings.

**Parameters:**
- `res` (str): Resolution preset ("480p", "720p", "1080p", "1440p", "2K", "4K", "8K", "1:1", "4:5", "9:16") or custom "WIDTHxHEIGHT"
- `fps` (int): Frames per second

**Returns:** `ProjectConfig` object

### `scene(name)`

Create a new scene context.

**Parameters:**
- `name` (str): Scene name

## Node Classes

### `Text(text, size, color)`

Create a text node.

**Parameters:**
- `text` (str): Text content
- `size` (float): Font size
- `color` (str): Hex color code (e.g., "#FFFFFF")

**Methods:**
- `align(alignment)`: Align text ("center", "left", "right", "top", "bottom")
- `fade_in(duration)`: Fade in animation
- `scale(start, end, ease)`: Scale animation

### `Image(path)`

Create an image node.

**Parameters:**
- `path` (str): Path to image file

### `Video(path)`

Create a video node.

**Parameters:**
- `path` (str): Path to video file

### `Shape(shape_type, color)`

Create a shape node.

**Parameters:**
- `shape_type` (str): Shape type ("circle", "rectangle", etc.)
- `color` (str): Hex color code

## Audio

### `Audio(path)`

Create an audio track.

**Parameters:**
- `path` (str): Path to audio file

**Properties:**
- `duration` (float): Audio duration in seconds

**Methods:**
- `play()`: Play audio
- `detect_beats()`: Detect beats in audio

## Renderer

### `Renderer(config)`

Create a renderer instance.

**Methods:**
- `add_scene(scene)`: Add a scene to render
- `render(output_path)`: Render to file

## Utility Functions

### `wait(duration)`

Wait for specified duration in seconds.

### `transition(effect, duration)`

Apply transition effect between scenes.

### `ease(ease_type, t)`

Apply an easing curve to a normalized time value.

### `available_resolutions()`

Return supported resolution presets and dimensions.

### `available_output_formats()`

Return supported output formats.

### `available_easings()`

Return supported easing function names.

### Utility exports

The following helpers are also available from the top-level `fluxreel` package:

- `lerp`
- `clamp`
- `map_range`
- `smoothstep`
- `hex_to_rgb`
- `rgb_to_hex`
- `rgb_to_hsl`
- `hsl_to_rgb`
- `bezier`
- `remap`
- `normalize`
- `denormalize`
