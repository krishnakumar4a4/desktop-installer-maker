
static MAC_INSTALL_DIR: &str = "/Library/TestApp/0.2.0";

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn get_user_list(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<Vec<String>, String> {
    let mac_users = std::process::Command::new("dscl").args([".", "list", "/Users"]).output().unwrap();
    let mac_users_string = String::from_utf8(mac_users.stdout).unwrap();
    let users = mac_users_string.split("\n").collect::<Vec<&str>>();
    
    println!("users list {:?}", users);
    Ok(users.into_iter().filter(|val| {
        !val.starts_with("_")
    }).map(|val| {
        val.to_owned()
    }).collect())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn get_default_install_folder(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<String, String> {
    Ok(MAC_INSTALL_DIR.to_owned())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn start_install(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<String, String> {
    use std::{fs, path::PathBuf, io::{self, Write}};
    let install_dir = MAC_INSTALL_DIR.to_owned();

    // create install dir
    dbg!("Creating installing dir");
    match fs::create_dir_all(install_dir.clone()) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("error creating install directory: {:?}", e))
        }
    }

    // create logs dir
    dbg!("Creating logs dir");
    let logs_dir = PathBuf::from(install_dir.clone()).join("logs");
    fs::create_dir_all(logs_dir).unwrap();
    
    Ok("start installation success".to_owned())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn stop_application() -> String {
    "stop installation success".to_owned()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn uninstall_application() -> String {
    "uninstall app success".to_owned()
}
