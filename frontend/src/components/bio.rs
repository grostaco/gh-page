use yew::{function_component, html};

#[function_component(Bio)]
pub fn bio() -> Html {
    html! {
        <section class="about">
            <div class="bio">
                <h2 class="name">{"Hi, I'm Gary."}</h2>
                <div class="content">
                    <p class="text" style="margin-top: 0">{"I am a college student learning programming. I love learning new things and seeing them come to life with code."}</p>
                    <p class="text">{"I am really interested in machine learning and UX/UI. I also love playing video games on the side when time permits."}</p>
                </div>

                <div class="links">
                    <a class="icon-lightup" href="https://github.com/grostaco" aria-label="github link">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="28" width="28" xmlns="http://www.w3.org/2000/svg">
                            <g>
                                <path fill="none" d="M0 0h24v24H0z"></path>
                                <path d="M12 2C6.475 2 2 6.475 2 12a9.994 9.994 0 0 0 6.838 9.488c.5.087.687-.213.687-.476 0-.237-.013-1.024-.013-1.862-2.512.463-3.162-.612-3.362-1.175-.113-.288-.6-1.175-1.025-1.413-.35-.187-.85-.65-.013-.662.788-.013 1.35.725 1.538 1.025.9 1.512 2.338 1.087 2.912.825.088-.65.35-1.087.638-1.337-2.225-.25-4.55-1.113-4.55-4.938 0-1.088.387-1.987 1.025-2.688-.1-.25-.45-1.275.1-2.65 0 0 .837-.262 2.75 1.026a9.28 9.28 0 0 1 2.5-.338c.85 0 1.7.112 2.5.337 1.912-1.3 2.75-1.024 2.75-1.024.55 1.375.2 2.4.1 2.65.637.7 1.025 1.587 1.025 2.687 0 3.838-2.337 4.688-4.562 4.938.362.312.675.912.675 1.85 0 1.337-.013 2.412-.013 2.75 0 .262.188.574.688.474A10.016 10.016 0 0 0 22 12c0-5.525-4.475-10-10-10z"></path>
                            </g>
                        </svg>
                    </a>
                    <a class="icon-lightup" href="https://mobile.twitter.com/XeteraMnemonics" aria-label="twitter link">
                        <svg class="chakra-link" stroke="currentColor" fill="currentColor" viewBox="328 355 335 276" height="28" width="28" xmlns="http://www.w3.org/2000/svg">
                            <g>
                                <path fill="none" d="M0 0h24v24H0z"></path>
                                <path d="
                                    M 630, 425
                                    A 195, 195 0 0 1 331, 600
                                    A 142, 142 0 0 0 428, 570
                                    A  70,  70 0 0 1 370, 523
                                    A  70,  70 0 0 0 401, 521
                                    A  70,  70 0 0 1 344, 455
                                    A  70,  70 0 0 0 372, 460
                                    A  70,  70 0 0 1 354, 370
                                    A 195, 195 0 0 0 495, 442
                                    A  67,  67 0 0 1 611, 380
                                    A 117, 117 0 0 0 654, 363
                                    A  65,  65 0 0 1 623, 401
                                    A 117, 117 0 0 0 662, 390
                                    A  65,  65 0 0 1 630, 425
                                    Z"/>
                            </g>
                        </svg>
                    </a>
                    <a class="icon-lightup" href="https://www.instagram.com/traits_sbrk/" aria-label="instagram link">
                        <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" fill="currentColor" class="bi bi-instagram" viewBox="0 0 16 16">
                            <g>
                                <path fill="none" d="M0 0h24v24H0z"></path>
                                <path d="M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.917 3.917 0 0 0-1.417.923A3.927 3.927 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.916 3.916 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.926 3.926 0 0 0-.923-1.417A3.911 3.911 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0h.003zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599.28.28.453.546.598.92.11.281.24.705.275 1.485.039.843.047 1.096.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.47 2.47 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.478 2.478 0 0 1-.92-.598 2.48 2.48 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.233 0-2.136.008-2.388.046-3.231.036-.78.166-1.204.276-1.486.145-.373.319-.64.599-.92.28-.28.546-.453.92-.598.282-.11.705-.24 1.485-.276.738-.034 1.024-.044 2.515-.045v.002zm4.988 1.328a.96.96 0 1 0 0 1.92.96.96 0 0 0 0-1.92zm-4.27 1.122a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217zm0 1.441a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334z"/>
                            </g>
                        </svg>
                    </a>
                </div>

            </div>
            <div class="avatar-area">
                <div role="group" class="avatar">
                    <div class="avatar-container">
                        <img src="https://cdn.discordapp.com/attachments/936974443559936032/1003266978045247529/gary-padoru_2.png" layout="fixed" width="300" height="300" data-main-image="" sizes="300px" decoding="async" />

                    </div>
                    <p class="credit">
                        {"☝️ Avatar by "}
                        <span class="instagram-link">
                            <a rel="external" href="https://www.instagram.com/lindsjanellee/">{"@lindsjanellee"}</a>
                        </span>
                        {" (My girlfriend <3)"}
                    </p>
                </div>
            </div>
        </section>
    }
}
