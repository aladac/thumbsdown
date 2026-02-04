mod cli;
mod error;
mod grid;
mod header;
mod video;

use std::process;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::Args::parse();
    cli::validate(&args)?;
    video::check_dependencies()?;

    if args.output.exists() && args.force {
        if args.verbose {
            eprintln!("Output file already exists: deleting");
        }
        std::fs::remove_file(&args.output)?;
    }

    let temp_dir = match &args.temp {
        Some(dir) => tempfile::Builder::new()
            .prefix("thumbsdown_")
            .tempdir_in(dir)?,
        None => tempfile::Builder::new().prefix("thumbsdown_").tempdir()?,
    };

    if args.verbose {
        eprintln!("Temp directory: {}", temp_dir.path().display());
    }

    let info = video::probe(&args.video)?;
    if args.verbose {
        eprintln!(
            "Video: {} ({}x{}, {}, {:.2} fps, {:.1}s)",
            info.filename, info.width, info.height, info.codec, info.fps, info.duration
        );
    }

    let step = (info.duration - args.start as f64) / args.thumbs as f64;

    let pb = ProgressBar::new(args.thumbs as u64);
    if let Ok(style) = ProgressStyle::default_bar().template("{bar:40} {pos}/{len} frames") {
        pb.set_style(style.progress_chars("=> "));
    }

    let mut thumbnails = Vec::with_capacity(args.thumbs as usize);
    for i in 0..args.thumbs {
        let time = args.start as f64 + (i as f64 * step);
        if time > info.duration {
            break;
        }

        let frame_path = temp_dir.path().join(format!("frame-{i:08}.png"));
        video::capture_frame(&args.video, time, &frame_path)?;

        let thumb = grid::process_thumbnail(&frame_path, args.width, 10)?;
        thumbnails.push(thumb);

        if args.verbose {
            eprintln!("Captured frame at {time:.1}s -> {}", frame_path.display());
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    let grid_image = grid::compose_grid(&thumbnails, args.columns);
    let header_image = header::render_header(&info)?;
    let final_image = grid::assemble_final(&header_image, &grid_image);

    final_image.save(&args.output)?;

    if args.verbose {
        eprintln!("Saved to {}", args.output.display());
        eprintln!("DONE.");
    }

    Ok(())
}
