// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::UserDirs;
use youtube_dl::YoutubeDl;

#[derive(Clone, serde::Serialize)]
struct Metadata {
  title: String,
}

#[tauri::command]
async fn download_metadata(url: String) -> Result<Metadata, String> {
  println!("Downloading metadata: {}", url);

  let mut instance = YoutubeDl::new(url);

  let result = instance
    .socket_timeout("15")
    .flat_playlist(true)
    .run_async()
    .await;

  println!("Downloaded metadata");

  match result {
    Ok(metadata) => {
      return Ok(Metadata {
        title: match metadata.clone().into_playlist() {
          Some(playlist) => playlist.title.unwrap().to_string(),
          None => match metadata.into_single_video() {
            Some(video) => video.title.unwrap().to_string(),
            None => return Err("Error getting title".to_string()),
          },
        },
      })
    }
    Err(e) => return Err(e.to_string()),
  }
}

#[tauri::command]
async fn download_video(
  url: String,
  audio_only: bool,
  folder_path: Option<String>,
) -> Result<(), String> {
  println!("Downloading video: {}", url);

  let mut instance = YoutubeDl::new(url);

  if audio_only {
    instance
      .extract_audio(true)
      .extra_arg("--audio-format")
      .extra_arg("mp3");
  } else {
    instance.extra_arg("--merge-output-format").extra_arg("mp4");
  }

  let path = match folder_path {
    Some(path) => path,
    None => match UserDirs::new() {
      Some(user_dirs) => user_dirs.audio_dir().unwrap().to_str().unwrap().to_string(),
      None => return Err("Error getting home path".to_string()),
    },
  };

  let result = instance.socket_timeout("15").download_to_async(path).await;

  match result {
    Ok(_) => {
      println!("Downloaded video");
      return Ok(());
    }
    Err(_) => return Err("Error downloading video".to_string()),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![download_video, download_metadata])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
