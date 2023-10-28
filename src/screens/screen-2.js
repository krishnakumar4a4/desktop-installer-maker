const { invoke } = window.__TAURI__.tauri;

let userIdentifyNoticeEl;
let usersListEl;
let elevateBtnEl;
let usersList;

// async function identifyCurrentUser() {
//   invoke("identify_curr_user")
//   .then((user) => {
//     userIdentifyNoticeEl.innerHTML = `The user has been identified as ${user}`
//     nextBtnEl.disabled = false;
//   })
//   .catch((errMsg) => {
//     userIdentifyNoticeEl.innerHTML = `Failed to identify user: ${errMsg}`
//   })
// }

async function setInstallUser(user) {
  await invoke("set_install_user", {name: user})
}

// async function getSelectedInstallUser() {
//   invoke("get_selected_install_user")
//   .then((user) => {
//     userIdentifyNoticeEl.innerHTML = `The user selected for the app: ${user}`
//   })
//   .catch((errMsg) => {
//     userIdentifyNoticeEl.innerHTML = `Failed to select user for app: ${errMsg}`
//   })
// }

async function getUsers() {
  invoke("get_user_list")
  .then((users) => {
    let options;
    var updatedUsers = users.slice();
    updatedUsers.unshift("<Select User>");

    updatedUsers.map((op,i)=>{
      options+=`<option value="${op}" id="${i}" style="border-radius: 5px;"">${op}</option>`
    });
    usersListEl.innerHTML = options
    usersList = updatedUsers;
  })
  .catch((errMsg) => {
    usersListEl.innerHTML = `Failed to fetch users: ${errMsg}`
  })
}

window.addEventListener("DOMContentLoaded", () => {
  userIdentifyNoticeEl = document.querySelector("#user-name-identify-notice");
  usersListEl = document.querySelector("#users-list");
  elevateBtnEl = document.querySelector("#next-btn");
  elevateBtnEl.disabled = true;

  getUsers();

  usersListEl.addEventListener("change", async (event) => {
    console.log(event.target.selectedIndex);
    if (event.target.selectedIndex == 0) {
      elevateBtnEl.disabled = true;
    } else {
      elevateBtnEl.disabled = false;
      await setInstallUser(usersList[event.target.selectedIndex]);
      userIdentifyNoticeEl.innerHTML = `The user selected for the app: ${usersList[event.target.selectedIndex]}`
    }
  });
  // document.querySelector("#identify-user-btn").addEventListener("click", () => {
  //   identifyCurrentUser();
  // });
});
