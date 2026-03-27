use clap::Parser;

use crate::util::types::{Float, Uint};

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,

    #[arg(short, long, default_value_t = 2)]
    pub scene: Uint,

    #[arg(short, long, default_value_t = 400)]
    pub width: Uint,

    #[arg(short, long, default_value_t = 16.0/9.0)]
    pub aspect: Float,

    #[arg(long, default_value_t = 100)]
    pub samples: Uint,

    #[arg(short, long, default_value_t = 10)]
    pub depth: Uint,

    #[arg(short, long, default_value_t = false)]
    pub multithreading: bool,

    #[arg(long, default_value_t = false)]
    pub importance: bool,

    #[arg(long, default_value_t = false)]
    pub gpu: bool,

    #[arg(long, default_value_t = 1)]
    pub dispatch_size: u32,

    #[arg(long, default_value_t = 1)]
    pub sampler: Uint,
}

pub fn print_config(args: &Args) {
    eprintln!("Render settings:");
    eprintln!("  interactive         = {}", args.interactive);
    eprintln!("  scene index         = {}", args.scene);
    eprintln!("  image width         = {}", args.width);
    eprintln!("  aspect ratio        = {}", args.aspect);
    eprintln!("  samples-per-pixel   = {}", args.samples);
    eprintln!("  max depth           = {}", args.depth);
    eprintln!("  dispatch size       = {}", args.dispatch_size);
    eprintln!("  multithreading      = {}", args.multithreading);
    eprintln!("  importance sampling = {}", args.importance);
    eprintln!("  GPU rendering       = {}", args.gpu);
    eprintln!(
        "  sampler             = {}",
        if args.sampler == 2 { "stratified" } else { "random" }
    );
}
