extern mod sdl2;

mod windowDemo;

#[main]
fn main() {
	println!("Starting window system demo...");
	windowDemo::main();
}

//#[start]
//fn start(argc: int, argv: **u8) -> int {
//    std::rt::start_on_main_thread(argc, argv, main)
//}

