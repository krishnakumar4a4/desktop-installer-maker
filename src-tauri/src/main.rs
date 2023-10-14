// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[derive(Default)]
struct MyState {
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

fn main() {
    let app = tauri::Builder::default()
    .manage(MyState::default())
    .invoke_handler(tauri::generate_handler![prev, next, finish, set_state, get_state])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

    app.windows().into_iter().for_each(|(_, w)| {
        w.hide().unwrap();
    });

    app.get_window("screen-1").unwrap().show().unwrap();

    // This will start the app and no other code below this will run.
    app.run(|_, _| {});
}
