🧪 Pollinations Yew Lab

A Rust + Yew Browser App for AI-Powered Image Generation

🌿 Pollinations-Yew-Lab is a lightweight, browser-based image generator powered by the Pollinations.ai API and built with Rust, Yew, and WebAssembly (WASM).
No backend server, no Flask, no Node — just a blazing-fast, compiled-to-WASM frontend running directly in the browser.

Perfect for experiments, demos, teaching AI prompt design, or embedding inside modern WebApps like SharePoint (SPFx), educational dashboards, or kids’ creativity tools.

🚀 Features
Capability	Description
🎨 AI Image Generation	Uses Pollinations.ai to generate creative images from user prompts
💾 Image Download Support	Right-click save or programmatic download depending on browser
⚡ WASM Performance	App runs locally at near-native speed thanks to Rust ➜ WASM
📱 Responsive UI	Clean interface designed to run well on laptop, tablet, or kiosk
🔁 Randomizer	Optional random prompt mode for creativity sessions
🧰 Extensible	Built clean with modular Rust components ready for expansion
🧩 Tech Stack
Layer	Technology
Language	🦀 Rust
Framework	🧷 Yew
Compilation	🧱 wasm-bindgen + trunk
API	🤖 Pollinations.ai Image Generator
Hosting-ready	🌐 Static hosting (Hostek, GitHub Pages, Cloudflare Pages, Netlify, etc.)
📂 Folder Structure
pollinations-yew-lab/
├── /src
│   ├── main.rs        # Main Yew app
│   ├── components/    # Future UI components
├── Trunk.toml
├── Cargo.toml
├── index.html
└── README.md

🛠️ Build & Run

Install dependencies if not already installed:

cargo install trunk
rustup target add wasm32-unknown-unknown


Then run locally:

trunk serve


Build for deployment:

trunk build --release


Your final deployable assets will live in:

/dist

🌐 Deploying to Hostek (or any static web server)

Build the project:

trunk build --release


Upload the contents of dist/ to:

/public_html/pollinations-yew-lab/


Ensure your server supports:

application/wasm MIME type

Static file serving (no server-side rendering needed)

🔧 API Notes

Pollinations requires a prompt passed as a query parameter.

Example fetch URL pattern used:

https://image.pollinations.ai/prompt/{your text here}


Prompts should be URL-encoded and descriptive — example:

"cute robot painting watercolor sunset, Pixar style, soft glow, high detail"

✨ Suggested Prompts

Try these to stress-test the system:

Hyper-realistic hummingbird drinking from neon flowers, macro photography, 8k

Cyberpunk cat sitting on rooftop in rain, cinematic lighting, detailed fur render

Bible story scene: David and Goliath in Pixar animation style, colorful, kid-friendly

Beautiful African-American astronaut floating in an ISS observation cupola watching Earth sunrise, photorealistic

🧠 Roadmap

 Add multi-image gallery mode

 Add prompt history + JSON export

 Add voice-to-image using WebSpeech API

 Add share-to-LinkedIn/X buttons

 Add animated loading indicator

 Add API key control if Pollinations requires it later

👏 Credits

Built by Michael Givens (aka MikeGyver / Vibechemist)

Powered by Rust + Yew + WASM

Image generation via Pollinations.ai

📸 Demo URL

🔗 Hosted on:
👉 https://www.webhtml5.info/pollinations-yew-lab/ (placeholder — update after deployment)

📣 Feedback & Contributions

PRs, ideas, and forks welcome — especially from educators, makers, and AI experimenters.

💡 Final Thought

“If kids can generate creativity using a browser, imagine what they'll build tomorrow with Rust and AI.”

If you'd like, I can also generate:
