"""
FluxReel - Code Less. Render Faster.

A high-performance, programmable video engine designed for developers.
"""

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
)

__version__ = "0.1.0"
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
    # Utilities
    "wait",
    "point_distance",
    "point_angle",
    "rotate_point",
    "detect_beats_energy",
    "calculate_bpm",
    "frames_to_seconds",
    "seconds_to_frames",
    "format_time",
    "parse_time",
    # Easing functions (via utils module)
    "ease",
]

# Global project configuration
_project_config = None

def setup(res: str = "1080p", fps: int = 60):
    """Configure global project settings.
    
    Args:
        res: Resolution preset ("4K", "1080p", "720p", "9:16") or custom "WIDTHxHEIGHT"
        fps: Frames per second
        
    Returns:
        ProjectConfig object
    """
    global _project_config
    _project_config = setup_project(res, fps)
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
    pass

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
    # This would call the Rust easing function
    return t  # Placeholder

