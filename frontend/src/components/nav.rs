use yew::{function_component, html};

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="icon-container">
                <a href="/" aria-current="page" aria-label="Go back home">
                    <img src="https://cdn.discordapp.com/attachments/936974443559936032/1003132146803822612/garyiconrequest.png" style="max-width: 100%; display: block; position: static;"/>
                </a>
            </div>
        </nav>
    }
}
