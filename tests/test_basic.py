"""
Basic tests for FluxReel
"""

import pytest
from fluxreel import setup, Scene, Text, Image, Audio, Renderer


def test_setup():
    """Test project setup"""
    config = setup(res="1080p", fps=60)
    assert config is not None
    assert config.fps == 60


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

