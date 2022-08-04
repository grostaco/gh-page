use std::collections::HashMap;

use yew::{function_component, html, use_effect_with_deps, virtual_dom::VNode};
use yew_hooks::use_async;

use services::spotify::chrono::DateTime;

use crate::services::spotify::get_tracks;

#[function_component(SpotifyList)]
pub fn spotify_list() -> Html {
    let state = use_async(async move { get_tracks().await });
    {
        let state = state.clone();
        let data = state.data.clone();
        use_effect_with_deps(
            move |_| {
                state.run();
                || ()
            },
            data,
        );
    }

    let mut groups: HashMap<String, (u64, Vec<VNode>)> = HashMap::new();
    if let Some(tracks) = &state.data {
        for item in &tracks.items {
            let dt = DateTime::parse_from_rfc3339(&item.added_at).unwrap();
            let ts = js_sys::Date::now() / 1000. - dt.timestamp() as f64;
            let days = ts / 3600. / 24.;

            let key = if days < 1. {
                "Just today".to_string()
            } else if days < 2. {
                "A day ago".to_string()
            } else if days < 30. {
                format!("{} days ago", days as u64)
            } else if days / 30. < 12. {
                format!("{} months ago", (days / 30.) as u64)
            } else if (days / 30.) as u64 == 12 {
                "About a year ago".to_string()
            } else if (days / 30. / 12.) < 2. {
                "A year ago".to_string()
            } else {
                format!("{} years ago", (days / 30. / 12.) as u64)
            };

            let value: VNode = html! {
                <a class="track-anchor" href={item.track.external_urls.as_object().unwrap().get("spotify").unwrap().as_str().unwrap().to_string()}>
                    <div class="track-info">

                        <img width="64px" height="64px" alt="Placeholder" src={item.track.album.images.last().unwrap().url.clone()} loading="lazy" class="track-image" fetchpriority="low"/>
                        <div class="track-about">
                            <p class="title">{item.track.album.name.clone()}</p>
                            <p class="artist">{
                                {
                                    let mut iter = item.track.artists.iter();
                                    let init = iter.next().unwrap().name.clone();
                                    iter.fold(init, |acc, artist| acc + ", " + &artist.name)
                                }
                            }
                            </p>
                        </div>

                    </div>
                </a>
            };

            match groups.get_mut(&key) {
                Some(grp) => grp.1.push(value),
                None => {
                    groups.insert(key, (ts as u64, vec![value]));
                }
            }
        }
    }
    let mut groups = groups.into_iter().collect::<Vec<_>>();
    groups.sort_by(|(_, (ts1, _)), (_, (ts2, _))| ts1.cmp(ts2));

    html! {
         <div class="spotify-container">
            <div class="spotify-header">
                <p class="text">{"My Liked Songs"}</p>
                <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="20" width="20" xmlns="http://www.w3.org/2000/svg">
                    <g>
                        <path fill="none" d="M0 0h24v24H0z"></path>
                        <path fill-rule="nonzero" d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.55 2 12 2zm3.75 14.65c-2.35-1.45-5.3-1.75-8.8-.95-.35.1-.65-.15-.75-.45-.1-.35.15-.65.45-.75 3.8-.85 7.1-.5 9.7 1.1.35.15.4.55.25.85-.2.3-.55.4-.85.2zm1-2.7c-2.7-1.65-6.8-2.15-9.95-1.15-.4.1-.85-.1-.95-.5-.1-.4.1-.85.5-.95 3.65-1.1 8.15-.55 11.25 1.35.3.15.45.65.2 1s-.7.5-1.05.25zM6.3 9.75c-.5.15-1-.15-1.15-.6-.15-.5.15-1 .6-1.15 3.55-1.05 9.4-.85 13.1 1.35.45.25.6.85.35 1.3-.25.35-.85.5-1.3.25C14.7 9 9.35 8.8 6.3 9.75z"></path>
                    </g>
                </svg>
            </div>
            <div class="tracks-container">
                if state.data.is_some() {
                    {for groups.into_iter().map(|(date, (_, tracks))| html!{
                        <div class="track-container">
                            <div class="track-date">
                                <div class="line-1"></div>
                                <p class="date">{{
                                    date
                                }}</p>
                                <div class="line-2"></div>
                            </div>
                            <div class="track">
                                {for tracks}
                            </div>
                        </div>
                    })}
                } else {
                    <p>{"Loading..."}</p>
                }
            </div>
        </div>
    }
}

//  <div class="spotify-container">
//                                     <div class="spotify-header">
//                                         <p class="text">{"My Liked Songs"}</p>
//                                         <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="20" width="20" xmlns="http://www.w3.org/2000/svg">
//                                             <g>
//                                                 <path fill="none" d="M0 0h24v24H0z"></path>
//                                                 <path fill-rule="nonzero" d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.55 2 12 2zm3.75 14.65c-2.35-1.45-5.3-1.75-8.8-.95-.35.1-.65-.15-.75-.45-.1-.35.15-.65.45-.75 3.8-.85 7.1-.5 9.7 1.1.35.15.4.55.25.85-.2.3-.55.4-.85.2zm1-2.7c-2.7-1.65-6.8-2.15-9.95-1.15-.4.1-.85-.1-.95-.5-.1-.4.1-.85.5-.95 3.65-1.1 8.15-.55 11.25 1.35.3.15.45.65.2 1s-.7.5-1.05.25zM6.3 9.75c-.5.15-1-.15-1.15-.6-.15-.5.15-1 .6-1.15 3.55-1.05 9.4-.85 13.1 1.35.45.25.6.85.35 1.3-.25.35-.85.5-1.3.25C14.7 9 9.35 8.8 6.3 9.75z"></path>
//                                             </g>
//                                         </svg>
//                                     </div>
//                                     <div class="tracks-container">
//                                         <div class="track-container">
//                                             <div class="track-date">
//                                                 <div class="line-1"></div>
//                                                 <p class="date">{"8 days ago"}</p>
//                                                 <div class="line-2"></div>
//                                             </div>
//                                             <div class="track">
//                                                 <a class="track-anchor" href="">
//                                                     <div class="track-info">
//                                                         <img width="64px" height="64px" alt="Album art for &quot;House of Glass&quot; by Eyes Set To Kill" src="https://i.scdn.co/image/ab67616d0000485120865cba1b66dded43a7f142" loading="lazy" class="track-image" fetchpriority="low"/>
//                                                         <div class="track-about">
//                                                             <p class="title">{"Ligma"}</p>
//                                                             <p class="artist">{"Balls"}</p>
//                                                         </div>
//                                                     </div>

//                                                 </a>
//                                             </div>
//                                         </div>
//                                     </div>

//                                 </div>

// #[cfg(test)]
// mod test {
//     use spotify::chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
//     #[test]
//     fn foo() {
//         let dt = DateTime::<FixedOffset>::parse_from_rfc3339("2022-05-07T01:04:10Z").unwrap();

//         let ts = Utc::now().timestamp() - dt.timestamp();

//         println!("{:#?} days ago", ts / 3600 / 24);
//     }
// }
