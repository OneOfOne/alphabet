use crate::alphabet::to_alphabet;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
	let input_ref = use_node_ref();
	let alphabet_value_handle = use_state(|| Vec::<String>::new());
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
				{ "Word(s): " }
				<input ref={input_ref} onchange={onchange} id="words" type="text"/>
			</label>
			<br />
			<ul>
				{alphabet_value.iter().map(|s| html! { <li><pre>{s}</pre></li> }).collect::<Html>()}
			</ul>
		</main>
	}
}
