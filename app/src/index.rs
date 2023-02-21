use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="container") { "Hello!" }
    }
}
