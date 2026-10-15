// alert_kind: `k` → caption/colour/melody/acts.
//   pio test -d firmware -e native -f test_alert_kind
#include <unity.h>
#include <string.h>
#include <initializer_list>
#include "alert_kind.h"
#include "../../src/lang.cpp"

void setUp(void) {}
void tearDown(void) {}

static void test_parse(void) {
    TEST_ASSERT_EQUAL_INT(AK_NONE, alert_kind_parse(""));
    TEST_ASSERT_EQUAL_INT(AK_NONE, alert_kind_parse(nullptr));
    TEST_ASSERT_EQUAL_INT(AK_NONE, alert_kind_parse("nope"));
    TEST_ASSERT_EQUAL_INT(AK_PERMISSION, alert_kind_parse("permission"));
    TEST_ASSERT_EQUAL_INT(AK_PERMISSION, alert_kind_parse("other"));   // follows permission, like the app
    TEST_ASSERT_EQUAL_INT(AK_DONE, alert_kind_parse("done"));
    TEST_ASSERT_EQUAL_INT(AK_CAL_START, alert_kind_parse("calstart"));
    TEST_ASSERT_EQUAL_INT(AK_MESSAGE, alert_kind_parse("msg"));
}

static void test_every_kind_has_a_caption_or_shows_its_message(void) {
    for (const UiText* t : { &UI_TEXT_EN, &UI_TEXT_RU })
        for (int k = 0; k < AK_COUNT; k++) {
            const bool own = alert_kind_shows_message((AlertKind)k);
            TEST_ASSERT_EQUAL_MESSAGE(own, t->alert_caption[k][0] == '\0', t->code);
        }
}

static void test_melodies_and_acts(void) {
    TEST_ASSERT_EQUAL_INT(MELODY_PERM, alert_kind_melody(AK_PERMISSION));
    TEST_ASSERT_EQUAL_INT(MELODY_DONE, alert_kind_melody(AK_DONE));
    TEST_ASSERT_EQUAL_INT(MELODY_DING, alert_kind_melody(AK_NONE));
    const char *a, *b;
    alert_kind_acts(AK_DONE, &a, &b);
    TEST_ASSERT_EQUAL_STRING("dancing", a);
    alert_kind_acts(AK_PERMISSION, &a, &b);
    TEST_ASSERT_EQUAL_STRING("pointing", a);
    TEST_ASSERT_EQUAL_STRING("waving", b);
}

int main(int, char**) {
    UNITY_BEGIN();
    RUN_TEST(test_parse);
    RUN_TEST(test_every_kind_has_a_caption_or_shows_its_message);
    RUN_TEST(test_melodies_and_acts);
    return UNITY_END();
}
