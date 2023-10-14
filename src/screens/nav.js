const { invoke } = window.__TAURI__.tauri;

async function navigate_prev_screen() {
  await invoke("prev", {})
}

async function navigate_next_screen() {
  await invoke("next", {})
}

async function finish_installation() {
  await invoke("finish", {})
}

window.addEventListener("DOMContentLoaded", () => {
  let prev_btn = document.querySelector("#prev-btn");
  let next_btn = document.querySelector("#next-btn");
  let finish_btn = document.querySelector("#finish-btn");

  if (prev_btn) {
    prev_btn.addEventListener("click", (e) => {
      e.preventDefault();
      navigate_prev_screen();
    });
  }

  if (next_btn) {
    next_btn.addEventListener("click", (e) => {
      e.preventDefault();
      navigate_next_screen();
    });
  }

  if (finish_btn) {
    finish_btn.addEventListener("click", (e) => {
      e.preventDefault();
      finish_installation();
    });
  }
});
