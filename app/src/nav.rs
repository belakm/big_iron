use sycamore::prelude::*;

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
    view! { cx,

        nav(class="nav") {
            div(class="nav-center") {
                a(href="/") { "🦀  BIG IRON" }
                a(href="/counter") { "Counter" }
                a(href="/blog") { "Blog" }
            }
        }
    }
}
