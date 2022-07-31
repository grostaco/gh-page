use yew::{html, Component};

mod components;
mod services;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
                <nav class="navbar">

                </nav>

                <div class="main">
                    <div class="main-content">
                        <section class="about">
                            <div class="bio">
                                <h2 class="name">{"Gros's Bio"}</h2>
                                <div class="content">
                                    <p>{"A"}</p>
                                </div>

                            </div>
                            <div class="avatar-area">
                            </div>
                        </section>
                        <div class="content">
                            <div class="content-grid">
                            </div>
                        </div>
                    </div>

                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
