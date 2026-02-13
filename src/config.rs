use std::process;

pub struct Config {
    pub audio_path: String,
    pub use_visualizer: bool,
    pub num_bars: usize,
    pub smoothing: f32,
    pub bass_boost: f32,
    pub volume_step: f32,
    pub seek_step: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio_path: String::new(),
            use_visualizer: false,
            num_bars: 100,
            smoothing: 0.7,
            bass_boost: 1.5,
            volume_step: 0.05,
            seek_step: 5,
        }
    }
}

impl Config {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut config = Config::default();
        let mut i = 1;

        while i < args.len() {
            match args[i].as_str() {
                "--visualizer" => {
                    config.use_visualizer = true;
                    i += 1;
                }
                "--bars" => {
                    if i + 1 >= args.len() {
                        eprintln!("Error: --bars requires a value");
                        Self::print_usage(&args[0]);
                    }
                    config.num_bars = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: --bars must be a positive integer");
                        Self::print_usage(&args[0]);
                    });
                    i += 2;
                }
                "--smoothing" => {
                    if i + 1 >= args.len() {
                        eprintln!("Error: --smoothing requires a value");
                        Self::print_usage(&args[0]);
                    }
                    config.smoothing = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: --smoothing must be a float between 0.0 and 1.0");
                        Self::print_usage(&args[0]);
                    });
                    config.smoothing = config.smoothing.clamp(0.0, 1.0);
                    i += 2;
                }
                "--bass-boost" => {
                    if i + 1 >= args.len() {
                        eprintln!("Error: --bass-boost requires a value");
                        Self::print_usage(&args[0]);
                    }
                    config.bass_boost = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: --bass-boost must be a float");
                        Self::print_usage(&args[0]);
                    });
                    i += 2;
                }
                "--volume-step" => {
                    if i + 1 >= args.len() {
                        eprintln!("Error: --volume-step requires a value");
                        Self::print_usage(&args[0]);
                    }
                    config.volume_step = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: --volume-step must be a float between 0.0 and 1.0");
                        Self::print_usage(&args[0]);
                    });
                    config.volume_step = config.volume_step.clamp(0.0, 1.0);
                    i += 2;
                }
                "--seek-step" => {
                    if i + 1 >= args.len() {
                        eprintln!("Error: --seek-step requires a value");
                        Self::print_usage(&args[0]);
                    }
                    config.seek_step = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: --seek-step must be an integer");
                        Self::print_usage(&args[0]);
                    });
                    i += 2;
                }
                "--help" | "-h" => {
                    Self::print_usage(&args[0]);
                }
                arg if !arg.starts_with('-') => {
                    if config.audio_path.is_empty() {
                        config.audio_path = arg.to_string();
                    } else {
                        eprintln!("Error: Multiple audio files specified");
                        Self::print_usage(&args[0]);
                    }
                    i += 1;
                }
                _ => {
                    eprintln!("Error: Unknown option '{}'", args[i]);
                    Self::print_usage(&args[0]);
                }
            }
        }

        if config.audio_path.is_empty() {
            eprintln!("Error: No audio file specified");
            Self::print_usage(&args[0]);
        }

        config
    }

    fn print_usage(program: &str) -> ! {
        eprintln!("Usage: {} [OPTIONS] <audio_file>", program);
        eprintln!("\nSupported formats: MP3, WAV, FLAC, OGG, AAC/M4A");
        eprintln!("\nOptions:");
        eprintln!("  --visualizer           Enable live spectrum analyzer");
        eprintln!("  --bars <n>             Number of frequency bars (default: 100)");
        eprintln!("  --smoothing <f>        Smoothing factor 0.0-1.0 (default: 0.7)");
        eprintln!("  --bass-boost <f>       Bass boost multiplier (default: 1.5)");
        eprintln!("  --volume-step <f>      Volume adjustment step (default: 0.05)");
        eprintln!("  --seek-step <n>        Seek step in seconds (default: 5)");
        eprintln!("  -h, --help             Show this help message");
        eprintln!("\nControls:");
        eprintln!("  Space    - Play/pause");
        eprintln!("  Q/Esc    - Quit");
        eprintln!("  ←/→      - Seek backward/forward");
        eprintln!("  ↑/↓      - Volume up/down");
        eprintln!("  R        - Restart");
        process::exit(1);
    }
}
