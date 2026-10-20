//! Russian for the pages part of the app, keyed by the English text.

pub(super) fn lookup(en: &str) -> Option<&'static str> {
    Some(match en {
        // The window and its sidebar.
        "Overview" => "Обзор",
        "Device" => "Устройство",
        "Flash" => "Прошивка",
        "Accounts" => "Аккаунты",
        "Hooks" => "Хуки",
        "Settings" => "Настройки",
        "Setup" => "Настройка",
        "Created by" => "Автор",
        "Version {}" => "Версия {}",
        "Close window" => "Закрыть окно",
        "Start Clawdmeter from your app menu, or run clawdmeter in a terminal." => {
            "Запустите Clawdmeter из меню приложений или командой clawdmeter в терминале."
        }

        // Overview.
        "{} s" => "{} с",
        "{} min" => "{} мин",
        "{} min {} s" => "{} мин {} с",
        "{} h {} min" => "{} ч {} мин",
        "Bluetooth unavailable" => "Bluetooth недоступен",
        "Connected" => "Подключено",
        "Usage out of date" => "Расход устарел",
        "Looking for Clawdmeter" => "Поиск Clawdmeter",
        "Signed out of Claude Code" => "Выполнен выход из Claude Code",
        "Claude needs you" => "Клод ждёт вас",
        "Resets in {}m" => "Сброс через {} мин",
        "Resets in {}h {}m" => "Сброс через {} ч {} мин",
        "Resets in {}d {}h" => "Сброс через {} д {} ч",
        "Not running" => "Не запущен",
        "Open Claude Code in a terminal and sign in." => "Откройте Claude Code в терминале и войдите в аккаунт.",
        "{} · updated {}" => "{} · обновлено {}",
        "No usage data yet" => "Данных о расходе пока нет",
        "Run setup guide again" => "Запустить мастер настройки снова",
        "Play a chime on this computer when Claude needs you" => "Звуковой сигнал на этом компьютере, когда Клод ждёт вас",
        "Follows the alert types on the Hooks page. Mute from the tray until restart." => {
            "Звучит для типов оповещений со страницы «Хуки». Выключить до перезапуска можно из трея."
        }
        "Session" => "Сессия",
        "Weekly" => "Неделя",
        "Not started" => "Не начата",
        "Firmware v{} is available. Connect USB to update." => "Доступна прошивка v{}. Подключите USB, чтобы обновить.",
        "Update firmware" => "Обновить прошивку",
        "Checking…" => "Проверка…",
        "Can check again in {}" => "Повторная проверка через {}",
        "Usage paused by Anthropic until {}" => "Anthropic приостановил запросы расхода до {}",
        "Refresh usage" => "Обновить расход",

        // The setup wizard.
        "Welcome" => "Начало",
        "Your board" => "Плата",
        "Pair" => "Подключение",
        "Done" => "Готово",
        "Set up Clawdmeter" => "Настройка Clawdmeter",
        "See your Claude Code usage on your desk. Clawd reacts to activity and alerts you when a session needs an answer." => {
            "Расход Claude Code — прямо у вас на столе. Клод реагирует на активность и оповещает, когда сессии нужен ваш ответ."
        }
        "Connect your board with a USB data cable. Setup installs firmware, pairs Bluetooth, and optionally adds Claude Code hooks. Claude Code, or the Claude desktop app, must be installed and signed in." => {
            "Подключите плату USB-кабелем с передачей данных. Мастер установит прошивку, подключит плату по Bluetooth и при желании добавит хуки Claude Code. Claude Code или десктопное приложение Claude должны быть установлены, а вход в них — выполнен."
        }
        "Usage will be read with {}." => "Расход будет считываться через {}.",
        "Claude Code wasn't found, so usage will update every 12 minutes. Install Claude Code or the Claude desktop app for updates while you work." => {
            "Claude Code не найден, поэтому расход будет обновляться раз в 12 минут. Установите Claude Code или десктопное приложение Claude, чтобы данные обновлялись во время работы."
        }
        "Open System Settings → Bluetooth and click Connect next to Clawdmeter." => {
            "Откройте Системные настройки → Bluetooth и нажмите «Подключить» рядом с Clawdmeter."
        }
        "Open Settings → Bluetooth & devices → Add device → Bluetooth, and choose Clawdmeter." => {
            "Откройте Параметры → Bluetooth и устройства → Добавить устройство → Bluetooth и выберите Clawdmeter."
        }
        "Open your desktop's Bluetooth settings and pair Clawdmeter. From a terminal: bluetoothctl, then scan on, pair <address> and trust <address>." => {
            "Откройте настройки Bluetooth в вашем окружении рабочего стола и подключите Clawdmeter. Из терминала: bluetoothctl, затем scan on, pair <адрес> и trust <адрес>."
        }
        "Waiting for Bluetooth connection" => "Ожидание подключения по Bluetooth",
        "The background process isn't running. Pair the device, then start Clawdmeter to check the connection." => {
            "Фоновый процесс не запущен. Подключите устройство, затем запустите Clawdmeter, чтобы проверить связь."
        }
        "Pair over Bluetooth" => "Подключение по Bluetooth",
        "Pair Clawdmeter in your computer's Bluetooth settings. It appears as a keyboard so its buttons can send key presses." => {
            "Подключите Clawdmeter в настройках Bluetooth компьютера. Устройство определится как клавиатура — так его кнопки смогут отправлять нажатия клавиш."
        }
        "Add Claude Code hooks" => "Добавьте хуки Claude Code",
        "Hooks let Clawd react to activity and show alerts. They store event metadata locally, plus notification or question text for alerts." => {
            "Хуки позволяют Клоду реагировать на активность и показывать оповещения. Они хранят локально метаданные событий, а для оповещений — ещё и текст уведомления или вопроса."
        }
        "Hooks installed in your account" => "Хуки установлены в ваш аккаунт",
        "Hooks installed in all {} accounts" => "Хуки установлены во все аккаунты ({})",
        "Latest firmware installed" => "Установлена последняя прошивка",
        "Firmware up to date" => "Прошивка актуальна",
        "Firmware not installed in this setup. Use Flash to install it." => {
            "Прошивка в этот раз не установлена. Установить её можно на странице «Прошивка»."
        }
        "Bluetooth connected" => "Bluetooth подключён",
        "Not connected. Pair in Bluetooth settings." => "Не подключено. Подключите в настройках Bluetooth.",
        " Couldn't read 1 account." => " Не удалось прочитать 1 аккаунт.",
        " Couldn't read {} accounts." => " Не удалось прочитать аккаунтов: {}.",
        "Hooks not installed. Add them from Hooks." => "Хуки не установлены. Добавить их можно на странице «Хуки».",
        "Hooks installed in {} of {} accounts.{}" => "Аккаунтов с хуками: {} из {}.{}",
        "Keep this window open to update usage." => "Не закрывайте это окно, чтобы расход обновлялся.",
        "Clawdmeter keeps running in the tray. Open it there to view usage or settings." => {
            "Clawdmeter продолжит работать в трее. Откройте его оттуда, чтобы посмотреть расход или настройки."
        }
        "Setup summary" => "Итоги настройки",
        "Paired with another computer before? Hold PWR for 3 seconds, then release to clear the old pairing." => {
            "Плата уже была подключена к другому компьютеру? Удерживайте PWR 3 секунды и отпустите, чтобы сбросить старое подключение."
        }
        "Paired with another computer before? While the screen shows the pairing hint, hold a finger on it for 5 seconds to clear the old pairing." => {
            "Плата уже была подключена к другому компьютеру? Пока на экране подсказка о подключении, удерживайте на нём палец 5 секунд, чтобы сбросить старое подключение."
        }
        "Paired with another computer before? Hold PWR for 3 seconds, then release to clear the old pairing. On boards without a PWR button, hold the screen for 5 seconds while it shows the pairing hint." => {
            "Плата уже была подключена к другому компьютеру? Удерживайте PWR 3 секунды и отпустите, чтобы сбросить старое подключение. На платах без кнопки PWR удерживайте палец на экране 5 секунд, пока на нём подсказка о подключении."
        }
        "Back" => "Назад",
        "Start setup" => "Начать настройку",
        "Skip this step" => "Пропустить шаг",
        "Skip hooks" => "Пропустить хуки",
        "Continue without pairing" => "Продолжить без подключения",
        "Continue" => "Далее",
        "Open Clawdmeter" => "Открыть Clawdmeter",
        "Skip setup" => "Пропустить настройку",
        _ => return None,
    })
}
