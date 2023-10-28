static WIN_INSTALL_DIR: &str = "C:\\TestApp\\";

#[cfg(windows)]
#[tauri::command]
pub async fn get_user_list(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<Vec<String>, String> {
    // TODO: get windows user list the right way
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

#[cfg(windows)]
#[tauri::command]
pub async fn get_default_install_folder(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<String, String> {
    Ok(WIN_INSTALL_DIR.to_owned())
}

#[cfg(windows)]
#[tauri::command]
pub async fn start_install(handle: tauri::AppHandle, state: tauri::State<'_, crate::MyState>) -> Result<String, String> {
    use std::{fs, path::PathBuf, io::{self, Write}};
    let install_dir = WIN_INSTALL_DIR.to_owned();

    // create install dir
    dbg!("Creating installing dir");
    match fs::create_dir_all(install_dir.clone()) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("error creating install directory: {:?}", e))
        }
    }

    // create x86_64 install dir
    dbg!("Creating x86_64 installing dir");
    match fs::create_dir_all(PathBuf::from(install_dir.clone()).join("x86_64")) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("error creating x86_64 install directory: {:?}", e))
        }
    }

    
    #[cfg(windows)]
    extern crate winreg;
    #[cfg(windows)]
    use winreg::enums::*;
    #[cfg(windows)]
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    dbg!("hklm: {:?}", &hklm);
    let (reg, disp) = hklm.create_subkey("SYSTEM\\CurrentControlSet\\Services\\TestApp").unwrap();
    dbg!("test app reg: {:?}", &reg);
    reg.set_value("test_path", &"C:\\TestApp\\config.toml").unwrap();
    dbg!("setting reg value");

    Ok("start installation success".to_owned())
}

#[cfg(windows)]
#[tauri::command]
pub fn stop_application() -> String {
    "stop installation success".to_owned()
}

#[cfg(windows)]
#[tauri::command]
pub fn uninstall_application() -> String {
    "uninstall app success".to_owned()
}
