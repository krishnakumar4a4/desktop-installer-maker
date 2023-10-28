// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::{Stdio, ExitStatus, ExitCode}, path::PathBuf};

use tauri::{Manager, App};
extern crate whoami;

#[derive(Default)]
pub struct MyState {
  t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[tauri::command]
async fn prev(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    let curr_win_label = handle.get_focused_window().unwrap().label().to_owned();
    let mut prev_win_label = "";

    if curr_win_label == "screen-1" {
        prev_win_label = ""
    } else if curr_win_label == "screen-2" {
        prev_win_label = "screen-1"
    } else if curr_win_label == "screen-3" {
        prev_win_label = "screen-2"
    } else if curr_win_label == "screen-4" {
        prev_win_label = "screen-3"
    }

    match handle.get_focused_window().unwrap().hide() {
        Ok(_) => {},
        Err(e) => println!("error from loading screen {:?}", e)
    };

    match handle.get_window(&prev_win_label).unwrap().show() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("error from loading screen {:?}", e))
    }
}

#[tauri::command]
async fn next(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    let curr_win_label = handle.get_focused_window().unwrap().label().to_owned();
    let mut next_win_label = "";

    if curr_win_label == "screen-1" {
        next_win_label = "screen-2"
    } else if curr_win_label == "screen-2" {
        next_win_label = "screen-3"
    } else if curr_win_label == "screen-3" {
        next_win_label = "screen-4"
    }
    
    let _ = match handle.get_focused_window().unwrap().hide() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("error from loading screen {:?}", e))
    };

    match handle.get_window(&next_win_label).unwrap().show() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("error from loading screen {:?}", e))
    }
}

#[tauri::command]
async fn finish(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    handle.exit(0);
    Ok(())
}

#[tauri::command]
async fn set_state(handle: tauri::AppHandle, state: tauri::State<'_, MyState>, key: String, value: String) -> Result<(), String> {
  state.t.lock().unwrap().insert(key, value);
  Ok(())
}


#[tauri::command]
async fn get_state(handle: tauri::AppHandle, state: tauri::State<'_, MyState>, key: String) -> Result<String, String> {
  match state.t.lock().unwrap().get(&key) {
    Some(val) => Ok(val.to_owned()),
    None => Err("no-value-err".to_owned())
  }
}

#[tauri::command]
async fn identify_curr_user(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<String, String> {
    let username = whoami::username();
    state.t.lock().unwrap().insert("install-user".to_owned(), username.clone());
    Ok(username)
}

#[tauri::command]
async fn get_selected_install_user(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<String, String> {
    Ok(state.t.lock().unwrap().get("install-user").unwrap().to_owned())
}

#[tauri::command]
async fn set_install_user(handle: tauri::AppHandle, state: tauri::State<'_, MyState>, name: String) -> Result<(), String> {
    state.t.lock().unwrap().insert("install-user".to_owned(), name.clone());
    Ok(())
}

mod macos;
mod win;

fn main() {
    #[cfg(target_os = "macos")]
    let app = tauri::Builder::default()
    .manage(MyState::default())
    .invoke_handler(tauri::generate_handler![prev, next, finish, 
        set_state, get_state, identify_curr_user, set_install_user, get_selected_install_user, elevate_re_run,
        macos::get_default_install_folder,macos::get_user_list, 
        macos::stop_application, macos::uninstall_application, macos::start_install])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

    #[cfg(windows)]
    let app = tauri::Builder::default()
    .manage(MyState::default())
    .invoke_handler(tauri::generate_handler![prev, next, finish, 
        set_state, get_state, identify_curr_user, set_install_user, get_selected_install_user, elevate_re_run,
        win::get_default_install_folder,win::get_user_list, 
        win::stop_application, win::uninstall_application, win::start_install])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

    app.windows().into_iter().for_each(|(_, w)| {
        w.hide().unwrap();
    });

    launch_window(&app);

    // This will start the app and no other code below this will run.
    app.run(|_, _| {});
}

#[cfg(target_os = "macos")]
fn launch_window(app: &App) {
    let running_as = sudo::check();
    dbg!("running_as check: ", &running_as);
    
    if running_as != sudo::RunningAs::Root {
        app.get_window("screen-0").unwrap().show().unwrap();
    } else {
        app.get_window("screen-1").unwrap().show().unwrap();
    }
}

#[cfg(windows)]
fn launch_window(app: &App) {
    app.get_window("screen-1").unwrap().show().unwrap();
}

#[cfg(target_os = "macos")]
#[tauri::command]
async fn elevate_re_run(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    handle.get_focused_window().unwrap().hide().unwrap();
    match sudo::escalate_if_needed() {
        Ok(as_user) => {
            dbg!("running as user", as_user);
            handle.exit(0);
            Ok(())
        },
        Err(e) => {
            dbg!("error elevating permissions", &e);
            Err(format!("error elevating permissions: {:?}", e))
        },
    }
}

#[cfg(windows)]
#[tauri::command]
async fn elevate_re_run(handle: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    Ok(())
}
