"""
FluxReel - Code Less. Render Faster.

A high-performance, programmable video engine designed for developers.
"""

import math
from types import SimpleNamespace
import time as _time

from fluxreel_core import (
    setup_project,
    Scene,
    TextNode as Text,
    ImageNode as Image,
    VideoNode as Video,
    ShapeNode as Shape,
    Renderer,
    AudioTrack as Audio,
    AudioMixer,
    Transition,
    BlurEffect,
    GlowEffect,
    ShadowEffect,
    ColorAdjustEffect,
    ChromaKeyEffect,
    NoiseEffect,
    Rectangle,
    Circle,
    Polygon,
    Animation,
    AnimationSequence,
    point_distance,
    point_angle,
    rotate_point,
    detect_beats_energy,
    calculate_bpm,
    frames_to_seconds,
    seconds_to_frames,
    format_time,
    parse_time,
    # Professional features
    ColorGrading,
    ColorCurves,
    LUT,
    Mask,
    RectangleMask,
    EllipseMask,
    MotionTracker,
    Stabilizer,
    Equalizer,
    Compressor,
    Reverb,
    Limiter,
    SpeedRamp,
    TimeRemap,
    BlendModeEffect,
    # Live Streaming
    StreamingConfig,
    Streamer,
    RTMPStreamer,
    create_rtmp_url,
    validate_rtmp_url,
    # Multi-Camera
    CameraAngle,
    MultiCamSequence,
    MultiCamEditor,
    # AI Asset Generation
    AIGenerator,
    ImageGenerator,
    VideoGenerator,
    AudioGenerator,
    TextGenerator,
    generate_asset,
)

__version__ = "0.1.0"

RESOLUTION_PRESETS = {
    "480p": (854, 480),
    "2K": (2048, 1080),
    "4K": (3840, 2160),
    "8K": (7680, 4320),
    "1080p": (1920, 1080),
    "1440p": (2560, 1440),
    "720p": (1280, 720),
    "1:1": (1080, 1080),
    "4:5": (1080, 1350),
    "9:16": (1080, 1920),
}

OUTPUT_FORMATS = ("mp4", "mov", "gif", "webp", "png_sequence")

EASING_FUNCTIONS = (
    "linear",
    "ease_in",
    "ease_out",
    "ease_in_out",
    "quad_in",
    "quad_out",
    "quad_in_out",
    "cubic_in",
    "cubic_out",
    "cubic_in_out",
    "quart_in",
    "quart_out",
    "quart_in_out",
    "quint_in",
    "quint_out",
    "quint_in_out",
    "sine_in",
    "sine_out",
    "sine_in_out",
    "expo_in",
    "expo_out",
    "expo_in_out",
    "circ_in",
    "circ_out",
    "circ_in_out",
    "elastic_in",
    "elastic_out",
    "elastic",
    "elastic_in_out",
    "back_in",
    "back_out",
    "back_in_out",
    "bounce_in",
    "bounce_out",
    "bounce_in_out",
)

__all__ = [
    # Core
    "setup",
    "Scene",
    "Text",
    "Image",
    "Video",
    "Shape",
    "Renderer",
    "Audio",
    "AudioMixer",
    # Transitions
    "Transition",
    "transition",
    # Effects
    "BlurEffect",
    "GlowEffect",
    "ShadowEffect",
    "ColorAdjustEffect",
    "ChromaKeyEffect",
    "NoiseEffect",
    # Geometry
    "Rectangle",
    "Circle",
    "Polygon",
    # Animation
    "Animation",
    "AnimationSequence",
    # Professional Color Grading
    "ColorGrading",
    "ColorCurves",
    "LUT",
    # Masking
    "Mask",
    "RectangleMask",
    "EllipseMask",
    # Motion Tracking
    "MotionTracker",
    "Stabilizer",
    # Professional Audio
    "Equalizer",
    "Compressor",
    "Reverb",
    "Limiter",
    # Speed Ramping
    "SpeedRamp",
    "TimeRemap",
    # Blend Modes
    "BlendModeEffect",
    # Live Streaming
    "StreamingConfig",
    "Streamer",
    "RTMPStreamer",
    "create_rtmp_url",
    "validate_rtmp_url",
    # Multi-Camera
    "CameraAngle",
    "MultiCamSequence",
    "MultiCamEditor",
    # AI Asset Generation
    "AIGenerator",
    "ImageGenerator",
    "VideoGenerator",
    "AudioGenerator",
    "TextGenerator",
    "generate_asset",
    # Utilities
    "wait",
    "available_resolutions",
    "available_output_formats",
    "available_easings",
    "point_distance",
    "point_angle",
    "rotate_point",
    "detect_beats_energy",
    "calculate_bpm",
    "frames_to_seconds",
    "seconds_to_frames",
    "format_time",
    "parse_time",
    "lerp",
    "clamp",
    "map_range",
    "smoothstep",
    "hex_to_rgb",
    "rgb_to_hex",
    "rgb_to_hsl",
    "hsl_to_rgb",
    "bezier",
    "remap",
    "normalize",
    "denormalize",
    # Easing functions (via utils module)
    "ease",
]

# Global project configuration
_project_config = None

def setup(res: str = "1080p", fps: int = 60):
    """Configure global project settings.
    
    Args:
        res: Resolution preset ("480p", "720p", "1080p", "1440p", "2K", "4K", "8K", "1:1", "4:5", "9:16") or custom "WIDTHxHEIGHT"
        fps: Frames per second
        
    Returns:
        ProjectConfig object
    """
    global _project_config
    config = setup_project(res, fps)
    _project_config = SimpleNamespace(
        resolution=res,
        fps=config["fps"],
        width=config["width"],
        height=config["height"],
        aspect_ratio=round(config["width"] / config["height"], 4),
    )
    return _project_config

def scene(name: str):
    """Create a new scene context manager.
    
    Args:
        name: Scene name
        
    Returns:
        Scene object
    """
    return Scene(name)

def wait(duration: float):
    """Wait for specified duration.
    
    Args:
        duration: Duration in seconds
    """
    if duration < 0:
        raise ValueError("wait duration must be non-negative")
    _time.sleep(duration)

def transition(effect: str, duration: float = 0.5, easing: str = "ease_in_out"):
    """Apply a transition effect between scenes.
    
    Args:
        effect: Transition type ("fade", "slide_left", "slide_right", etc.)
        duration: Transition duration in seconds
        easing: Easing function name
    """
    t = Transition(effect, duration)
    t.with_easing(easing)
    return t

def ease(ease_type: str, t: float) -> float:
    """Apply easing function to normalized time value (0-1).
    
    Args:
        ease_type: Easing function name (e.g., "ease_in", "elastic_out")
        t: Normalized time value (0.0 to 1.0)
        
    Returns:
        Eased value
    """
    t = max(0.0, min(1.0, t))

    if ease_type == "linear":
        return t
    if ease_type in {"ease_in", "quad_in"}:
        return t * t
    if ease_type in {"ease_out", "quad_out"}:
        return 1.0 - (1.0 - t) * (1.0 - t)
    if ease_type in {"ease_in_out", "quad_in_out"}:
        if t < 0.5:
            return 2.0 * t * t
        return 1.0 - 2.0 * (1.0 - t) * (1.0 - t)
    if ease_type == "cubic_in":
        return t * t * t
    if ease_type == "cubic_out":
        return 1.0 - (1.0 - t) ** 3
    if ease_type == "cubic_in_out":
        if t < 0.5:
            return 4.0 * t * t * t
        return 1.0 - 4.0 * (1.0 - t) ** 3
    if ease_type == "sine_in":
        return 1.0 - math.cos((t * math.pi) / 2.0)
    if ease_type == "sine_out":
        return math.sin((t * math.pi) / 2.0)
    if ease_type == "sine_in_out":
        return -(math.cos(math.pi * t) - 1.0) / 2.0
    if ease_type in {"elastic_out", "elastic"}:
        if t == 0.0 or t == 1.0:
            return t
        c4 = (2.0 * math.pi) / 3.0
        return (2.0 ** (-10.0 * t)) * math.sin((t * 10.0 - 0.75) * c4) + 1.0
    if ease_type == "bounce_out":
        return _bounce_out(t)
    if ease_type == "bounce_in":
        return 1.0 - _bounce_out(1.0 - t)
    if ease_type == "bounce_in_out":
        if t < 0.5:
            return (1.0 - _bounce_out(1.0 - 2.0 * t)) / 2.0
        return (1.0 + _bounce_out(2.0 * t - 1.0)) / 2.0

    raise ValueError(f"unknown easing function: {ease_type}")


def available_resolutions():
    """Return supported resolution presets and their dimensions."""
    return dict(RESOLUTION_PRESETS)


def available_output_formats():
    """Return supported output format identifiers."""
    return list(OUTPUT_FORMATS)


def available_easings():
    """Return supported easing function names."""
    return list(EASING_FUNCTIONS)


def _bounce_out(t: float) -> float:
    n1 = 7.5625
    d1 = 2.75

    if t < 1.0 / d1:
        return n1 * t * t
    if t < 2.0 / d1:
        t -= 1.5 / d1
        return n1 * t * t + 0.75
    if t < 2.5 / d1:
        t -= 2.25 / d1
        return n1 * t * t + 0.9375

    t -= 2.625 / d1
    return n1 * t * t + 0.984375


from .utils import (
    bezier,
    clamp,
    denormalize,
    hex_to_rgb,
    hsl_to_rgb,
    lerp,
    map_range,
    normalize,
    remap,
    rgb_to_hex,
    rgb_to_hsl,
    smoothstep,
)
