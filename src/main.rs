mod alphabet;

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
fn main() {
	yew::Renderer::<app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
use std::{env, io};
#[cfg(not(target_arch = "wasm32"))]
fn main() -> io::Result<()> {
	// skip first arg, which is the program's name

	for arg in env::args().skip(1) {
		println!("-> {} <-", arg);
		for (c, s) in alphabet::to_alphabet(arg) {
			println!("{} -> {}", c, s);
		}
	}
	Ok(())
}
