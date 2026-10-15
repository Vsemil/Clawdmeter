#include "ui_text.h"
#include <string.h>

// The language tables (ui_text.h). Pure data and lookups: no Arduino, so
// test_layout can size every panel for each language.

#define ELLIPSIS "\xE2\x80\xA6"
#define SPINNER  "\xE2\x9C\xBB "      // the status line's resting glyph
#define COUNT(a) (int)(sizeof(a) / sizeof((a)[0]))

static const char* const WORDS_EN[] = {
    "Accomplishing", "Elucidating", "Perusing",
    "Actioning", "Enchanting", "Philosophising",
    "Actualizing", "Envisioning", "Pondering",
    "Baking", "Finagling", "Pontificating",
    "Booping", "Flibbertigibbeting", "Processing",
    "Brewing", "Forging", "Puttering",
    "Calculating", "Forming", "Puzzling",
    "Cerebrating", "Frolicking", "Reticulating",
    "Channelling", "Generating", "Ruminating",
    "Churning", "Germinating", "Scheming",
    "Clauding", "Hatching", "Schlepping",
    "Coalescing", "Herding", "Shimmying",
    "Cogitating", "Honking", "Shucking",
    "Combobulating", "Hustling", "Simmering",
    "Computing", "Ideating", "Smooshing",
    "Concocting", "Imagining", "Spelunking",
    "Conjuring", "Incubating", "Spinning",
    "Considering", "Inferring", "Stewing",
    "Contemplating", "Jiving", "Sussing",
    "Cooking", "Manifesting", "Synthesizing",
    "Crafting", "Marinating", "Thinking",
    "Creating", "Meandering", "Tinkering",
    "Crunching", "Moseying", "Transmuting",
    "Deciphering", "Mulling", "Unfurling",
    "Deliberating", "Mustering", "Unravelling",
    "Determining", "Musing", "Vibing",
    "Discombobulating", "Noodling", "Wandering",
    "Divining", "Percolating", "Whirring",
    "Doing", "Wibbling",
    "Effecting", "Wizarding",
    "Working", "Wrangling",
};

static const char* const WORDS_RU[] = {
    "Вершит", "Проясняет", "Штудирует",
    "Действует", "Чарует", "Философствует",
    "Воплощает", "Прозревает", "Раздумывает",
    "Печёт", "Хитрит", "Вещает",
    "Бупает", "Балаболит", "Обрабатывает",
    "Варит", "Куёт", "Возится",
    "Считает", "Формирует", "Ломает голову",
    "Мозгует", "Резвится", "Ретикулирует",
    "Транслирует", "Генерирует", "Пережёвывает",
    "Взбивает", "Проращивает", "Замышляет",
    "Клодит", "Высиживает", "Тащит",
    "Сплавляет", "Пасёт", "Пританцовывает",
    "Кумекает", "Бибикает", "Лущит",
    "Комбобулирует", "Шустрит", "Томит",
    "Вычисляет", "Придумывает", "Мнёт",
    "Стряпает", "Воображает", "Копает вглубь",
    "Колдует", "Инкубирует", "Крутит",
    "Взвешивает", "Умозаключает", "Тушит",
    "Созерцает", "Джайвит", "Просекает",
    "Готовит", "Манифестирует", "Синтезирует",
    "Мастерит", "Маринует", "Думает",
    "Творит", "Петляет", "Ковыряется",
    "Хрустит", "Бредёт", "Трансмутирует",
    "Расшифровывает", "Обмозговывает", "Разворачивает",
    "Совещается", "Собирается", "Распутывает",
    "Определяет", "Грезит", "Вайбит",
    "Дискомбобулирует", "Наигрывает", "Блуждает",
    "Гадает", "Процеживает", "Жужжит",
    "Делает", "Колышется",
    "Осуществляет", "Волшебничает",
    "Работает", "Укрощает",
};

const UiText UI_TEXT_EN = {
    "en",
    "Usage",
    { "To pair", "hold the power button", "for 3 seconds, then release" },
    { "To pair", "touch and hold here", "for 5 seconds" },
    { "Ready to pair", "choose Clawdmeter", "on your computer" },
    "keep holding...",
    { "Session", "Weekly", "Spending", "Period" },
    { "5h", "7d", "Spend", "Period" },
    "Resets in %dm", "Resets in %dh %dm", "Resets in %dd %dh", "Not started",
    { "Resets in %dm", "Resets in %dh %dm", "Resets in %dd %dh" },
    "of spending limit", "Under pace", "On pace", "Over pace", "Resets %s",
    { "Resets in 23h 59m", "Resets in 6d 23h", "of spending limit" },
    { "Resets in 23h 59m", "Resets in 6d 23h", "of spending limit" },
    "Under pace - Resets May 30",
    "Resets May 30",
    "Waiting", "Listening", "Connected", "No data", "Stale",
    { SPINNER "Connected" ELLIPSIS, SPINNER "Listening" ELLIPSIS,
      SPINNER "Waiting" ELLIPSIS,   SPINNER "No data" ELLIPSIS,
      SPINNER "Stale" ELLIPSIS },
    { "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat" },
    { "", "Permission needed", "Question for you", "Waiting for you", "MCP request",
      "Done!", "Limit almost hit!", "Limits refreshed!", "Meeting soon!", "Meeting started!", "" },
    WORDS_EN, COUNT(WORDS_EN),
};

const UiText UI_TEXT_RU = {
    "ru",
    "Лимиты",
    { "Сопряжение", "зажмите кнопку питания", "на 3 секунды и отпустите" },
    { "Сопряжение", "коснитесь и держите", "5 секунд" },
    { "Можно сопрягать", "выберите Clawdmeter", "на компьютере" },
    "держите дальше...",
    { "Сессия", "Неделя", "Расходы", "Период" },
    { "5ч", "7д", "Расх.", "Пер." },
    "До сброса %dм", "До сброса %dч %dм", "До сброса %dд %dч", "Не начата",
    { "Через %dм", "Через %dч %dм", "Через %dд %dч" },
    "от лимита трат", "Ниже темпа", "В темпе", "Выше темпа", "Сброс %s",
    { "До сброса 23ч 59м", "До сброса 6д 23ч", "от лимита трат" },
    { "Через 23ч 59м", "Через 6д 23ч", "от лимита трат" },
    "Ниже темпа - Сброс 30 мая",
    "Сброс 30 мая",
    "Ожидание", "Слушает", "На связи", "Нет цифр", "Устарело",
    { SPINNER "На связи" ELLIPSIS, SPINNER "Слушает" ELLIPSIS,
      SPINNER "Ожидание" ELLIPSIS,   SPINNER "Нет цифр" ELLIPSIS,
      SPINNER "Устарело" ELLIPSIS },
    { "вс", "пн", "вт", "ср", "чт", "пт", "сб" },
    { "", "Нужно разрешение", "Вопрос к вам", "Клод ждёт ответа", "Запрос MCP",
      "Готово!", "Лимит близко!", "Лимиты обновились!", "Скоро встреча!", "Встреча началась!", "" },
    WORDS_RU, COUNT(WORDS_RU),
};

static const UiText* const ALL[] = { &UI_TEXT_EN, &UI_TEXT_RU };

const UiText* S = &UI_TEXT_EN;

const UiText* lang_find(const char* code) {
    if (!code || !*code) return nullptr;
    for (const UiText* t : ALL)
        if (!strcmp(code, t->code)) return t;
    return nullptr;
}

void lang_select(const UiText* t) {
    if (t) S = t;
}
