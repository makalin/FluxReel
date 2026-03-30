"""
Basic tests for FluxReel
"""

import pytest
import fluxreel
from fluxreel import (
    Audio,
    Image,
    Renderer,
    Scene,
    Text,
    available_easings,
    available_output_formats,
    available_resolutions,
    ease,
    setup,
)


def test_setup():
    """Test project setup"""
    config = setup(res="1080p", fps=60)
    assert config is not None
    assert config.fps == 60
    assert config.width == 1920
    assert config.height == 1080

    quad_hd = setup(res="1440p", fps=30)
    assert quad_hd.width == 2560
    assert quad_hd.height == 1440


def test_text_node():
    """Test Text node creation"""
    text = Text("Hello World", size=100, color="#FFFFFF")
    assert text.text == "Hello World"
    assert text.size == 100
    assert text.color == "#FFFFFF"


def test_image_node():
    """Test Image node creation"""
    image = Image("assets/test.jpg")
    assert image.path == "assets/test.jpg"


def test_scene():
    """Test Scene creation"""
    scene = Scene("TestScene")
    assert scene.name == "TestScene"


def test_audio_track():
    """Test Audio track creation"""
    audio = Audio("assets/test.mp3")
    assert audio.path == "assets/test.mp3"


def test_available_helpers():
    """Test capability helper functions."""
    assert available_resolutions()["4K"] == (3840, 2160)
    assert available_resolutions()["4:5"] == (1080, 1350)
    assert available_resolutions()["1:1"] == (1080, 1080)
    assert "mp4" in available_output_formats()
    assert "elastic_out" in available_easings()


def test_ease_function():
    """Test easing helper output."""
    assert ease("linear", 0.25) == pytest.approx(0.25)
    assert ease("ease_in", 0.5) == pytest.approx(0.25)
    assert ease("bounce_out", 1.0) == pytest.approx(1.0)


def test_wait_rejects_negative_values():
    """Test wait validation."""
    with pytest.raises(ValueError):
        fluxreel.wait(-0.1)
