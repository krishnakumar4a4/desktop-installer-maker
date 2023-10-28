const { invoke } = window.__TAURI__.tauri;

let nextBtnEl;
let installFolderEl;

async function getDefaultInstallFolder() {
  invoke("get_default_install_folder", {}).then((val) => {
    installFolderEl.value = val;
  }).catch((_) => {});
}

window.addEventListener("DOMContentLoaded", () => {
  installFolderEl = document.querySelector("#install-folder");
  getDefaultInstallFolder();
  document.querySelector("#install-folder").addEventListener("change", (event) => {
    console.log("event", event);
  })
});
