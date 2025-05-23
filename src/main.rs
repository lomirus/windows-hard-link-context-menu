#![windows_subsystem = "windows"]

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use winsafe::{co, HWND, prelude::*};

fn alert_error(message: &str) -> Result<()>{
    let hwnd: HWND = HWND::DESKTOP;

    hwnd.MessageBox(
        message,
        "Error",
        co::MB::OK | co::MB::ICONWARNING,
    )?;

    Ok(())
}

fn create_hard_link(path: &Path) -> Result<()> {
    let new_filename = match path.extension() {
        Some(ext) => format!(
            "{}.hl.{}",
            path.file_stem()
                .context("Failed to get target file stem")?
                .to_str()
                .context("Failed to convert target file stem to `&str`")?,
            ext.to_str()
                .context("Failed to convert target file extention to `&str`")?
        ),
        None => format!(
            "{}.hl",
            path.file_name()
                .context("Failed to get target file name")?
                .to_str()
                .context("Failed to convert target file name to `&str`")?
        ),
    };

    std::fs::hard_link(
        path,
        path.parent()
            .context("Failed to get target parent")?
            .join(new_filename),
    )?;

    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let target = match args.get(1) {
        Some(v) => v,
        None => {
            alert_error("Target path argument is empty").unwrap();
            return;
        }
    };
    let path = PathBuf::from(target);
    if let Err(e) = create_hard_link(&path) {
        alert_error(&e.to_string()).unwrap();
    }
}
