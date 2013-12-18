Getting Started with SDL-2.0 and Rust
=================
This project is a tutorial for using the multimedia library SDL-2.0 with Rust. While the tools exist to use this library with Rust, there's relatively scant documentation. I will guide users on how to use this library with Rust and provide sample code and documentation on how to use it.


### Initial Setup

To start working with Rust-SDL you will want to follow the installation guide provided here: https://github.com/AngryLawyer/rust-sdl2


```rust
extern mod sdl2;

mod windowDemo;

#[main]
fn main() {
	println!("Going to window system...");
	windowDemo::main();
}
 
 ```

Then, to start with your own code you will want to create two files a main file, and a file to work with SDL's windowing system. In this case, we'll call it windowDemo.rs. You will have to declare extern mod sdl2 at the beginning of the main file and put a short empty public main function in windowDemo.rs for now. In order to run properly, both of these files should be placed in your /rust-sdl2/src folder. Then you can proceed to call "rustpkg install <folder>" from your rust-sdl2 director, where <folder> is the folder they're placed in. For this example, I placed them in a folder called winDemo. To run your program after calling rustpkg install winDemo, call ./bin/winDemo. Right now because there's no code in windowDemo.rs, nothing should happen, but we'll get to that next.

### Opening a Window

```rust
     let window = match sdl2::video::Window::new("rust-sdl2 demo: Videoooooo", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::Shown]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

 ```
