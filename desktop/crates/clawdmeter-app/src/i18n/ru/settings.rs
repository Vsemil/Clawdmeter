//! Russian for the settings part of the app, keyed by the English text.

pub(super) fn lookup(en: &str) -> Option<&'static str> {
    Some(match en {
        // Device
        "Device" => "Устройство",
        "Board not identified" => "Плата не определена",
        "Last detected" => "Определена ранее",
        "{} · last detected" => "{} · определена ранее",
        "Clawdmeter isn't running" => "Clawdmeter не запущен",
        "Bluetooth connected" => "Bluetooth подключён",
        "Bluetooth unavailable" => "Bluetooth недоступен",
        "Connecting…" => "Подключение…",
        "Looking for Clawdmeter" => "Поиск Clawdmeter",
        "Checking USB…" => "Проверка USB…",
        "USB: {}" => "USB: {}",
        "Connect USB to identify the board." => "Подключите USB, чтобы определить плату.",
        "USB not connected" => "USB не подключён",
        "Firmware v{} · v{} available" => "Прошивка v{} · доступна v{}",
        "Firmware v{}" => "Прошивка v{}",
        "Firmware version unknown" => "Версия прошивки неизвестна",
        "Update firmware" => "Обновить прошивку",
        "Flash firmware…" => "Прошить…",
        "Clock" => "Часы",
        "Replaces the Usage title with a clock. Auto follows this computer's time format." => {
            "Показывает часы вместо заголовка «Расход». «Авто» — в формате времени этого компьютера."
        }
        "Off" => "Выкл",
        "Auto" => "Авто",
        "12 h" => "12 ч",
        "24 h" => "24 ч",
        "Animation silliness" => "Озорство анимации",
        "How lively Clawd is" => "Насколько резвится Клод",
        "No-nonsense" => "Без шалостей",
        "Some-nonsense" => "Немного шалостей",
        "All-nonsense" => "Сплошные шалости",
        "Still in his corner, a new verb once a minute. Taps and alerts still get a reaction." => {
            "Тихо сидит в углу, новое слово раз в минуту. На касания и оповещения всё равно реагирует."
        }
        "Mostly still: fewer corner acts, and shows only after long busy stretches." => {
            "Почти спокоен: реже выступает в углу, а шоу устраивает только после долгой работы."
        }
        "Corner acts now and then, a show after a busy stretch, a screensaver while you're away." => {
            "Иногда выступает в углу, после напряжённой работы — шоу, пока вас нет — заставка."
        }
        "Livelier: more corner acts, and shows come sooner and more often." => {
            "Бойче: чаще выступает в углу, а шоу начинаются раньше и бывают чаще."
        }
        "Frequent shows and reactions. Even a moderately busy stretch earns a show." => {
            "Частые шоу и реакции. Шоу бывает даже после не самой напряжённой работы."
        }
        "Attention animation" => "Анимация оповещения",
        "How long Clawd points and waves when Claude needs you. The alert stays up either way." => {
            "Сколько Клод показывает на экран и машет, когда вы нужны Claude. Оповещение в любом случае остаётся."
        }
        "2m" => "2 мин",
        "30s" => "30 с",
        "10s" => "10 с",
        "Language" => "Язык",
        "The device's interface language. It restarts once to lay out the new words." => {
            "Язык интерфейса устройства. При смене оно один раз перезапустится, чтобы перестроить экран."
        }
        "Pace marker" => "Метка темпа",
        "Marks how far into the session and the week you are. Bars turn amber when usage is ahead of it." => {
            "Отмечает, какая часть сессии и недели прошла. Полосы желтеют, когда расход её опережает."
        }
        "Alert chime" => "Звук оповещения",
        "Chimes when Claude needs you, on boards with a speaker. Mute from the tray until restart." => {
            "Звучит, когда вы нужны Claude, на платах с динамиком. В трее можно выключить до перезапуска."
        }
        "Session reset chime" => "Сигнал сброса сессии",
        "Chimes when your session usage resets, on boards with a speaker." => {
            "Звучит, когда расход сессии обнуляется, на платах с динамиком."
        }
        "Check for firmware updates" => "Проверять обновления прошивки",
        "Checks GitHub daily. Updates appear here and in the tray." => {
            "Раз в день проверяет GitHub. Обновления появляются здесь и в трее."
        }
        "Clear saved device" => "Забыть устройство",
        "Clears the app's saved address and searches for a paired Clawdmeter. Bluetooth pairing stays in place." => {
            "Стирает сохранённый адрес и заново ищет сопряжённый Clawdmeter. Сопряжение Bluetooth сохраняется."
        }
        "Clear" => "Забыть",
        "Clear the board's pairing" => "Сбросить сопряжение платы",
        "Makes the board forget every computer it was paired with, so it can pair again." => {
            "Плата забудет все компьютеры, с которыми была сопряжена, и сможет подключиться заново."
        }
        "Makes the board forget every computer it was paired with. Connect USB to use this." => {
            "Плата забудет все компьютеры, с которыми была сопряжена. Для этого подключите её по USB."
        }
        "Clear pairing" => "Сбросить",
        "The board's pairing is cleared. Remove Clawdmeter from this computer's Bluetooth settings, then pair it again." => {
            "Сопряжение платы сброшено. Удалите Clawdmeter из настроек Bluetooth этого компьютера и подключите \
             его снова."
        }
        "No response from the device. Reconnect USB or update the firmware, then try again." => {
            "Устройство не отвечает. Переподключите USB или обновите прошивку и попробуйте снова."
        }

        // Accounts
        "Accounts" => "Аккаунты",
        "Add account folder…" => "Добавить папку аккаунта…",
        "Add Claude Code account folder" => "Добавить папку аккаунта Claude Code",
        "The default Claude Code folder is ~/.claude. Add a config folder for each other account you want to monitor. The device follows the account whose session usage increased most recently." => {
            "Папка Claude Code по умолчанию — ~/.claude. Добавьте папку конфигурации для каждого другого \
             аккаунта, который нужно отслеживать. Устройство показывает аккаунт, у которого расход сессии рос \
             последним."
        }
        "Usage is read with {}." => "Расход считывается через {}.",
        "Claude Code not found, usage updates every 12 min." => {
            "Claude Code не найден, расход обновляется раз в 12 мин."
        }
        "Retrying" => "Повтор",
        "Needs attention" => "Требует внимания",
        "On device" => "На устройстве",
        "Not selected for the device" => "Не выбран для устройства",
        "Start Clawdmeter to check usage." => "Запустите Clawdmeter, чтобы проверить расход.",
        "Remove" => "Удалить",
        "Removing a folder stops monitoring it; its files and installed hooks stay in place. The last folder can't be removed." => {
            "Удалённая папка больше не отслеживается; её файлы и установленные хуки остаются. Последнюю папку \
             удалить нельзя."
        }
        "Removing a folder stops monitoring it; its files and installed hooks stay in place." => {
            "Удалённая папка больше не отслеживается; её файлы и установленные хуки остаются."
        }

        // Hooks
        "Hooks" => "Хуки",
        "Hooks let Clawd react to activity and show alerts. The event log stores timestamps, event types, and session IDs. Alerts also store notification or question text for the device. Usage meters work without hooks." => {
            "Хуки позволяют Клоду реагировать на активность и показывать оповещения. В журнале событий хранятся \
             время, типы событий и ID сессий. Для оповещений также сохраняется текст уведомления или вопроса \
             для устройства. Шкалы расхода работают и без хуков."
        }
        "{} hooks installed" => "Установлено хуков: {}",
        "Installed" => "Установлены",
        "Review removal" => "Проверить удаление",
        "Clawdmeter hooks are not installed. Other hooks are kept." => {
            "Хуки Clawdmeter не установлены. Другие хуки сохраняются."
        }
        "Not installed" => "Не установлены",
        "Review install" => "Проверить установку",
        "Uses attention-hook.sh. Update to the built-in Clawdmeter hooks." => {
            "Используется attention-hook.sh. Обновите до встроенных хуков Clawdmeter."
        }
        "Legacy hooks" => "Устаревшие хуки",
        "Review update" => "Проверить обновление",
        "Runs {}, {}." => "Запускает {}, {}.",
        "Uses another Clawdmeter executable: {}." => "Использует другой исполняемый файл Clawdmeter: {}.",
        "Runs {}, which no longer exists." => "Запускает {}, которого больше нет.",
        "Out of date" => "Устарели",
        "Some hooks are missing." => "Не хватает некоторых хуков.",
        "Incomplete" => "Неполные",
        "Unreadable" => "Не читается",
        "Couldn't locate the Clawdmeter executable." => "Не удалось найти исполняемый файл Clawdmeter.",
        "Review 1 file" => "Проверьте файл",
        "Review {} files" => "Проверьте файлы ({})",
        "Review hook removal" => "Проверьте удаление хуков",
        "Install hooks" => "Установить хуки",
        "Remove hooks" => "Удалить хуки",
        "Cancel" => "Отмена",
        "Review before applying. Your original settings are kept in settings.json.bak." => {
            "Проверьте перед применением. Исходные настройки сохранятся в settings.json.bak."
        }
        "{} (new file)" => "{} (новый файл)",
        "{} (also re-indented)" => "{} (также изменены отступы)",
        "Review updates for {} accounts" => "Проверить обновление в аккаунтах: {}",
        "Review installation" => "Проверить установку",
        "Review installation in {} accounts" => "Проверить установку в аккаунтах: {}",
        "No changes needed." => "Изменения не нужны.",
        "Couldn't write {}. {}{}" => "Не удалось записать {}. {}{}",
        " 1 other file was already saved." => " Ещё один файл уже сохранён.",
        " {} other files were already saved." => " Другие файлы уже сохранены: {}.",
        "Hooks installed in 1 account. Your original settings are kept in settings.json.bak." => {
            "Хуки установлены в 1 аккаунте. Исходные настройки сохранены в settings.json.bak."
        }
        "Hooks installed in {} accounts. Your original settings are kept in settings.json.bak." => {
            "Хуки установлены в аккаунтах: {}. Исходные настройки сохранены в settings.json.bak."
        }
        "Hooks removed from 1 account. Your original settings are kept in settings.json.bak." => {
            "Хуки удалены из 1 аккаунта. Исходные настройки сохранены в settings.json.bak."
        }
        "Hooks removed from {} accounts. Your original settings are kept in settings.json.bak." => {
            "Хуки удалены из аккаунтов: {}. Исходные настройки сохранены в settings.json.bak."
        }
        "Alert types" => "Типы оповещений",
        "Applies to all accounts, on the device and in the tray." => "Для всех аккаунтов, на устройстве и в трее.",
        "Tool approvals" => "Разрешения на инструменты",
        "Claude needs permission to run a tool." => "Claude нужно разрешение на запуск инструмента.",
        "Questions" => "Вопросы",
        "Claude has a question for you." => "У Claude есть к вам вопрос.",
        "Idle sessions" => "Ожидание запроса",
        "Claude has been waiting for your next prompt." => "Claude ждёт вашего следующего запроса.",
        "MCP requests" => "Запросы MCP",
        "An MCP server needs information from you." => "MCP-серверу нужны от вас данные.",
        "Finished turns" => "Завершённые ответы",
        "Claude finished a turn that took longer than 45 seconds." => {
            "Claude закончил ответ, который занял больше 45 секунд."
        }
        "Send test alert" => "Тестовое оповещение",
        "Sent" => "Отправлено",
        "Clawdmeter isn't running." => "Clawdmeter не запущен.",
        "Couldn't send it. {}" => "Не удалось отправить. {}",

        // Settings
        "Settings" => "Настройки",
        "Start at login" => "Запускать при входе",
        "Opens Clawdmeter when you sign in." => "Открывает Clawdmeter при входе в систему.",
        "Couldn't change Start at login. {}" => "Не удалось изменить «Запускать при входе». {}",
        "Start at login opens {}, {}." => "Автозапуск открывает {}, {}.",
        "Start at login opens {}, which no longer exists." => "Автозапуск открывает {}, которого больше нет.",
        "Start at login opens another copy of Clawdmeter: {}." => {
            "Автозапуск открывает другую копию Clawdmeter: {}."
        }
        "Start at login can't be read." => "Не удалось прочитать запись автозапуска.",
        "Use this copy" => "Использовать эту копию",
        "Clawdmeter is running from its disk image." => "Clawdmeter запущен с образа диска.",
        "Clawdmeter is running from a temporary copy macOS made." => {
            "Clawdmeter запущен из временной копии, созданной macOS."
        }
        "Clawdmeter is running from a temporary folder." => "Clawdmeter запущен из временной папки.",
        // clawdmeter_core::launch::Unstable (passed through tr at runtime).
        "which is on a disk image" => "который находится на образе диска",
        "a temporary copy macOS made" => "временную копию, созданную macOS",
        "which is in a temporary folder" => "который находится во временной папке",
        "Move Clawdmeter to Applications first, then open it from there." => {
            "Сначала переместите Clawdmeter в «Программы» и откройте его оттуда."
        }
        "Extract Clawdmeter from the zip first, then run it from there." => {
            "Сначала распакуйте Clawdmeter из zip-архива и запустите его оттуда."
        }
        "Usage checks" => "Проверка расхода",
        "Automatic checks every 3 minutes while working, then less often when quiet. Retry delays apply to every check." => {
            "«Автоматически» — каждые 3 минуты во время работы, в простое реже. Паузы перед повтором действуют \
             для всех проверок."
        }
        "Automatic" => "Автоматически",
        "While working: 2 min" => "При работе: 2 мин",
        "While working: 5 min" => "При работе: 5 мин",
        "While working: {} s" => "При работе: {} с",
        "Claude Code wasn't found, so usage updates every 12 minutes. Install Claude Code or the Claude desktop app for updates while you work, or set claude_path in the config file." => {
            "Claude Code не найден, поэтому расход обновляется раз в 12 минут. Установите Claude Code или \
             десктопное приложение Claude, чтобы получать обновления во время работы, или укажите claude_path \
             в файле конфигурации."
        }
        "Open config" => "Открыть конфиг",
        "Create config" => "Создать конфиг",
        "Quit Clawdmeter" => "Выйти из Clawdmeter",
        "Stops usage updates and closes Clawdmeter. The device retains its last reading." => {
            "Останавливает обновление расхода и закрывает Clawdmeter. На устройстве останутся последние данные."
        }
        "Quit" => "Выйти",
        "Legacy service detected" => "Найдена старая служба",
        "Disable {} so only one Clawdmeter process connects to the device." => {
            "Отключите {}, чтобы к устройству подключался только один процесс Clawdmeter."
        }
        "Disable legacy service" => "Отключить старую службу",
        "Couldn't disable {}. Remove it by hand." => "Не удалось отключить {}. Удалите вручную.",
        "Advanced" => "Дополнительно",
        "Claude Code executable" => "Исполняемый файл Claude Code",
        "Not found." => "Не найден.",
        "Start Clawdmeter to detect Claude Code." => "Запустите Clawdmeter, чтобы найти Claude Code.",
        "Log file" => "Файл журнала",
        "Open log" => "Открыть журнал",
        "Config file" => "Файл конфигурации",
        "Couldn't read {}: {}" => "Не удалось прочитать {}: {}",
        _ => return None,
    })
}
