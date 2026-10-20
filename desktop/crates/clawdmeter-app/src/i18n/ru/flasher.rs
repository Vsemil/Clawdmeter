//! Russian for the flasher part of the app, keyed by the English text.

pub(super) fn lookup(en: &str) -> Option<&'static str> {
    Some(match en {
        // Page and step header.
        "Flash" => "Прошивка",
        "Connect" => "Подключение",
        "Identify" => "Определение",
        "Download" => "Загрузка",
        "Done" => "Готово",

        // Supported boards.
        "Buy ↗" => "Купить ↗",

        // Release and local manifest.
        "Couldn't open {}. {}" => "Не удалось открыть {}. {}",
        "This file isn't a valid Clawdmeter release manifest." => "Это не манифест релиза Clawdmeter.",
        "No firmware release was found in {}." => "В {} не найден релиз прошивки.",
        "Choose a Clawdmeter manifest.json" => "Выберите manifest.json Clawdmeter",
        "Checking for firmware…" => "Ищем прошивку…",
        "{} If you have a release folder, open its manifest.json to flash from it." => {
            "{} Если у вас есть папка с релизом, откройте её manifest.json, чтобы прошить из неё."
        }
        "Open manifest.json…" => "Открыть manifest.json…",
        "Check again" => "Проверить снова",

        // Connect.
        "Connect your board" => "Подключите плату",
        "Connect the board with a USB data cable. A charge-only cable won't work." => {
            "Подключите плату USB-кабелем для передачи данных. Кабель только для зарядки не подойдёт."
        }
        "Waiting for USB" => "Ждём USB",
        "Hide supported boards" => "Скрыть поддерживаемые платы",
        "View supported boards" => "Поддерживаемые платы",

        // Identify.
        "This board has an {}{}. None of the boards in this release match it." => {
            "На плате {}{}. Ни одна плата из этого релиза ей не подходит."
        }
        " with {} MB of flash" => " с {} МБ флеш-памяти",
        "Running Clawdmeter {}" => "Работает Clawdmeter {}",
        "{} MB" => "{} МБ",
        "? MB" => "? МБ",
        "Your board" => "Ваша плата",
        "Choose your board" => "Выберите плату",
        "Identifying board…" => "Определяем плату…",
        "Checking board details. A board without Clawdmeter firmware may restart into its bootloader." => {
            "Проверяем плату. Плата без прошивки Clawdmeter может перезапуститься в загрузчик."
        }
        "Firmware is up to date ({}). Continue, or reinstall if needed." => {
            "Актуальная версия прошивки ({}). Продолжите или прошейте заново, если нужно."
        }
        "Board identified from its Clawdmeter firmware." => "Плата определена по прошивке Clawdmeter.",
        "One board in this release matches the detected chip." => "Этому чипу подходит одна плата из релиза.",
        "{} boards match this chip. Select the model on your desk." => {
            "Этому чипу подходят платы ({}). Выберите свою модель."
        }
        "Reinstall firmware" => "Прошить заново",
        "Continue" => "Продолжить",
        "Back" => "Назад",

        // Firmware.
        "Place {} in the same folder as manifest.json." => "Положите {} в ту же папку, что и manifest.json.",
        "Couldn't read {}. {}" => "Не удалось прочитать {}. {}",
        "Downloading… {}%" => "Загрузка… {}%",
        "Checksum verified" => "Контрольная сумма совпала",
        "Failed" => "Ошибка",
        "Downloading firmware" => "Загружаем прошивку",
        "Ready to install" => "Можно прошивать",
        "Couldn't download firmware" => "Не удалось загрузить прошивку",
        "Board" => "Плата",
        "Firmware" => "Прошивка",
        "Verification" => "Проверка",
        "Details" => "Подробности",
        "File" => "Файл",
        "Source" => "Источник",
        "Install {}" => "Прошить {}",

        // Flash.
        "USB port: {}" => "Порт USB: {}",
        "{} ({} MB), checksum verified" => "{} ({} МБ), контрольная сумма совпала",
        "Restarting the board into its bootloader…" => "Перезапускаем плату в загрузчик…",
        "Writing {} MB…" => "Записываем {} МБ…",
        "Verifying firmware…" => "Проверяем прошивку…",
        "Restarting the board…" => "Перезапускаем плату…",
        "Firmware installed" => "Плата прошита",
        "Installation stopped." => "Прошивка прервана.",
        "Firmware installation failed" => "Не удалось прошить плату",
        "Writing firmware" => "Записываем прошивку",
        "Keep the USB cable connected until installation finishes." => {
            "Не отключайте USB-кабель, пока прошивка не завершится."
        }
        "Start over" => "Начать заново",
        "Retry" => "Повторить",

        // Done.
        "Bluetooth connected." => "Bluetooth подключён.",
        "If the board isn't paired yet, choose Clawdmeter in your Bluetooth settings." => {
            "Если плата ещё не подключена, выберите Clawdmeter в настройках Bluetooth."
        }
        "Waiting for Bluetooth connection" => "Ждём подключения по Bluetooth",
        "Flash another board" => "Прошить другую плату",
        "Device settings" => "Настройки устройства",

        // Board errors (clawdmeter_flash::device::DeviceError).
        "No permission to open {}. Add yourself to the dialout group (uucp on Arch): sudo usermod -aG dialout $USER, then log out and back in." => {
            "Нет доступа к {}. Добавьте себя в группу dialout (uucp в Arch): sudo usermod -aG dialout $USER, затем выйдите из системы и войдите снова."
        }
        "Permission denied for {}." => "Нет доступа к {}.",
        "{} is in use. Close the serial monitor or other app using it, then retry." => {
            "{} занят. Закройте монитор порта или другое приложение, которое его использует, и повторите."
        }
        "The board disconnected. Reconnect USB and retry." => "Плата отключилась. Переподключите USB и повторите.",
        "The board didn't respond. Unplug it, hold BOOT while you plug it back in, then try again." => {
            "Плата не отвечает. Отключите её, зажмите BOOT, подключите снова и повторите."
        }
        "This board has an {}, but that firmware is for an {}. Go back and pick your board again." => {
            "На плате {}, а эта прошивка для {}. Вернитесь и выберите плату заново."
        }
        "Couldn't talk to the board ({})." => "Не удалось связаться с платой ({}).",
        "Flashing stopped ({}). Unplug the board, plug it back in and retry." => {
            "Прошивка прервана ({}). Отключите плату, подключите снова и повторите."
        }
        "{} isn't a USB serial port." => "{} — не USB-порт.",
        "The firmware was written, but the board didn't answer after restarting. Unplug it and plug it back in; if it still doesn't come up, flash it again." => {
            "Прошивка записана, но после перезапуска плата не ответила. Отключите и снова подключите её; если не заработает, прошейте заново."
        }
        "The board restarted, but it reports {} firmware instead of {}. Flash it again." => {
            "Плата перезапустилась, но сообщает о прошивке {} вместо {}. Прошейте её заново."
        }

        // Release errors (clawdmeter_flash::release::ReleaseError).
        "No firmware release was found." => "Релиз прошивки не найден.",
        "Couldn't reach GitHub. Check your internet connection and try again." => {
            "Не удалось связаться с GitHub. Проверьте подключение к интернету и повторите."
        }
        "GitHub is limiting requests from this network. Try again later." => {
            "GitHub ограничивает запросы из этой сети. Повторите позже."
        }
        "Couldn't read the release response from GitHub. Try again later." => {
            "Не удалось прочитать ответ GitHub о релизе. Повторите позже."
        }
        "Couldn't read this release's manifest.json. Check that the app supports this release." => {
            "Не удалось прочитать manifest.json релиза. Проверьте, что приложение поддерживает этот релиз."
        }
        "This release is missing {}." => "В релизе нет файла {}.",
        "Checksum mismatch for {}. Download it again before flashing." => {
            "Контрольная сумма {} не совпала. Загрузите файл заново перед прошивкой."
        }
        "No permission to read or write that file." => "Нет доступа к файлу на чтение или запись.",
        "File not found." => "Файл не найден.",

        _ => return None,
    })
}
