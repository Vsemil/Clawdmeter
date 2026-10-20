//! Strings shared across pages.

pub(super) fn lookup(en: &str) -> Option<&'static str> {
    Some(match en {
        "the Claude desktop app's Claude Code {}" => "Claude Code {} из приложения Claude",
        "the Claude desktop app's Claude Code" => "Claude Code из приложения Claude",
        "{} s ago" => "{} с назад",
        "{} min ago" => "{} мин назад",
        "{} h ago" => "{} ч назад",
        // Sentences of the core's usage errors (`i18n::core_message`).
        "Usage paused by Anthropic until {}." => "Anthropic приостановил запросы расхода до {}.",
        "Trying again in about {} min." => "Повтор примерно через {} мин.",
        "Retrying." => "Повторяем.",
        "Couldn't run Claude Code. Check its installation and the claude_path setting." => {
            "Не удалось запустить Claude Code. Проверьте установку и настройку claude_path."
        }
        "Claude Code took too long to answer." => "Claude Code слишком долго не отвечал.",
        "Couldn't read Claude Code's usage response." => "Не удалось разобрать ответ Claude Code о расходе.",
        "Sign in to Claude Code for this account folder." => "Войдите в Claude Code для этой папки аккаунта.",
        "This account doesn't report subscription usage limits." => "Этот аккаунт не сообщает лимиты подписки.",
        "No usage data was returned." => "Данные о расходе не пришли.",
        "Usage requests are temporarily limited." => "Запросы расхода временно ограничены.",
        "Waiting for Claude Code to renew its sign-in." => "Ждём, пока Claude Code обновит вход.",
        "Using Claude Code or the Claude desktop app renews it." => {
            "Вход обновится, когда вы воспользуетесь Claude Code или приложением Claude."
        }
        "Couldn't reach Claude." => "Не удалось связаться с Claude.",
        _ => return None,
    })
}
