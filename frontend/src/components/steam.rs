use log::info;
use yew::{function_component, html, use_effect_with_deps};
use yew_hooks::use_async;

use crate::{
    error::Error,
    services::steam::{get_game_infos, get_owned_games, get_recent_games},
};

#[function_component(GameList)]
pub fn game_list() -> Html {
    let state = use_async(async move {
        async {
            let recent_games = get_recent_games().await?;
            let infos =
                get_game_infos(recent_games.games.iter().map(|game| game.appid).collect()).await?;

            Ok::<_, Error>(
                infos
                    .into_iter()
                    .zip(recent_games.games.into_iter())
                    .collect::<Vec<_>>(),
            )
        }
        .await
    });

    let owned_games = use_async(async move { get_owned_games().await });

    {
        let state = state.clone();
        let data = state.data.clone();

        use_effect_with_deps(
            move |_| {
                state.run();
                || ()
            },
            data.clone(),
        );

        let owned_games = owned_games.clone();
        let data = owned_games.data.is_some();
        use_effect_with_deps(
            move |_| {
                owned_games.run();
                || ()
            },
            data,
        )
    }

    html! {
        <div class="blog-container">
            <div class="blog-header">
                <div>
                    <p class="header-text">{format!("My steam games ({})", match &owned_games.data {
                        Some(games) => games.game_count,
                        None => 0,
                    })}</p>
                </div>
                <div class="flex-row align-center">
                    <a href="https://github.com/grostaco/gh-page" class="code">{"View this site's code"}</a>
                    <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="18" width="18" xmlns="http://www.w3.org/2000/svg"><g><path fill="none" d="M0 0h24v24H0z"></path><path d="M12 2C6.475 2 2 6.475 2 12a9.994 9.994 0 0 0 6.838 9.488c.5.087.687-.213.687-.476 0-.237-.013-1.024-.013-1.862-2.512.463-3.162-.612-3.362-1.175-.113-.288-.6-1.175-1.025-1.413-.35-.187-.85-.65-.013-.662.788-.013 1.35.725 1.538 1.025.9 1.512 2.338 1.087 2.912.825.088-.65.35-1.087.638-1.337-2.225-.25-4.55-1.113-4.55-4.938 0-1.088.387-1.987 1.025-2.688-.1-.25-.45-1.275.1-2.65 0 0 .837-.262 2.75 1.026a9.28 9.28 0 0 1 2.5-.338c.85 0 1.7.112 2.5.337 1.912-1.3 2.75-1.024 2.75-1.024.55 1.375.2 2.4.1 2.65.637.7 1.025 1.587 1.025 2.687 0 3.838-2.337 4.688-4.562 4.938.362.312.675.912.675 1.85 0 1.337-.013 2.412-.013 2.75 0 .262.188.574.688.474A10.016 10.016 0 0 0 22 12c0-5.525-4.475-10-10-10z"></path></g></svg>
                </div>
            </div>
            <div class="flex-column">
                <p class="title-text" style="color: #eee;">{"Recently played"}</p>
                <div class="game-list flex-column">
                            if let Some(data) = &state.data {
                                { for data.iter().take(5).map(|(gameinfo, game)| {
                                    html! { <div class="flex-row justify-between" style="gap: 20px;">
                                            <img src={gameinfo.header_image.clone()} style="width: 320px;"/>
                                            <div class="flex-column">
                                                <span style="margin-bottom: 5px;">
                                                <p class="header-text game-title">{game.name.clone()}</p>
                                                <p class="header-text game-time">{
                                                    format!("{:.1} hours", game.playtime_forever as f64 / 60.)
                                                }</p>

                                                </span>
                                                <div class="game-description-content">
                                                {gameinfo.short_description.clone()}
                                                </div>
                                            </div>
                                        </div>
                                }
                                })}
                            }
                </div>
            </div>
        </div>
    }
}
