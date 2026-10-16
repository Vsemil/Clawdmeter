// volume_swipe: a vertical swipe sets the loudness; a short touch stays a tap.
//   pio test -d firmware -e native -f test_volume_swipe
#include <unity.h>
#include "volume_swipe.h"

void setUp(void) {}
void tearDown(void) {}

static void test_a_small_wobble_is_a_tap(void) {
    VolumeSwipe s = {};
    volume_swipe_press(&s, 200, 50);
    TEST_ASSERT_FALSE(volume_swipe_move(&s, 200 - (VOLUME_SWIPE_START_PX - 1)));
    TEST_ASSERT_FALSE(volume_swipe_release(&s));
    TEST_ASSERT_EQUAL_UINT8(50, s.level);
}

static void test_up_is_louder_down_is_quieter(void) {
    VolumeSwipe s = {};
    volume_swipe_press(&s, 300, 50);
    TEST_ASSERT_TRUE(volume_swipe_move(&s, 300 - 2 * VOLUME_STEP_PX));
    TEST_ASSERT_EQUAL_UINT8(70, s.level);
    TEST_ASSERT_TRUE(volume_swipe_move(&s, 300 + VOLUME_STEP_PX));   // back down past the start
    TEST_ASSERT_EQUAL_UINT8(40, s.level);
    TEST_ASSERT_TRUE(volume_swipe_release(&s));
}

static void test_clamped(void) {
    VolumeSwipe s = {};
    volume_swipe_press(&s, 400, 90);
    volume_swipe_move(&s, 400 - 5 * VOLUME_STEP_PX);
    TEST_ASSERT_EQUAL_UINT8(100, s.level);
    volume_swipe_move(&s, 400 + 20 * VOLUME_STEP_PX);
    TEST_ASSERT_EQUAL_UINT8(0, s.level);
}

int main(int, char**) {
    UNITY_BEGIN();
    RUN_TEST(test_a_small_wobble_is_a_tap);
    RUN_TEST(test_up_is_louder_down_is_quieter);
    RUN_TEST(test_clamped);
    return UNITY_END();
}
