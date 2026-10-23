use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // Удаляем из профиля пользователя
    if let Ok(classes) = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS) {
        let _ = classes.delete_subkey_all(r"*\shell\ConvertVideo");
        let _ = classes.delete_subkey_all(r"MyConverterMenu");
    }

    // На всякий случай чистим системный реестр (если туда что-то попало)
    let _ = hkcr.delete_subkey_all(r"*\shell\ConvertVideo");
    let _ = hkcr.delete_subkey_all(r"MyConverterMenu");

    println!("Меню успешно удалено!");
    Ok(())
}