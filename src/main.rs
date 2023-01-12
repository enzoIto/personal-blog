use yew::prelude::*;

#[function_component]
fn LandingPage() -> Html {
    html! {
        <div class="container">
            <h1>{"Welcome to the Rust Blog"}</h1>
            <p>{"This is a blog application built using Rust and Yew"}</p>
            <a href="/posts" class="button">{"View Posts"}</a>
        </div>
    }
}

fn main() {
    yew::Renderer::<LandingPage>::new().render();
}
