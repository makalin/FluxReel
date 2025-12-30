"""
Utility functions for FluxReel
"""

from typing import Tuple, List
import math

def lerp(start: float, end: float, t: float) -> float:
    """Linear interpolation between two values.
    
    Args:
        start: Start value
        end: End value
        t: Interpolation factor (0.0 to 1.0)
        
    Returns:
        Interpolated value
    """
    return start + (end - start) * t

def clamp(value: float, min_val: float, max_val: float) -> float:
    """Clamp value between min and max.
    
    Args:
        value: Value to clamp
        min_val: Minimum value
        max_val: Maximum value
        
    Returns:
        Clamped value
    """
    return max(min_val, min(value, max_val))

def map_range(value: float, in_min: float, in_max: float, out_min: float, out_max: float) -> float:
    """Map value from one range to another.
    
    Args:
        value: Input value
        in_min: Input range minimum
        in_max: Input range maximum
        out_min: Output range minimum
        out_max: Output range maximum
        
    Returns:
        Mapped value
    """
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min

def smoothstep(edge0: float, edge1: float, x: float) -> float:
    """Smooth step interpolation.
    
    Args:
        edge0: Lower edge
        edge1: Upper edge
        x: Input value
        
    Returns:
        Smooth stepped value
    """
    t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0)
    return t * t * (3.0 - 2.0 * t)

def hex_to_rgb(hex_color: str) -> Tuple[int, int, int]:
    """Convert hex color to RGB tuple.
    
    Args:
        hex_color: Hex color string (e.g., "#FF0000")
        
    Returns:
        (R, G, B) tuple
    """
    hex_color = hex_color.lstrip('#')
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

def rgb_to_hex(r: int, g: int, b: int) -> str:
    """Convert RGB tuple to hex color.
    
    Args:
        r: Red component (0-255)
        g: Green component (0-255)
        b: Blue component (0-255)
        
    Returns:
        Hex color string
    """
    return f"#{r:02x}{g:02x}{b:02x}"

def rgb_to_hsl(r: int, g: int, b: int) -> Tuple[float, float, float]:
    """Convert RGB to HSL.
    
    Args:
        r: Red component (0-255)
        g: Green component (0-255)
        b: Blue component (0-255)
        
    Returns:
        (H, S, L) tuple where H is 0-360, S and L are 0-1
    """
    r, g, b = r / 255.0, g / 255.0, b / 255.0
    max_val = max(r, g, b)
    min_val = min(r, g, b)
    delta = max_val - min_val
    
    l = (max_val + min_val) / 2.0
    
    if delta == 0:
        h = s = 0.0
    else:
        s = delta / (1.0 - abs(2.0 * l - 1.0))
        if max_val == r:
            h = ((g - b) / delta) % 6.0
        elif max_val == g:
            h = (b - r) / delta + 2.0
        else:
            h = (r - g) / delta + 4.0
        h *= 60.0
    
    return (h, s, l)

def hsl_to_rgb(h: float, s: float, l: float) -> Tuple[int, int, int]:
    """Convert HSL to RGB.
    
    Args:
        h: Hue (0-360)
        s: Saturation (0-1)
        l: Lightness (0-1)
        
    Returns:
        (R, G, B) tuple
    """
    c = (1.0 - abs(2.0 * l - 1.0)) * s
    x = c * (1.0 - abs((h / 60.0) % 2.0 - 1.0))
    m = l - c / 2.0
    
    if h < 60:
        r, g, b = c, x, 0
    elif h < 120:
        r, g, b = x, c, 0
    elif h < 180:
        r, g, b = 0, c, x
    elif h < 240:
        r, g, b = 0, x, c
    elif h < 300:
        r, g, b = x, 0, c
    else:
        r, g, b = c, 0, x
    
    return (
        int((r + m) * 255),
        int((g + m) * 255),
        int((b + m) * 255)
    )

def bezier(t: float, p0: float, p1: float, p2: float, p3: float) -> float:
    """Cubic Bezier interpolation.
    
    Args:
        t: Parameter (0.0 to 1.0)
        p0: Start point
        p1: First control point
        p2: Second control point
        p3: End point
        
    Returns:
        Interpolated value
    """
    u = 1.0 - t
    tt = t * t
    uu = u * u
    uuu = uu * u
    ttt = tt * t
    
    return uuu * p0 + 3 * uu * t * p1 + 3 * u * tt * p2 + ttt * p3

def remap(value: float, old_min: float, old_max: float, new_min: float, new_max: float) -> float:
    """Remap value from old range to new range (alias for map_range)."""
    return map_range(value, old_min, old_max, new_min, new_max)

def normalize(value: float, min_val: float, max_val: float) -> float:
    """Normalize value to 0-1 range.
    
    Args:
        value: Value to normalize
        min_val: Minimum value of range
        max_val: Maximum value of range
        
    Returns:
        Normalized value (0.0 to 1.0)
    """
    if max_val == min_val:
        return 0.0
    return (value - min_val) / (max_val - min_val)

def denormalize(value: float, min_val: float, max_val: float) -> float:
    """Denormalize value from 0-1 range to specified range.
    
    Args:
        value: Normalized value (0.0 to 1.0)
        min_val: Minimum value of target range
        max_val: Maximum value of target range
        
    Returns:
        Denormalized value
    """
    return min_val + value * (max_val - min_val)

