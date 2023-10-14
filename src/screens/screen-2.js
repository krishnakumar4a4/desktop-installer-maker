const { invoke } = window.__TAURI__.tauri;

let greetEl;

async function greet_user_selection() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
   invoke("get_state", { key: "name" })
    .then((value) => document.getElementById("greet").innerHTML = `User selected for processing: ${value}`)
    .catch((errMsg) => document.getElementById("greet").innerHTML = `error received ${errMsg}`)
}

window.addEventListener("DOMContentLoaded", () => {
  greet_user_selection();
});
