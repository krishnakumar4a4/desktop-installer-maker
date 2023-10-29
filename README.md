# Cross Platform Desktop Installer Maker
Build installers for Mac, Windows and Linux at oneshot.
Fully customizable User Interface with HTML/CSS/Javascript.
Code your installation actions in Rustlang and leverage all libraries from Rust ecosystem.
Built with Tauri + Vanilla + Rust.

## How to use this template
1. Fork this repository.
2. Add/Remove screen-<>.html and screen-<>.js as you like [here](src/screens). You can customise UX of each screen individually. 
3. Use common [styles.css](src/screens/styles.css) for customisation.
4. When you add/remove screens, you should update navigation rules in rust code in [main.rs](src-tauri/src/main.rs).
5. You should also add rust code for actions your installer needs to perform on each screen for [macos](src-tauri/src/macos.rs) and [win](src-tauri/src/win.rs).
5. Build & Run for a local preview.
6. Push to remote and let the github actions generate installers for you.

## High Level Installer Architecture
![arch](docs/assets/arch.GIF)

## Installer Run Preview
> ![Installer in Action](https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExZWw1M3IyYnptMmRheDJpemNxNmV1MHU5MGdiaHY5YzN5eWQ4YjhmaiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/tqGR8qdQB7UiwQUZCF/giphy.gif)


## Recommended IDE Setup
This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build & Run
1. npm install
2. npm run tauri dev