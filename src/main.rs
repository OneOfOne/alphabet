mod alphabet;

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
fn main() {
	yew::Renderer::<app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
	// skip first arg, which is the program's name
	for arg in std::env::args().skip(1) {
		println!("-> {} <-", arg);
		for (c, s) in alphabet::to_alphabet(arg) {
			println!("{} -> {}", c, s);
		}
	}
}
