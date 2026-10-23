use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;

    let exe_path = std::env::current_exe()?.with_file_name("converter.exe");
    let exe_str = exe_path.to_str().unwrap();

    // 1. Очищаем старые записи (если были)
    let _ = classes.delete_subkey_all(r"*\shell\ConvertVideo");
    let _ = classes.delete_subkey_all(r"MyConverterMenu"); // Удаляем старую структуру подменю

    // 2. Создаем структуру самого подменю
    // Внутри ключа "shell" будут лежать наши форматы
    let (submenu_key, _) = classes.create_subkey(r"MyConverterMenu\shell")?;

    let formats = [
        ("MP4", "mp4"), ("AVI", "avi"), ("MKV", "mkv"), ("WEBM", "webm"),
        ("JPG", "jpg"), ("PNG", "png"), ("WEBP", "webp"), ("GIF", "gif")
    ];

    for (name, ext) in formats {
        // Создаем ключ для конкретного формата (например, "mp4")
        let (fmt_key, _) = submenu_key.create_subkey(ext)?;
        
        // Текст пункта (MP4, AVI и т.д.)
        fmt_key.set_value("MUIVerb", &name)?;
        

        // Сама команда
        let (cmd_key, _) = fmt_key.create_subkey("command")?;
        let command = format!("\"{}\" \"%1\" {}", exe_str, ext);
        cmd_key.set_value("", &command)?;
    }

    // 3. Создаем главный пункт "Конвертировать в"
    let (parent_key, _) = classes.create_subkey(r"*\shell\ConvertVideo")?;
    parent_key.set_value("MUIVerb", &"Конвертировать в")?;

    
    // 4. Привязываем наше подменю к главному пункту
    parent_key.set_value("ExtendedSubCommandsKey", &"MyConverterMenu")?;

    println!("Установщик (ExtendedSubCommandsKey) успешно отработал!");
    Ok(())
}