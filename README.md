# Arma 3 Terrain Height Extractor
A dynamic link library (DLL) for extracting terrain height data from Arma 3 into a `.csv` file, written in Rust using Brett Mayson's [arma-rs](https://github.com/brettmayson/arma-rs) crate. The `extractor.sqf` script contains an implementation to retrieve terrain height data for the entire map at 5-meter intervals.

## Download Terrain Height Data
If you prefer not to generate the terrain height data yourself, visit the [Releases](https://github.com/Dreamfarer/arma-3-terrain-height-extractor/releases) page to directly download a pre-extracted `.csv` containing the height data for a specific map.

## Generate Terrain Height Data
If you choose to extract the terrain height data yourself, you have two options. You can either download the binary from [Releases](https://github.com/Dreamfarer/arma-3-terrain-height-extractor/releases) (only `x64` is supported), or you can clone the repository, navigate to the root directory, and then compile the extension yourself using: `cargo build --release`
1. Place the `extractor_x64.dll` file inside your Arma 3 installation folder: `...\steamapps\common\Arma 3`
1. Launch Arma 3 with Battleye turned **off** (otherwise, the extension cannot be loaded).
1. Open the Editor, place a player character, start the game, and copy the contents of `extractor.sqf` into Arma's debug console (accessible from the pause menu). This will initiate the (lengthy) extraction process.

## Further Help and Contribution
Don't hesitate to open an [Issue](https://github.com/Dreamfarer/arma-3-terrain-height-extractor/issues) for any questions, requests, or contributions. If you feel comfortable making changes directly, feel free to fork this repository, implement your changes, and submit a pull request.

## Disclaimer
This tool has been developed and tested exclusively on the 64-bit version of Arma 3 running on Windows 11. The extraction process can be time-consuming (e.g., it took 45 minutes to extract Altis at 5-meter intervals). Lastly, for the [freeExtension](https://community.bistudio.com/wiki/freeExtension) scripting command to work, you must be on the [dev](https://dev.arma3.com/dev-branch) branch of Arma 3.