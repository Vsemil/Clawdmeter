//! Russian for the tray part of the app, keyed by the English text.

pub(super) fn lookup(en: &str) -> Option<&'static str> {
    Some(match en {
        // Tray menu and tooltip.
        "Starting" => "Запуск",
        "Bluetooth unavailable" => "Bluetooth недоступен",
        "{} · updated {}" => "{} · обновлено {}",
        "Open Claude Code to sign in" => "Откройте Claude Code, чтобы войти",
        "No usage data yet" => "Данных о расходе пока нет",
        "Session  {}%" => "Сессия  {}%",
        "Session  not started" => "Сессия  не началась",
        "Session  –" => "Сессия  –",
        "Weekly  {}%" => "Неделя  {}%",
        "Weekly  –" => "Неделя  –",
        "Update firmware to {}…" => "Обновить прошивку до {}…",
        "Flash firmware…" => "Прошить устройство…",
        "Refresh usage" => "Обновить расход",
        "Mute chimes" => "Без звука",
        "Open Clawdmeter" => "Открыть Clawdmeter",
        "Settings…" => "Настройки…",
        "Quit Clawdmeter" => "Выйти из Clawdmeter",
        " · Session {}% · Weekly {}%" => " · сессия {}% · неделя {}%",
        // Tray states (`TrayState::label`).
        "Connected" => "Подключено",
        "Usage out of date" => "Данные о расходе устарели",
        "Looking for device" => "Поиск устройства",
        "Signed out" => "Вход не выполнен",
        "Claude needs you" => "Клод ждёт вас",
        // errors.rs.
        "Write permission denied." => "Нет прав на запись.",
        "The file or folder was not found." => "Файл или папка не найдены.",
        "The disk is read-only." => "Диск доступен только для чтения.",
        "The disk is full." => "Диск заполнен.",
        "Couldn't save {} ({}). {}" => "Не удалось сохранить {} ({}). {}",
        // `what` in `errors::save_failed`.
        "settings" => "настройки",
        // hooks/install.rs: settings.json problems.
        "settings.json contains invalid JSON. Fix the file, then retry. Nothing was written." => {
            "В settings.json некорректный JSON. Исправьте файл и повторите. Ничего не записано."
        }
        "Unsupported settings.json layout: {}. Nothing was written." => {
            "Неподдерживаемая структура settings.json: {}. Ничего не записано."
        }
        "the top level isn't an object" => "верхний уровень — не объект",
        "\"hooks\" isn't an object" => "«hooks» — не объект",
        "a hook event isn't a list" => "событие хука — не список",
        // Board summaries (`BoardInfo::summary_text`), one part at a time.
        "1 button" => "1 кнопка",
        "2 buttons" => "2 кнопки",
        "3 buttons" => "3 кнопки",
        "watch" => "часы",
        "round" => "круглый",
        "RGB panel" => "RGB-панель",
        "e-paper" => "электронная бумага",
        "touch only" => "только сенсор",
        "no touch" => "без сенсора",
        "resistive touch" => "резистивный сенсор",
        "capacitive touch" => "ёмкостный сенсор",
        _ => return None,
    })
}
