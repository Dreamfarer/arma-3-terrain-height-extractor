# Arma 3 Terrain Height Extractor
A dynamic link library (DLL) for extracting terrain height data from Arma 3, written in Rust using Brett Mayson's [arma-rs](https://github.com/brettmayson/arma-rs) crate.

## Installation
1. Navigate to the root directory and compile the extension using the following command:
    ```sh
    cargo build --release
    ```
2. Place the `extractor_x64.dll` file inside Arma 3's installation folder: `...\steamapps\common\Arma 3`.
3. Launch Arma 3 with Battleye turned **off** (otherwise, the extension cannot be loaded).
4. Open the Editor, place a player character, start the game, and copy the content from `extractor.sqf` into Arma's debug console (accessible from the pause menu). This will initiate the extraction process.

## Disclaimer
This tool has been developed and tested exclusively on the 64-bit version of Arma 3 running on Windows 11. Furthermore, for the [freeExtension](https://community.bistudio.com/wiki/freeExtension) scripting command to have any effect, you have to be in the [dev](https://dev.arma3.com/dev-branch) branch of Arma 3.