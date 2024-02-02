use app_core::add;
use tauri::api::process::{Command, CommandEvent};

/// early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        tracing::error!("{}", $err);
        return Err($err.to_string());
    }};
}

#[tauri::command]
pub(crate) async fn add_num(num1: u64, num2: u64) -> Result<u64, String> {
    add(num1, num2).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn change_log_level(log_level: Option<&str>) -> Result<(), String> {
    tracing::debug!("Selected log level: {:?}", log_level);
    crate::logging::change_log_level(log_level.unwrap_or("error")).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn input_command(command: String, args: Vec<String>) -> Result<String, String> {
    tracing::debug!("Selected command: {:?}", command);
    tracing::debug!("Selected args: {:?}", args);

    // https://docs.rs/tauri/latest/tauri/api/process/struct.Command.html#method.spawn
    let (mut rx, mut child) = match Command::new(command).args(args).spawn() {
        Ok(res) => res,
        Err(err) => bail!(err),
    };

    let mut i = 0;
    let mut res = String::new();
    while let Some(event) = rx.recv().await {
        tracing::debug!("Selected event: {:?}", event);
        if let CommandEvent::Stdout(line) = event {
            tracing::debug!("got: {}", line);
            res.push_str(line.as_str());
            i += 1;
            if i == 4 {
                child.write("message from Rust\n".as_bytes()).unwrap();
                i = 0;
            }
        }
    }

    Ok(res)
}
