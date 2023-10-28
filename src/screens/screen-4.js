const { invoke } = window.__TAURI__.tauri;

let startBtn;
let finishBtnEl;
// let loaderEl;
let installErrDesEl;
let installSuccessDescEl;

let isAppStopped = false;
let isUnInstalled = false
let isInstalled = false;

let installProgressEl;

function stopApplication() {
  invoke("stop_application", {}).then((val) => {
    isAppStopped = true;
    console.log("stop application", isAppStopped);
    installProgressEl.style.width = '33%';
  }).catch((e) => {
    console.log("error stopping app", e);
    installErrDesEl.innerHTML = `Error stopping application: ${e}`
    isAppStopped = false;
    setForInstallError();
  });
}

function uninstallApplication() {
  invoke("uninstall_application", {}).then((val) => {
    isUnInstalled = true;
    console.log("uninstall application", isUnInstalled);
    installProgressEl.style.width = '66%';
  }).catch((e) => {
    console.log("error uninstalling app", e);
    installErrDesEl.innerHTML = `Error uninstalling application: ${e}`;
    isUnInstalled = false;
    setForInstallError();
  });
}

function startInstall() {
  invoke("start_install", {}).then((val) => {
    isInstalled = true;
    console.log("install application", isInstalled);
    installProgressEl.style.width = '100%';
    evaluateInstallation();
  }).catch((e) => {
    console.log("error installing app", e);
    installErrDesEl.innerHTML = `Error installing application: ${e}`;
    isInstalled = false;
    setForInstallError();
  });
}

function setForInstallError() {
  // loaderEl.style.visibility = "hidden";
  finishBtnEl.disabled = false;

  startBtn.innerHTML = "Retry";
  finishBtnEl.innerHTML = "UnInstall";
  setStartBtnListener();
}

function setStartBtnListener() {
  startBtn.addEventListener("click", async () => {
    // loaderEl.style.visibility = "visible";
    stopApplication();
    uninstallApplication();
    startInstall();
  })
}

window.addEventListener("DOMContentLoaded", () => {
  startBtn = document.querySelector("#start-btn");
  finishBtnEl = document.querySelector("#finish-btn");
  // loaderEl = document.querySelector("#loader");
  installErrDesEl = document.querySelector("#install-error-description");
  installSuccessDescEl = document.querySelector("#install-success-message");
  installProgressEl = document.querySelector(".progress-bar-fill");

  finishBtnEl.disabled = true;
  // loaderEl.style.visibility = "hidden";

  setStartBtnListener();
});

function evaluateInstallation() {
  console.log(isAppStopped, isUnInstalled, isInstalled);

  if (isAppStopped && isUnInstalled && isInstalled) {
    // startBtn.disabled = true;
    startBtn.remove();
    finishBtnEl.disabled = false;
    // loaderEl.remove();
    installSuccessDescEl.innerHTML = "Installation Successful ✔"
  } else {
    finishBtnEl.disabled = false;
    // loaderEl.style.visibility = "hidden";
  }
}

