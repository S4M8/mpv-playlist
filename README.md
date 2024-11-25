# Documentation for Episode Playlist Generator

## Overview

The **Episode Playlist Generator** is a command-line utility that organizes video files in a directory into a playlist, sorted based on filenames containing season and episode information (e.g., `S1E1`, `S1E2`, etc.). It supports playback through `mpv` or generates scripts to manually start the playlist.

## Features

1. **Automatic Playlist Creation**:
   - Creates a playlist in the same directory as the video files.
   - Playlist filename is derived from the directory name (e.g., for `./MyShows`, it generates `MyShows_playlist.txt`).

2. **Video Playback**:
   - Attempts to run `mpv` directly to play the playlist.
   - If `mpv` is unavailable, generates a script for manual execution.

3. **Supported Platforms**:
   - Linux, macOS, and Windows.

4. **Supported Video Formats**:
   - `.mp4`, `.mkv`, `.avi`, `.mov`, `.flv`, `.wmv`.

---

## Requirements

1. **mpv Media Player**:
   The application requires the `mpv` media player to play the playlist. Ensure that `mpv` is installed and available in your system's `PATH`.

   - **Installing mpv**:
     - On Linux: `sudo apt install mpv` (Debian/Ubuntu) or `sudo dnf install mpv` (Fedora).
     - On macOS: Use [Homebrew](https://brew.sh): `brew install mpv`.
     - On Windows: Download from [mpv.io](https://mpv.io/) and add its location to the `PATH`.

   - **Verifying mpv Installation**:
     Run the following command in your terminal or command prompt:
     ```bash
     mpv --version
     ```

2. **Rust Toolchain**:
   - The utility is written in Rust. To compile it, ensure the Rust toolchain is installed. Use [rustup](https://rustup.rs/) for installation.

   - **Verifying Rust Installation**:
     Run the following command:
     ```bash
     rustc --version
     ```

3. **File Naming Convention**:
   - Filenames must include season and episode information in the format `S<SeasonNumber>E<EpisodeNumber>`.
   - Example valid filenames:
     - `S1E1.mp4`
     - `S1E2.mkv`

---

## Usage Instructions

### Compilation

1. **Clone or Download the Repository**:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```

2. **Build the Application**:
   ```bash
   cargo build --release
   ```
   The compiled binary will be located in `./target/release`.

---

### Running the Program

```bash
./target/release/mpv-playlist --directory <video_directory>
```

#### Arguments:
- `--directory <video_directory>`:
  The directory containing the video files. Defaults to `./files`.

#### Example:
```bash
./target/release/mpv-playlist --directory ./MyShows
```

### What Happens?

1. **Playlist Creation**:
   - A playlist file (e.g., `MyShows_playlist.txt`) is created in the specified directory.

2. **Video Playback**:
   - The application will attempt to launch `mpv` and play the playlist.

3. **Script Generation**:
   - If `mpv` cannot be launched, a script file (`play_playlist.sh` on Linux/macOS or `play_playlist.bat` on Windows) is generated in the directory.

---

### Playing the Playlist

#### Method 1: Direct Playback
Run the program to play the playlist immediately:
```bash
./target/release/mpv-playlist --directory ./MyShows
```

#### Method 2: Using Generated Script
1. Locate the generated script:
   - **Linux/macOS**: `play_playlist.sh`
   - **Windows**: `play_playlist.bat`
2. Double-click the script to launch `mpv` and play the playlist.

---

## Additional Notes

1. **Error Handling**:
   - If the directory does not exist or contains no video files, the program will terminate with an appropriate error message.

2. **File Sorting**:
   - Video files are sorted by season and episode numbers.
   - Files without a valid `S<Season>E<Episode>` pattern are ignored.

3. **Script Permissions** (Linux/macOS):
   - Ensure the generated script has executable permissions.
     ```bash
     chmod +x play_playlist.sh
     ```

---

## Troubleshooting

### mpv Not Found
Ensure `mpv` is installed and added to your `PATH`.
- **Linux/macOS**: Add the installation directory to `$PATH`.
- **Windows**: Add the directory containing `mpv.exe` to the system `PATH`.

### Invalid Filenames
Rename files to include `S<Season>E<Episode>` format (e.g., `S1E1.mp4`). Use batch renaming tools if needed.

### mpv-playlist Not Found
Ensure `mvp-playlist` is added to your `PATH`
- **Linux/macOS**: Add the installation directory to `$PATH`.
- **Windows**: Add the directory containing `mpv.exe` to the system `PATH`.

---

## License
This project is licensed under the MIT License.

For additional questions or support, refer to the [project repository](<repository-url>).
