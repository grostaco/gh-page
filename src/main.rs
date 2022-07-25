use yew::{Component, html};

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
            <h1>{"Yew"}</h1>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}