use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use gloo_net::http::Request;
use js_sys::Math;
use urlencoding::encode;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement, InputEvent, Window};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Teal,
    Lilac,
    Dark,
}

impl Theme {
    fn as_class(&self) -> &'static str {
        match self {
            Theme::Teal => "theme-teal",
            Theme::Lilac => "theme-lilac",
            Theme::Dark => "theme-dark",
        }
    }
}

const PROMPTS: &[&str] = &[
    "A serene mountain landscape at sunrise with low mist and golden light",
    "A cozy cabin in a snowy forest at night, warm light glowing from the windows",
    "A futuristic city skyline with neon lights reflecting on rain-soaked streets",
    "A starry night sky over a calm lake, with the Milky Way clearly visible",
    "An ancient library filled with floating books and glowing runes, magical atmosphere",
    "A Pixar-style robot watering flowers on a tiny floating island in the sky",
    "A peaceful Japanese garden with a red bridge, koi pond, and cherry blossoms",
    "A dramatic thunderstorm over a sunflower field, cinematic lighting and contrast",
    "A whimsical treehouse village built in giant redwood trees, lanterns glowing at dusk",
    "An astronaut standing on an alien world, looking at a huge ringed planet in the sky",
    "A vibrant coral reef teeming with colorful fish and marine life, sun rays penetrating the water",
    "A fantasy castle perched on a cliff overlooking a vast ocean, sunset lighting",
    "A close-up portrait of a majestic lion with a flowing mane, golden hour lighting",
    "A bustling medieval marketplace with vendors, townsfolk, and lively activity",
    "A surreal desert landscape with giant floating crystals and a purple sky",
    "A cute puppy playing in a field of wildflowers under a bright blue sky",
    "A steampunk airship flying over a Victorian-era city, detailed and intricate design",
    "A magical forest clearing with glowing mushrooms and fairies dancing",
    "Majestic sunrise over a crystal-clear mountain lake, vibrant sky reflections, ultra-realistic colors, serene and inspiring.",
    "Desert dunes at twilight with long shadows, glowing horizon, dramatic clouds, cinematic mood.",
    "Van Gogh–inspired starry night over a quiet European village, swirling vibrant skies, painterly textures.",
    "Studio Ghibli–style cozy cottage in a meadow, magical lighting, whimsical atmosphere.",
    "Futuristic city skyline with floating skybridges, neon reflections in rain-soaked streets, cyberpunk aesthetic.",
    "A lone astronaut standing on an alien cliffside overlooking bioluminescent forests, surreal colors.",
    "A single bright red cardinal perched on a snowy branch, soft bokeh background, peaceful winter mood.",
    "A candlelit wooden desk with an open journal, handwritten notes, warm cozy glow, nostalgic atmosphere.",
];

fn random_prompt() -> &'static str {
    let n = PROMPTS.len() as f64;
    let idx = (Math::random() * n).floor() as usize;
    PROMPTS[idx]
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

#[function_component(App)]
fn app() -> Html {
    let prompt = use_state(|| "A serene mountain landscape at sunrise".to_string());
    let width = use_state(|| 1280u32);
    let height = use_state(|| 720u32);
    let model = use_state(|| "flux".to_string());
    let theme = use_state(|| Theme::Teal);
    let status = use_state(|| "Ready.".to_string());
    let error = use_state(|| Option::<String>::None);
    let is_generating = use_state(|| false);
    let preview_data_url = use_state(|| Option::<String>::None);

    // --- Handlers ---

    // Theme change
    let on_theme_change = {
        let theme = theme.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let val = select.value();
            let new_theme = match val.as_str() {
                "Lilac" => Theme::Lilac,
                "Dark" => Theme::Dark,
                _ => Theme::Teal,
            };
            theme.set(new_theme);
        })
    };

    // Prompt typing
    let on_prompt_input = {
        let prompt = prompt.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(target) = event.target() {
                if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                    prompt.set(textarea.value());
                }
            }
        })
    };

    // Width / height updates
    let on_width_change = {
        let width = width.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(target) = event.target() {
                let input = target
                    .unchecked_into::<web_sys::HtmlInputElement>();
                let value = input.value();
                if let Ok(num) = value.parse::<u32>() {
                    width.set(num);
                }
            }
        })
    };

    let on_height_change = {
        let height = height.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(target) = event.target() {
                let input = target
                    .unchecked_into::<web_sys::HtmlInputElement>();
                let value = input.value();
                if let Ok(num) = value.parse::<u32>() {
                    height.set(num);
                }
            }
        })
    };

    // Model selection
    let on_model_change = {
        let model = model.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            model.set(select.value());
        })
    };

    // Random prompt
    let on_random_prompt = {
        let prompt = prompt.clone();
        let status = status.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let p = random_prompt().to_string();
            prompt.set(p);
            status.set("Random prompt suggested. Ready to generate! 🎲".to_string());
            error.set(None);
        })
    };

    // Generate image (Pollinations)
    let on_generate = {
        let prompt = prompt.clone();
        let width = width.clone();
        let height = height.clone();
        let model = model.clone();
        let status = status.clone();
        let error = error.clone();
        let is_generating = is_generating.clone();
        let preview_data_url = preview_data_url.clone();

        Callback::from(move |_| {
            let prompt_value = (*prompt).trim().to_string();
            if prompt_value.is_empty() {
                status.set("Please enter a description for your image.".to_string());
                error.set(Some("Missing prompt".to_string()));
                return;
            }

            let w = *width;
            let h = *height;
            let m = (*model).clone();

            status.set("Generating image... please wait.".to_string());
            error.set(None);
            is_generating.set(true);
            preview_data_url.set(None);

            // Build URL
            let encoded_prompt = encode(&prompt_value);
            let cache_bust = js_sys::Date::now().to_string();
            let url = format!(
                "https://image.pollinations.ai/prompt/{}?width={}&height={}&model={}&cacheBust={}",
                encoded_prompt, w, h, m, cache_bust
            );

            let status_clone = status.clone();
            let error_clone = error.clone();
            let is_generating_clone = is_generating.clone();
            let preview_clone = preview_data_url.clone();

            spawn_local(async move {
                let resp = Request::get(&url).send().await;

                match resp {
                    Ok(r) => {
                        if !r.ok() {
                            let msg = format!("Error from server: HTTP {}", r.status());
                            status_clone.set("Error while generating image.".to_string());
                            error_clone.set(Some(msg));
                            is_generating_clone.set(false);
                            return;
                        }
                        match r.binary().await {
                            Ok(bytes) => {
                                let b64 = BASE64.encode(&bytes);
                                let data_url =
                                    format!("data:image/jpeg;base64,{}", b64);
                                preview_clone.set(Some(data_url));
                                status_clone.set(format!(
                                    "Image generated at {}x{} using model '{}'.",
                                    w, h, m
                                ));
                                is_generating_clone.set(false);
                            }
                            Err(e) => {
                                let msg = format!("Failed to read image bytes: {e}");
                                status_clone.set("Error while generating image.".to_string());
                                error_clone.set(Some(msg));
                                is_generating_clone.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        let msg = format!("Network error: {e}");
                        status_clone.set("Error while generating image.".to_string());
                        error_clone.set(Some(msg));
                        is_generating_clone.set(false);
                    }
                }
            });
        })
    };

    // Convert button (desktop-only feature)
    let on_convert_click = Callback::from(move |_| {
        let _ = window().alert_with_message(
            "JPG → PNG/ICO conversion is available in the desktop Python version.\n\nIn this web version, right-click the generated image and choose “Save image as…”",
        );
    });

    // Derived values
    let theme_class = theme.as_class();
    let generating = *is_generating;
    let status_text = (*status).clone();
    let error_text = (*error).clone();
    let prompt_value = (*prompt).clone();
    let width_value = *width;
    let height_value = *height;
    let model_value = (*model).clone();
    let preview_url = (*preview_data_url).clone();

    html! {
        <div class={classes!("app", theme_class)}>
            <div class="app-inner">
                <div class="header-bar">
                    <div class="header-title">
                        { "🎨 Pollinations.ai Image Lab (Yew)" }
                    </div>
                    <div class="header-right">
                        <span>{ "Theme:" }</span>
                        <select onchange={on_theme_change}>
                            <option value="Teal" selected={matches!(*theme, Theme::Teal)}>{ "Teal" }</option>
                            <option value="Lilac" selected={matches!(*theme, Theme::Lilac)}>{ "Lilac" }</option>
                            <option value="Dark" selected={matches!(*theme, Theme::Dark)}>{ "Dark" }</option>
                        </select>
                    </div>
                </div>

                <div class="shadow-frame">
                    <div class="card">
                        <div class="title">{ "Pollinations.ai Image Generator" }</div>
                        <div class="subtitle">
                            { "Enter a prompt, choose size and model, then generate your AI image." }
                        </div>

                        <div class="section-label">{ "Image Prompt:" }</div>
                        <textarea
                            class="prompt-textarea"
                            value={prompt_value}
                            oninput={on_prompt_input}
                            placeholder="Describe the image you want to create..."
                        />

                        <div class="options-row">
                            <div class="option-group">
                                <label>{ "Width" }</label>
                                <input
                                    type="number"
                                    min="256"
                                    max="4096"
                                    step="64"
                                    value={width_value.to_string()}
                                    oninput={on_width_change}
                                />
                            </div>
                            <div class="option-group">
                                <label>{ "Height" }</label>
                                <input
                                    type="number"
                                    min="256"
                                    max="4096"
                                    step="64"
                                    value={height_value.to_string()}
                                    oninput={on_height_change}
                                />
                            </div>
                            <div class="option-group">
                                <label>{ "Model" }</label>
                                <select value={model_value} onchange={on_model_change}>
                                    <option value="flux">{ "flux" }</option>
                                    <option value="turbo">{ "turbo" }</option>
                                    <option value="anime">{ "anime" }</option>
                                    <option value="realistic">{ "realistic" }</option>
                                </select>
                            </div>
                        </div>

                        <div class="buttons-row">
                            <button class="btn btn-secondary" onclick={on_random_prompt}>
                                { "🎲 Random Prompt" }
                            </button>
                            <button
                                class={classes!("btn", "btn-accent", generating.then_some("btn-disabled"))}
                                onclick={on_generate}
                                disabled={generating}
                            >
                                { "🎨 Generate Image" }
                            </button>
                            <button class="btn btn-secondary" onclick={on_convert_click}>
                                { "🪄 Convert JPG → PNG/ICO" }
                            </button>
                        </div>

                        <div class="status-bar">
                            { status_text }
                        </div>
                        {
                            if let Some(err) = error_text {
                                html! { <div class="error-text">{ err }</div> }
                            } else {
                                html! {}
                            }
                        }

                        <div class="preview-section-title">
                            { "Image Preview:" }
                        </div>
                        <div class="preview-card">
                        {
                            if let Some(url) = preview_url {
                                html! {
                                    <img class="preview-image" src={url} alt="Generated Pollinations image" />
                                }
                            } else {
                                html! {
                                    <div class="preview-placeholder">
                                        { "No image generated yet. Describe something and click “Generate Image”!" }
                                    </div>
                                }
                            }
                        }
                        </div>
                        <div class="footer-hint">
                            { "Tip: Right-click (or long-press on mobile) the image to save it locally." }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}