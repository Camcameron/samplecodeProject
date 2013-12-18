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

First, as part of SDL, we want to declare what parts of SDL we want initiliazed. In this case we want their video system, so in Rust we simply declare:

```rust
sdl2::init([sdl2::InitVideo]);
 ```
Next, we'll need to declare our window. 

```rust
     let window = match sdl2::video::Window::new("rust-sdl2 demo: Videoooooo", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::Shown]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

 ```


We need to handle the case where for some reason, the window is not set properly, which in Rust we do with a match statement that simply continues if nothing is wrong and throws an error if something does fail. sdl2::video can be used to get us most of the information we need. Passing the initialization flag for the window's type is a bit different in Rust. The various types you can pass besides Shown can be found in the /src/sdl2/video.rs file as a windowFlags enum.


#### Final Notes

This is an in progress tutorial and is continuing to be updated...
