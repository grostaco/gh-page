use yew::{html, Component};

mod components;
mod error;
mod services;

use components::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    // TODO: clean this up
    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
                <NavBar />
                <div class="main">
                    <div class="main-content">
                        <Bio />
                        <div class="content">
                            <div class="content-grid">
                               <GameList />
                               <SpotifyList />
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
