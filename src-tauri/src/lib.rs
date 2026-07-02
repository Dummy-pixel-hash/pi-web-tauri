use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder, command};
use rand::prelude::IteratorRandom;

/// Shared state for the app
pub struct AppState {
    pub server_process: Mutex<Option<std::process::Child>>,
    pub pi_web_dir: PathBuf,
    pub server_port: Mutex<Option<u16>>,
}

impl AppState {
    pub fn new() -> Self {
        // CARGO_MANIFEST_DIR is src-tauri/src
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let pi_web_tauri_dir = manifest_dir.parent().unwrap().parent().unwrap(); // pi-web-tauri/
        let pi_web_dir = pi_web_tauri_dir.parent().unwrap().join("pi-web-New-UI-");

        Self {
            server_process: Mutex::new(None),
            pi_web_dir,
            server_port: Mutex::new(None),
        }
    }
}

/// Start the PI WEB server
#[command]
fn start_server(app_state: State<Mutex<AppState>>) -> Result<String, String> {
    let state = app_state.lock().map_err(|e| e.to_string())?;

    if state.server_process.lock().unwrap().is_some() {
        return Err("Server is already running".to_string());
    }

    // Find the node binary
    let node_path = which::which("node").map_err(|e| format!("Node not found: {}", e))?;

    // Use a random port between 8500-9500
    let port: u16 = (8500..9500).choose(&mut rand::thread_rng()).unwrap();

    // Build the command to start the PI WEB server
    let mut cmd = Command::new(&node_path);
    cmd.current_dir(&state.pi_web_dir)
        .arg("dist/server/index.js")
        .arg(format!("--port={}", port))
        .env("PI_WEB_HOST", "127.0.0.1");

    // Start the process
    let child = cmd.spawn().map_err(|e| format!("Failed to start server: {}", e))?;

    drop(state);
    let state = app_state.lock().map_err(|e| e.to_string())?;
    *state.server_process.lock().unwrap() = Some(child);
    *state.server_port.lock().unwrap() = Some(port);

    Ok(format!("Server started on port {}", port))
}

/// Stop the PI WEB server
#[command]
fn stop_server(app_state: State<Mutex<AppState>>) -> Result<String, String> {
    let state = app_state.lock().map_err(|e| e.to_string())?;

    if let Some(mut child) = state.server_process.lock().unwrap().take() {
        child.kill().map_err(|e| format!("Failed to kill server: {}", e))?;
        child.wait().map_err(|e| format!("Failed to wait for server: {}", e))?;
    }

    *state.server_port.lock().unwrap() = None;

    Ok("Server stopped successfully".to_string())
}

/// Get the server port
#[command]
fn get_server_port(app_state: State<Mutex<AppState>>) -> Result<Option<u16>, String> {
    let state = app_state.lock().map_err(|e| e.to_string())?;
    let port = state.server_port.lock().map_err(|e| e.to_string())?;
    Ok(*port)
}

/// Get the PI WEB data directory
#[command]
fn get_data_dir() -> Result<String, String> {
    let data_dir = dirs::data_dir()
        .ok_or("Unable to determine data directory")?
        .join("pi-web");

    Ok(data_dir.to_string_lossy().to_string())
}

/// Create a directory
#[command]
fn create_directory(path: String) -> Result<(), String> {
    std::fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {}", e))
}

/// List files in a directory
#[command]
fn list_directory(path: String) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        files.push(entry.file_name().to_string_lossy().to_string());
    }

    Ok(files)
}

/// Read a file
#[command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Write to a file
#[command]
fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| format!("Failed to write file: {}", e))
}

/// Execute a shell command
#[command]
fn execute_command(command: String, cwd: Option<String>) -> Result<String, String> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(&command);

    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    let output = cmd.output().map_err(|e| format!("Failed to execute command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("Command failed: {}\n{}", stdout, stderr))
    }
}

/// Get the current working directory
#[command]
fn get_cwd() -> Result<String, String> {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to get cwd: {}", e))
}

/// Set the current working directory
#[command]
fn set_cwd(path: String) -> Result<(), String> {
    std::env::set_current_dir(&path)
        .map_err(|e| format!("Failed to set cwd: {}", e))
}

/// Get environment variable
#[command]
fn get_env_var(name: String) -> Result<Option<String>, String> {
    std::env::var(&name)
        .map(Some)
        .map_err(|e| format!("Failed to get env var: {}", e))
}

/// Set environment variable
#[command]
fn set_env_var(name: String, value: String) -> Result<(), String> {
    std::env::set_var(&name, &value);
    Ok(())
}

/// Get OS info
#[command]
fn get_os_info() -> Result<serde_json::Value, String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    Ok(serde_json::json!({
        "os": os,
        "arch": arch,
        "family": std::env::consts::FAMILY
    }))
}

/// Get system memory info (Windows-specific using powershell)
#[command]
fn get_memory_info() -> Result<serde_json::Value, String> {
    let output = Command::new("powershell")
        .args(["-Command", "[math]::Round((Get-CimInstance Win32_OperatingSystem).TotalVisibleMemorySize/1MB, 2)"])
        .output()
        .map_err(|e| format!("Failed to get memory info: {}", e))?;

    let total = String::from_utf8_lossy(&output.stdout).trim().parse::<f64>().unwrap_or(0.0);

    let output_avail = Command::new("powershell")
        .args(["-Command", "[math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory/1KB, 2)"])
        .output()
        .map_err(|e| format!("Failed to get memory info: {}", e))?;

    let available = String::from_utf8_lossy(&output_avail.stdout).trim().parse::<f64>().unwrap_or(0.0);

    Ok(serde_json::json!({
        "total_gb": total,
        "available_gb": available
    }))
}

/// Get disk usage
#[command]
fn get_disk_usage(path: String) -> Result<serde_json::Value, String> {
    let usage = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to get disk usage: {}", e))?;

    Ok(serde_json::json!({
        "size": usage.len(),
        "is_dir": usage.is_dir()
    }))
}

/// Open a URL in the default browser
#[command]
fn open_url(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| format!("Failed to open URL: {}", e))
}

#[cfg(dev)]
static DEFAULT_PI_WEB_URL: &str = "http://localhost:8505";
#[cfg(not(dev))]
static DEFAULT_PI_WEB_URL: &str = "http://localhost:8504";

/// Run the Tauri app
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let pi_web_url = std::env::var("PI_WEB_URL")
                .unwrap_or_else(|_| DEFAULT_PI_WEB_URL.to_string());

            let url = url::Url::parse(&pi_web_url)
                .unwrap_or_else(|e| {
                    log::error!("Invalid PI_WEB_URL '{}': {}, using default", pi_web_url, e);
                    url::Url::parse(DEFAULT_PI_WEB_URL).expect("invalid default URL")
                });

            log::info!("Loading PI WEB from: {}", url);

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("PI WEB")
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 600.0)
                .resizable(true)
                .fullscreen(false)
                .build()
                .expect("failed to create main window");

            let state = AppState::new();
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            get_server_port,
            get_data_dir,
            create_directory,
            list_directory,
            read_file,
            write_file,
            execute_command,
            get_cwd,
            set_cwd,
            get_env_var,
            set_env_var,
            get_os_info,
            get_memory_info,
            get_disk_usage,
            open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
