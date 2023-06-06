// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod model;

use rand::Rng;
use std::fs::{create_dir, remove_file, write};
use std::path::Path;
use std::path::PathBuf;

use model::Game;
use opener::open;
use tauri::{Manager, State};

#[tauri::command]
fn open_app(path: String) -> Result<(), String> {
    if let Err(_) = open(path) {
        return Err("Could not open app".into());
    }

    Ok(())
}

#[tauri::command]
fn load_data(db: State<PathBuf>) -> Result<Vec<Game>, String> {
    Ok(db::get_all(&db)?)
}

#[tauri::command]
fn update_data(games: Vec<Game>, db: State<PathBuf>) -> Result<(), String> {
    db::save_new_data(&db, games)?;
    Ok(())
}

enum ImageType {
    Icon = 1,
    Background = 2,
}

#[tauri::command]
fn save_image(
    image: Vec<u8>,
    id: u64,
    image_type: u8,
    old_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("{}", old_path);
    let Some(data_dir) = app_handle.path_resolver().app_data_dir() else {
        return Err("Couldn't find data directory".to_string());
    };

    if Path::new(&old_path).exists() {
        remove_file(old_path).ok();
    }

    let img_path: PathBuf;
    let rand_path_number = rand::thread_rng().gen_range(0..10000);
    match image_type {
        x if x == ImageType::Icon as u8 => {
            img_path =
                data_dir
                    .join("icons")
                    .join(format!("{}-{}.png", id.to_string(), rand_path_number));
        }
        x if x == ImageType::Background as u8 => {
            img_path = data_dir.join("backgrounds").join(format!(
                "{}-{}.png",
                id.to_string(),
                rand_path_number
            ));
        }
        _ => return Err("Invalid image type".to_string()),
    }
    write(&img_path, image).map_err(|err| err.to_string())?;

    Ok(img_path.to_string_lossy().to_string())
}

fn prepare_directories(path: &PathBuf) {
    if !Path::new(path).exists() {
        create_dir(path).unwrap();
    };
    let background_directory = PathBuf::from(path.to_string_lossy().to_string() + "/backgrounds");
    if !Path::new(&background_directory).exists() {
        create_dir(&background_directory).unwrap();
    }

    let icon_directory = PathBuf::from(path.to_string_lossy().to_string() + "/icons");
    if !Path::new(&icon_directory).exists() {
        create_dir(&icon_directory).unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let Some(resource_path) = app.path_resolver().app_data_dir() else {
                panic!("Cannot find path for data folder");
            };

            prepare_directories(&resource_path);
            if let Ok(db_path) = db::prepare_db(&resource_path) {
                app.manage(db_path);
            } else {
                panic!("Could not create db");
            };

            let Some(main_window) = app.get_window("main") else {
                return Ok(());
            };

            main_window.maximize()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_app,
            load_data,
            update_data,
            save_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
