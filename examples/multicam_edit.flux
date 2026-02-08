from fluxreel import MultiCamEditor, setup

# Setup
setup(res="1080p", fps=60)

# Create multi-camera editor
editor = MultiCamEditor()

# Add camera angles
editor.add_camera("Main Camera", "footage/main_cam.mp4")
editor.add_camera("Close-up", "footage/closeup.mp4")
editor.add_camera("Wide Shot", "footage/wide.mp4")
editor.add_camera("B-Roll", "footage/broll.mp4")

# Auto-sync all cameras by audio
editor.auto_sync()

# Create edit with cuts
editor.cut_to_angle(time=0.0, angle_name="Wide Shot")
editor.cut_to_angle(time=3.0, angle_name="Main Camera")
editor.cut_to_angle(time=8.0, angle_name="Close-up")
editor.cut_to_angle(time=12.0, angle_name="B-Roll")
editor.cut_to_angle(time=15.0, angle_name="Main Camera")

# Render final edit
editor.render("multicam_output.mp4")

