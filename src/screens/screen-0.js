const { invoke } = window.__TAURI__.tauri;

function reRunAsAdmin() {
  invoke("elevate_re_run")
  .then((_) => {
    console.log("Re Run as admin");
  })
  .catch((errMsg) => {
    console.log("Error Re Run as admin", errMsg);
  })
}

window.addEventListener("DOMContentLoaded", () => {
  let elevateBtnEl = document.querySelector("#elevate-btn");
  elevateBtnEl.addEventListener('click', (e) => {
    console.log("elevate button clicked", e);
    e.preventDefault();
    reRunAsAdmin();
  })
});
