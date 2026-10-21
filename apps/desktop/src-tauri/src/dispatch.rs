//! Labyrica Dispatch beacon. On launch we drop a small JSON file so the
//! Dispatch updater can find this install, read the running version, and
//! close the right process before updating. Contract:
//! https://github.com/Labyrica/labyrica-dispatch/blob/main/docs/BEACON.md
//!
//! Best effort: every error is ignored, startup never fails over it.

use tauri::Manager;

const ID: &str = "foundry-unity";
const NAME: &str = "Foundry for Unity";

pub fn write_beacon<R: tauri::Runtime>(app: &tauri::App<R>) {
    let Ok(base) = app.path().local_data_dir() else {
        return;
    };
    let dir = base.join("Labyrica").join("apps");
    let exe = std::env::current_exe().ok();
    // macOS: the enclosing .app, if we're running from one.
    let bundle = exe.as_deref().and_then(|p| {
        p.ancestors()
            .find(|a| a.extension().is_some_and(|e| e == "app"))
            .map(|a| a.to_path_buf())
    });
    let mut beacon = serde_json::json!({
        "schema": 1,
        "id": ID,
        "name": NAME,
        "version": app.package_info().version.to_string(),
        "executable": exe,
        "pid": std::process::id(),
    });
    if let Some(bundle) = bundle {
        beacon["bundle"] = serde_json::json!(bundle);
    }
    let Ok(json) = serde_json::to_string_pretty(&beacon) else {
        return;
    };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let tmp = dir.join(format!("{ID}.json.tmp"));
    if std::fs::write(&tmp, json).is_ok() {
        let _ = std::fs::rename(&tmp, dir.join(format!("{ID}.json")));
    }
}
