# Rust-Based-3D-Model-Viewer
Simple 3d model viewer powered by Raylib using Rust. This was a project I made while learing Rust with Raylib. I will not be making changes to it, feel free to modify and improve on it.


What It Is:
-A lightweight, high-performance 3D model viewer built with Rust and Raylib. Designed for game developers and artists to quickly preview and inspect 3D models with real-time rendering. 
-A demo of Raylib with Rust bindings.

What It's Not:
-A game engine.
-Meant for real-world use unless improved upon.

Features

- Leverages Raylib's hardware-accelerated graphics for smooth performance
- Load and view OBJ, GLTF/GLB, and other common 3D model formats. (only .obj files were tested).
- View models with textures and material information (must be in same folder as model and mtl file must have the correct path. (It does not automatcially find material files).

How To Use:
-Create new folder under "assets" with the model files
--funko and spider are provided for testing purposes, I am not the creator of them.
-On line 30 of "main.rs", add model path. 
    --  let mut model_path = PathBuf::from("./src/assets/funko/funko.obj");
-Run application.

Note:
-The program does not automatically get materials. It reads the .obj file for the path provided to the .mtl file. So if materials are not loaded, check the .obj file path. 
    --For example, line 3 of spider.obj has mtllib Only_Spider_with_Animations_Export.mtl. 
    --I recommend keeping the .mtl file in the same directory as the .obj file.


Prerequisites

- Rust 1.70 or later
- Raylib development libraries

Installation

```bash
git clone https://github.com/yourusername/3d-model-viewer.git
cd 3d-model-viewer
cargo run --release
```

Usage

```bash
cargo run --release -- path/to/model.obj
```


## Project Structure

```
src/
├── main.rs           # Application entry point
├── assets            # Core 3D viewer functionality
  |--funko            #test model folder
  |--spider            #test model folder
├── engine          #custom code for engine as this was originally meant to branch into a game engine.
```

Dependencies

- `raylib-rs` - Rust bindings for Raylib
- `tobj` - OBJ file parsing
- `gltf` - GLTF/GLB file support

Performance

The viewer is optimized for performance and can handle large models efficiently. Typical frame rates:

- Simple models (< 50k vertices): 500+ FPS
- Complex models (500k+ vertices): 60+ FPS

Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

License

This project is licensed under the MIT License - see the LICENSE file for details.

Built With

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Raylib](https://www.raylib.com/) - Graphics and game development library
- [raylib-rs](https://github.com/deltaphc/raylib-rs) - Rust bindings for Raylib
