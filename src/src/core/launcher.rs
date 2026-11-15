use crate::core::config::LaunchMethod;
use std::path::Path;
use std::process::Command;

pub struct GameLauncher;

impl GameLauncher {
    pub fn launch(method: &LaunchMethod) -> Result<(), String> {
        match method {
            LaunchMethod::Exe { path } => {
                let p = Path::new(path);
                if !p.exists() {
                    return Err(format!("Executable does not exist: {}", path));
                }
                let parent = p.parent().unwrap_or(Path::new(""));
                Command::new(path)
                    .current_dir(parent)
                    .spawn()
                    .map_err(|e| format!("Failed to spawn process: {}", e))?;
                Ok(())
            }
            LaunchMethod::Uri { uri } => {
                open::that_detached(uri).map_err(|e| format!("Failed to open URI {}: {}", uri, e))
            }
            LaunchMethod::Client { client_path, args } => {
                let p = Path::new(client_path);
                let parent = p.parent().unwrap_or(Path::new(""));
                Command::new(client_path)
                    .args(args)
                    .current_dir(parent)
                    .spawn()
                    .map_err(|e| format!("Failed to launch client: {}", e))?;
                Ok(())
            }
            LaunchMethod::Bnet { client_path, code } => {
                let p = Path::new(client_path);
                let parent = p.parent().unwrap_or(Path::new(""));

                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    let raw_command = format!("--exec=\"launch {}\"", code);
                    
                    // Check if Battle.net is already running
                    let is_running = if let Ok(output) = Command::new("tasklist").output() {
                        String::from_utf8_lossy(&output.stdout)
                            .to_lowercase()
                            .contains("battle.net.exe")
                    } else {
                        false
                    };

                    if is_running {
                        Command::new(client_path)
                            .current_dir(parent)
                            .raw_arg(&raw_command)
                            .spawn()
                            .map_err(|e| format!("Failed to start Battle.net game: {}", e))?;
                    } else {
                        Command::new(client_path)
                            .current_dir(parent)
                            .spawn()
                            .map_err(|e| format!("Failed to start Battle.net client: {}", e))?;

                        let client_clone = client_path.clone();
                        let parent_clone = parent.to_path_buf();
                        tokio::spawn(async move {
                            tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
                            let _ = Command::new(&client_clone)
                                .current_dir(parent_clone)
                                .raw_arg(&raw_command)
                                .spawn();
                        });
                    }
                    Ok(())
                }
                #[cfg(not(target_os = "windows"))]
                {
                    Err("Battle.net launch is only supported on Windows".to_string())
                }
            }
        }
    }

    pub fn open_folder(path: &str) -> Result<(), String> {
        let p = Path::new(path);
        let target = if p.is_file() {
            p.parent().unwrap_or(p)
        } else {
            p
        };
        open::that_detached(target).map_err(|e| format!("Failed to open folder: {}", e))
    }
}
