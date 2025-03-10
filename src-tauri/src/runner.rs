use crate::logging::init_logger;
use anyhow::Context as _;

pub fn run_tauri() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| Ok(init_logger(app)?))
        .invoke_handler(tauri::generate_handler![
            crate::cmd::change_log_level,
            crate::cmd::add_num,
            crate::cmd::input_command,
        ])
        .run(tauri::generate_context!())
        .context("Failed to execute tauri")
}
