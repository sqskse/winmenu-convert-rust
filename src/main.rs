#![windows_subsystem = "windows"] // Скрываем консоль

use std::env;
use std::path::Path;
use std::process::Command;
use std::os::windows::process::CommandExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // args[1] - путь к файлу, args[2] - формат (mp4, jpg и т.д.)
    if args.len() < 3 {
        return; // Если запустили без аргументов, просто выходим
    }

    let input_path = Path::new(&args[1]);
    let target_format = &args[2];

    // Защита: если формат совпадает с текущим, ничего не делаем
    let current_ext = input_path.extension().unwrap_or_default().to_str().unwrap_or("").to_lowercase();
    if current_ext == *target_format {
        return;
    }

    let output_path = input_path.with_extension(target_format);

    // Запускаем FFmpeg в фоне
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input_path.to_str().unwrap(), output_path.to_str().unwrap(), "-y"]);
    cmd.creation_flags(0x08000000); // Скрываем окно FFmpeg

    // Ждем завершения и удаляем старый файл
    if let Ok(status) = cmd.status() {
        if status.success() {
            let _ = std::fs::remove_file(input_path);
        }
    }
}