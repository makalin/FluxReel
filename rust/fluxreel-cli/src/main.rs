use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "fluxreel")]
#[command(about = "FluxReel - Code Less. Render Faster.", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new FluxReel project
    New {
        /// Project name
        name: String,
        /// Template to use
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Render a script to video
    Render {
        /// Input script file (.flux or .py)
        input: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Resolution override
        #[arg(short, long)]
        resolution: Option<String>,
        /// FPS override
        #[arg(short = 'f', long)]
        fps: Option<u32>,
        /// Quality preset (low, medium, high, ultra)
        #[arg(short, long)]
        quality: Option<String>,
    },
    /// Launch FluxReel Studio GUI
    Studio {
        /// Optional project path
        project: Option<PathBuf>,
    },
    /// Generate code from natural language prompt
    Copilot {
        /// Natural language description
        prompt: String,
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Extract audio from video
    ExtractAudio {
        /// Input video file
        input: PathBuf,
        /// Output audio file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert video format
    Convert {
        /// Input file
        input: PathBuf,
        /// Output file
        output: PathBuf,
        /// Output format
        #[arg(short, long)]
        format: Option<String>,
    },
    /// Analyze audio file
    AnalyzeAudio {
        /// Input audio file
        input: PathBuf,
        /// Detect beats
        #[arg(short, long)]
        beats: bool,
        /// Calculate BPM
        #[arg(long)]
        bpm: bool,
    },
    /// List available templates
    Templates,
    /// List built-in capabilities such as resolutions, formats, and easing functions
    List {
        /// Category to list: all, resolutions, formats, easings, features
        #[arg(default_value = "all")]
        category: String,
    },
    /// Inspect a FluxReel or Python script and summarize its contents
    Inspect {
        /// Script file to inspect
        input: PathBuf,
    },
    /// Validate script syntax
    Validate {
        /// Script file to validate
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {
            create_new_project(&name, template.as_deref())?;
        }
        Commands::Render {
            input,
            output,
            resolution,
            fps,
            quality,
        } => {
            render_project(
                &input,
                output.as_deref(),
                resolution.as_deref(),
                fps,
                quality.as_deref(),
            )?;
        }
        Commands::Studio { project } => {
            launch_studio(project.as_deref())?;
        }
        Commands::Copilot { prompt, output } => {
            generate_code(&prompt, output.as_deref())?;
        }
        Commands::ExtractAudio { input, output } => {
            extract_audio(&input, output.as_deref())?;
        }
        Commands::Convert {
            input,
            output,
            format,
        } => {
            convert_video(&input, &output, format.as_deref())?;
        }
        Commands::AnalyzeAudio { input, beats, bpm } => {
            analyze_audio(&input, beats, bpm)?;
        }
        Commands::Templates => {
            list_templates()?;
        }
        Commands::List { category } => {
            list_capabilities(&category)?;
        }
        Commands::Inspect { input } => {
            inspect_script(&input)?;
        }
        Commands::Validate { input } => {
            validate_script(&input)?;
        }
    }

    Ok(())
}

fn create_new_project(name: &str, template: Option<&str>) -> Result<()> {
    use std::fs;

    fs::create_dir_all(name)?;
    fs::create_dir_all(format!("{}/assets", name))?;

    let main_flux = match template {
        Some("minimal") => "from fluxreel import Scene, Text\n\nsetup(res=\"1080p\", fps=60)\n\n# Create scene\nmain_scene = Scene(\"Main\")\ntitle = Text(\"Hello FluxReel\", size=100, color=\"#FFFFFF\")\ntitle.align(\"center\")\n".to_string(),
        Some("vertical") => "from fluxreel import Scene, Text, Image\n\nsetup(res=\"9:16\", fps=60)\n\n# Create scene\nmain_scene = Scene(\"Main\")\nbg = Image(\"assets/background.jpg\")\ntitle = Text(\"FluxReel\", size=120, color=\"#FFFFFF\")\ntitle.align(\"center\")\n".to_string(),
        _ => "from fluxreel import Scene, Text, Image, Audio\n\n# Global Settings\nsetup(res=\"1080p\", fps=60)\n\n# Create intro scene\nintro_scene = Scene(\"Intro\")\nbg = Image(\"assets/background.jpg\")\ntitle = Text(\"Hello FluxReel\", size=100, color=\"#FFFFFF\")\ntitle.align(\"center\")\ntitle.fade_in(duration=1.0)\ntitle.scale(start=0.5, end=1.0, ease=\"elastic_out\")\n\n# Audio Sync\nvoice = Audio(\"assets/intro.mp3\")\nvoice.play()\n\n# Create main scene\nmain_scene = Scene(\"Main\")\ntransition(effect=\"slide_left\", duration=0.5)\n# ... your main content here\n".to_string(),
    };

    fs::write(format!("{}/main.flux", name), main_flux)?;
    println!("Created new project: {}", name);
    Ok(())
}

fn render_project(
    input: &std::path::Path,
    output: Option<&std::path::Path>,
    resolution: Option<&str>,
    fps: Option<u32>,
    quality: Option<&str>,
) -> Result<()> {
    let output_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| input.with_extension("mp4"));

    println!("Rendering {} to {}", input.display(), output_path.display());
    if let Some(res) = resolution {
        println!("  Resolution: {}", res);
    }
    if let Some(f) = fps {
        println!("  FPS: {}", f);
    }
    if let Some(q) = quality {
        println!("  Quality: {}", q);
    }
    // Actual rendering would be implemented here
    Ok(())
}

fn launch_studio(project: Option<&std::path::Path>) -> Result<()> {
    println!("Launching FluxReel Studio...");
    if let Some(p) = project {
        println!("Opening project: {}", p.display());
    }
    // Studio GUI would be launched here
    Ok(())
}

fn generate_code(prompt: &str, output: Option<&std::path::Path>) -> Result<()> {
    println!("Generating code from prompt: {}", prompt);
    // AI code generation would be implemented here
    if let Some(out) = output {
        println!("Writing to: {}", out.display());
    }
    Ok(())
}

fn extract_audio(input: &std::path::Path, output: Option<&std::path::Path>) -> Result<()> {
    let output_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| input.with_extension("mp3"));

    println!(
        "Extracting audio from {} to {}",
        input.display(),
        output_path.display()
    );
    // Audio extraction would be implemented here
    Ok(())
}

fn convert_video(
    input: &std::path::Path,
    output: &std::path::Path,
    format: Option<&str>,
) -> Result<()> {
    println!("Converting {} to {}", input.display(), output.display());
    if let Some(f) = format {
        println!("  Format: {}", f);
    }
    // Video conversion would be implemented here
    Ok(())
}

fn analyze_audio(input: &std::path::Path, beats: bool, bpm: bool) -> Result<()> {
    println!("Analyzing audio: {}", input.display());
    if beats {
        println!("  Detecting beats...");
    }
    if bpm {
        println!("  Calculating BPM...");
    }
    // Audio analysis would be implemented here
    Ok(())
}

fn list_templates() -> Result<()> {
    println!("Available templates:");
    println!("  - default  : Full-featured template with intro scene");
    println!("  - minimal  : Minimal template with basic setup");
    println!("  - vertical : Template optimized for 9:16 format");
    Ok(())
}

fn list_capabilities(category: &str) -> Result<()> {
    match category {
        "all" => {
            print_section("Resolution Presets", RESOLUTION_PRESETS);
            print_section("Output Formats", OUTPUT_FORMATS);
            print_section("Easing Functions", EASING_FUNCTIONS);
            print_section("Feature Modules", FEATURE_MODULES);
        }
        "resolutions" => print_section("Resolution Presets", RESOLUTION_PRESETS),
        "formats" => print_section("Output Formats", OUTPUT_FORMATS),
        "easings" => print_section("Easing Functions", EASING_FUNCTIONS),
        "features" => print_section("Feature Modules", FEATURE_MODULES),
        _ => bail!(
            "Unknown category '{category}'. Use: all, resolutions, formats, easings, features"
        ),
    }
    Ok(())
}

fn inspect_script(input: &Path) -> Result<()> {
    validate_supported_script(input)?;

    let source = fs::read_to_string(input)?;
    let line_count = source.lines().count();
    let scene_count = source.matches("Scene(").count();
    let transition_count =
        source.matches("transition(").count() + source.matches("Transition(").count();
    let audio_refs = source.matches("Audio(").count();
    let image_refs = source.matches("Image(").count();
    let video_refs = source.matches("Video(").count();
    let ai_refs = source.matches("Generator(").count() + source.matches("generate_asset(").count();

    println!("Script summary: {}", input.display());
    println!("  Size: {} bytes", source.len());
    println!("  Lines: {}", line_count);
    println!("  Scenes: {}", scene_count);
    println!("  Transitions: {}", transition_count);
    println!("  Audio refs: {}", audio_refs);
    println!("  Image refs: {}", image_refs);
    println!("  Video refs: {}", video_refs);
    println!("  AI helpers: {}", ai_refs);

    if source.contains("setup(") {
        println!("  Setup: present");
    } else {
        println!("  Setup: missing");
    }

    Ok(())
}

fn validate_script(input: &Path) -> Result<()> {
    println!("Validating script: {}", input.display());
    validate_supported_script(input)?;

    let source = fs::read_to_string(input)?;
    if source.trim().is_empty() {
        bail!("Script is empty");
    }

    ensure_balanced(&source, '(', ')', "parentheses")?;
    ensure_balanced(&source, '[', ']', "brackets")?;
    ensure_balanced(&source, '{', '}', "braces")?;

    if !source.contains("setup(") {
        bail!("Missing setup(...) call");
    }
    if !source.contains("Scene(") {
        bail!("Missing Scene(...) definition");
    }

    println!("✓ Script is valid");
    Ok(())
}

const RESOLUTION_PRESETS: &[&str] = &[
    "480p (854x480)",
    "720p (1280x720)",
    "1080p (1920x1080)",
    "1440p (2560x1440)",
    "2K (2048x1080)",
    "4K (3840x2160)",
    "8K (7680x4320)",
    "1:1 (1080x1080)",
    "4:5 (1080x1350)",
    "9:16 (1080x1920)",
];
const OUTPUT_FORMATS: &[&str] = &["mp4", "mov", "gif", "webp", "png_sequence"];
const FEATURE_MODULES: &[&str] = &[
    "transitions",
    "effects",
    "color_grading",
    "masking",
    "motion_tracking",
    "audio_pro",
    "speed_ramping",
    "blend_modes",
    "streaming",
    "multicam",
    "ai_assets",
];
const EASING_FUNCTIONS: &[&str] = &[
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
];

fn print_section(title: &str, items: &[&str]) {
    println!("{title}:");
    for item in items {
        println!("  - {item}");
    }
}

fn validate_supported_script(input: &Path) -> Result<()> {
    if !input.exists() {
        bail!("Script not found: {}", input.display());
    }

    match input.extension().and_then(|ext| ext.to_str()) {
        Some("flux") | Some("py") => Ok(()),
        _ => bail!("Unsupported script type. Expected .flux or .py"),
    }
}

fn ensure_balanced(source: &str, open: char, close: char, label: &str) -> Result<()> {
    let mut depth = 0_i32;
    for ch in source.chars() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
        }

        if depth < 0 {
            bail!("Unbalanced {label}");
        }
    }

    if depth != 0 {
        bail!("Unbalanced {label}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_pairs_pass_validation() {
        assert!(ensure_balanced("setup(res='1080p')", '(', ')', "parentheses").is_ok());
    }

    #[test]
    fn unbalanced_pairs_fail_validation() {
        assert!(ensure_balanced("setup(res='1080p'", '(', ')', "parentheses").is_err());
    }

    #[test]
    fn supported_extensions_are_detected() {
        let temp_path = std::env::temp_dir().join("fluxreel_cli_validation_test.flux");
        fs::write(&temp_path, "setup(res='1080p', fps=60)\nScene('Main')\n").unwrap();

        assert!(validate_supported_script(&temp_path).is_ok());
        assert!(validate_supported_script(Path::new("example.txt")).is_err());

        fs::remove_file(temp_path).unwrap();
    }
}
