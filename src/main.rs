use clap::{Arg, Command};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Episode {
    season: u32,
    episode: u32,
    filename: String,
}

fn parse_filename(filename: &str) -> Option<Episode> {
    let re = Regex::new(r"S(\d+)E(\d+)").unwrap();
    if let Some(caps) = re.captures(filename) {
        let season = caps.get(1)?.as_str().parse::<u32>().ok()?;
        let episode = caps.get(2)?.as_str().parse::<u32>().ok()?;
        return Some(Episode {
            season,
            episode,
            filename: filename.to_string(),
        });
    }
    None
}

fn is_video_file(filename: &str) -> bool {
    let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv"];
    if let Some(ext) = Path::new(filename).extension() {
        return video_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str());
    }
    false
}

fn create_playlist_file(dir: &Path, episodes: &Vec<Episode>) -> std::io::Result<PathBuf> {
    let playlist_name = dir.file_name().unwrap().to_string_lossy().to_string() + "_playlist.txt";
    let playlist_path = dir.join(playlist_name);

    let mut file = fs::File::create(&playlist_path)?;
    for episode in episodes {
        writeln!(file, "{}", dir.join(&episode.filename).display())?;
    }

    println!("Playlist file created: {}", playlist_path.display());
    Ok(playlist_path)
}

fn create_execution_script(dir: &Path, playlist_path: &Path) -> std::io::Result<()> {
    let os = std::env::consts::OS;

    let script_name = match os {
        "windows" => "play_playlist.bat",
        _ => "play_playlist.sh",
    };
    let script_path = dir.join(script_name);

    let mut script = fs::File::create(&script_path)?;

    if os == "windows" {
        writeln!(script, "@echo off")?;
        writeln!(script, "mpv --playlist=\"{}\"", playlist_path.display())?;
    } else {
        writeln!(script, "#!/bin/bash")?;
        writeln!(script, "mpv --playlist=\"{}\"", playlist_path.display())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    println!("Execution script created: {}", script_path.display());
    Ok(())
}

fn main() {
    let matches = Command::new("Episode Playlist Generator")
        .version("1.0")
        .author("Matthew Urrea")
        .about("Generates or plays a sorted playlist of video files in a directory")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("The directory containing video files")
                .default_value("./files"),
        )
        .get_matches();

    let input_dir = matches
        .get_one::<String>("directory")
        .expect("Failed to get directory argument");
    let path = Path::new(input_dir);

    if !path.is_dir() {
        eprintln!("The specified directory does not exist: {}", input_dir);
        return;
    }

    // Create playlist
    let mut episodes: Vec<Episode> = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = entry.file_name();
                    if let Some(filename_str) = filename.to_str() {
                        if is_video_file(filename_str) {
                            if let Some(episode) = parse_filename(filename_str) {
                                episodes.push(episode);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read the directory: {}", e);
            return;
        }
    }

    episodes.sort();

    let playlist_path = path.join(format!(
        "{}_playlist.txt",
        path.file_name().unwrap().to_string_lossy()
    ));

    if !playlist_path.exists() {
        if let Err(e) = create_playlist_file(path, &episodes) {
            eprintln!("Failed to create playlist file: {}", e);
            return;
        }
    } else {
        println!("Playlist file already exists: {}", playlist_path.display());
    }

    // Run mpv directly or generate script
    let playlist_action = ProcessCommand::new("mpv")
        .arg("--playlist")
        .arg(&playlist_path)
        .spawn();

    match playlist_action {
        Ok(_) => println!("Playing playlist: {}", playlist_path.display()),
        Err(_) => {
            println!("Failed to launch mpv. Generating execution script...");
            if let Err(e) = create_execution_script(path, &playlist_path) {
                eprintln!("Failed to create execution script: {}", e);
            }
        }
    }
}
