mod alphabet;
use alphabet::to_alphabet;

use std::{env, io};

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
fn main() {
	yew::Renderer::<app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> io::Result<()> {
	// skip first arg, which is the program's name

	for arg in env::args().skip(1) {
		println!("-> {} <-", arg);
		println!("  {}", to_alphabet(arg).join("\n  "));
	}
	Ok(())
}
