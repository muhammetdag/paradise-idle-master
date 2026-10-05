use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

type SteamAPIInit = unsafe extern "C" fn() -> bool;
type SteamAPIRunCallbacks = unsafe extern "C" fn();
type SteamAPIShutdown = unsafe extern "C" fn();
type SteamAPIInitFlat = unsafe extern "C" fn(*mut u8) -> i32;

static ACTIVE_IDLERS: Lazy<Arc<Mutex<HashMap<String, Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn start_idler(app_id: String) -> Result<(), String> {
    if app_id.is_empty() || !app_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid App ID. App ID must contain only digits.".to_string());
    }

    let mut idlers = ACTIVE_IDLERS.lock().unwrap();
    
    if idlers.contains_key(&app_id) {
        return Err("This game is already running".to_string());
    }

    let exe_path = env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;

    let mut cmd = Command::new(exe_path);
    cmd.arg("--idle-worker").arg(&app_id);

    #[cfg(windows)]
    {
        // CREATE_NO_WINDOW so it doesn't flash a console window
        cmd.creation_flags(0x08000000);
    }

    let child = cmd.spawn()
        .map_err(|e| format!("Failed to start idler background process: {}", e))?;

    idlers.insert(app_id, child);

    Ok(())
}

pub fn stop_idler(app_id: &str) -> Result<(), String> {
    if app_id.is_empty() || !app_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid App ID. App ID must contain only digits.".to_string());
    }

    let mut idlers = ACTIVE_IDLERS.lock().unwrap();
    
    if let Some(mut child) = idlers.remove(app_id) {
        let _ = child.kill();
        let _ = child.wait();
        Ok(())
    } else {
        Err("Game is not running".to_string())
    }
}

pub fn run_headless_idler(app_id: String) {
    if app_id.is_empty() || !app_id.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Invalid App ID");
        std::process::exit(1);
    }

    // Set environment variables
    env::set_var("SteamAppId", &app_id);
    env::set_var("SteamGameId", &app_id);

    // Also write to steam_appid.txt
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let txt_path = exe_dir.join("steam_appid.txt");
            let _ = std::fs::write(&txt_path, &app_id);
        }
    } else {
        let _ = std::fs::write("steam_appid.txt", &app_id);
    }

    // Find steam_api64.dll
    let dll_path = match find_steam_dll() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    
    // Load the DLL
    let lib = unsafe {
        match Library::new(&dll_path) {
            Ok(l) => l,
            Err(_) => std::process::exit(1),
        }
    };

    // Try SteamAPI_Init first
    let mut success = false;
    
    if let Ok(init) = unsafe { lib.get::<Symbol<SteamAPIInit>>(b"SteamAPI_Init\0") } {
        success = unsafe { init() };
    } else if let Ok(init_flat) = unsafe { lib.get::<Symbol<SteamAPIInitFlat>>(b"SteamAPI_InitFlat\0") } {
        let mut err_msg = [0u8; 1024];
        let result = unsafe { init_flat(err_msg.as_mut_ptr()) };
        if result == 0 {
            success = true;
        }
    }

    if !success {
        std::process::exit(1);
    }

    let run_callbacks: Symbol<SteamAPIRunCallbacks> = unsafe {
        match lib.get(b"SteamAPI_RunCallbacks\0") {
            Ok(sym) => sym,
            Err(_) => std::process::exit(1),
        }
    };

    let _shutdown: Symbol<SteamAPIShutdown> = unsafe {
        match lib.get(b"SteamAPI_Shutdown\0") {
            Ok(sym) => sym,
            Err(_) => std::process::exit(1),
        }
    };

    // Main loop (will run until killed)
    loop {
        unsafe { run_callbacks() };
        thread::sleep(Duration::from_secs(5));
    }
}

fn find_steam_dll() -> Result<PathBuf, String> {
    // Only search in the executable directory to prevent DLL hijacking
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let dll_path = exe_dir.join("steam_api64.dll");
            if dll_path.exists() {
                return Ok(dll_path);
            }
        }
    }

    Err("steam_api64.dll not found. Please place it in the application directory next to the executable.".to_string())
}
