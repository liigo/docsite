#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
	// ANCHOR: prevent_default
	cx.render(rsx! {
		a {
			href: "https://example.com",
			prevent_default: "onclick",
			"example.com",
		}
	})
	// ANCHOR_END: prevent_default
}
