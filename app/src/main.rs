// main.rs
use chrono::prelude::*;
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let utc: String = Utc::now().format("%d. %m %Y %H:%M:%S").to_string();
        let state = create_signal(cx, utc);

        spawn_local_scoped(cx, async move {
            loop {
                let utc: String = Utc::now().format("%d. %m %Y %H:%M:%S").to_string();
                TimeoutFuture::new(10000).await;
                state.set(utc);
            }
        });
        view! { cx,
            App(state=state)
        }
    });
}

#[derive(Prop)]
struct AppProps<'a> {
    state: &'a ReadSignal<String>,
}

#[component]
fn App<'a, G: Html>(cx: Scope<'a>, props: AppProps<'a>) -> View<G> {
    view! {cx,
        div(class="page-wrapper with-navbar") {
            nav(class="navbar") {
                div(class="content") {
                    "🦀  BIG IRON"
                }
            }
            div(class="content-wrapper") {
                div(class="content") {
                    section(class="card") {
                        h1 {
                            "BIG IRON"
                        }
                        h2(class="card-title") {
                            "Just ship it."
                        }
                        p {
                            (props.state.get())
                        }
                    }
                    section(class="card") {
                        h2(class="card-title") {
                            "Portfolio"
                        }
                        img(style="width: 100%;", src=format!("http://localhost:8000/plot/account_balance_history?timestamp={}", props.state.get()))
                    }
                }
            }
        }
    }
}
