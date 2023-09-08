use crate::alphabet::to_alphabet;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
	let input_ref = use_node_ref();
	let alphabet_value_handle = use_state(|| Vec::<(char, String)>::new());
	let alphabet_value = (*alphabet_value_handle).clone();

	let onchange = {
		let input_ref = input_ref.clone();

		Callback::from(move |_| {
			let input = input_ref.cast::<HtmlInputElement>();
			println!("aaahhh");
			if let Some(input) = input {
				let value = input.value();
				let alpha = to_alphabet(value.clone());
				alphabet_value_handle.set(alpha);
			}
		})
	};

	html! {
		<main>
			<label for="words">
				{"Words:"}
			</label>
			<input ref={input_ref} onchange={onchange} id="words" type="text"/>
			<br />
			<ul>
				{alphabet_value.iter().map(|(c,s)| html! { <li><pre>{"`"}{c}{"` -> "}{s}</pre></li> }).collect::<Html>()}
			</ul>
			<footer>
				{"Made with the complete lack of sleep using Rust, WASM and NeoVim."}
				<br />
				{"©️ 2023 "}<a href={"https://github.com/OneOfOne"}>{"OneOfOne"}</a>{" ("}<a href={"https://github.com/OneOfOne/alphabet"}>{"src"}</a>{")."}
			</footer>
		</main>
	}
}
