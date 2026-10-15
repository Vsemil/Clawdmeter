// melody: the alert melodies, strikes of the ding at other pitches.
//   pio test -d firmware -e native -f test_melody
#include <unity.h>
#include <math.h>
#include "ding.h"
#include "melody.h"

static const uint32_t RATE = 44100;

void setUp(void) {}
void tearDown(void) {}

static float peak(Melody m, uint32_t a, uint32_t b) {
    float p = 0;
    for (uint32_t n = a; n < b; n++) p = fmaxf(p, fabsf(melody_value(m, n, RATE)));
    return p;
}

static void test_ding_is_the_ding(void) {
    TEST_ASSERT_EQUAL_UINT32(ding_frames(RATE), melody_frames(MELODY_DING, RATE));
    for (uint32_t n = 0; n < RATE; n += 997)
        TEST_ASSERT_EQUAL_FLOAT(ding_value(n, RATE), melody_value(MELODY_DING, n, RATE));
    // The generalized strike at the ding's pitch is the ding.
    for (uint32_t n = 0; n < RATE; n += 997)
        TEST_ASSERT_EQUAL_FLOAT(ding_value(n, RATE), ding_note_value(n, RATE, 554.5f));
}

static void test_length_is_the_last_strike_plus_its_ring(void) {
    // perm: strikes at 0, 170, 340 ms.
    TEST_ASSERT_EQUAL_UINT32(340 * RATE / 1000 + ding_frames(RATE), melody_frames(MELODY_PERM, RATE));
    TEST_ASSERT_EQUAL_FLOAT(0.0f, melody_value(MELODY_PERM, melody_frames(MELODY_PERM, RATE), RATE));
    for (int m = 0; m < MELODY_COUNT; m++)
        TEST_ASSERT_TRUE(melody_frames((Melody)m, RATE) <= melody_max_frames(RATE));
    TEST_ASSERT_EQUAL_UINT32(0, melody_frames(MELODY_COUNT, RATE));
}

static void test_every_strike_is_heard_and_nothing_clips(void) {
    for (int m = 1; m < MELODY_COUNT; m++) {
        const Melody mel = (Melody)m;
        const float p = peak(mel, 0, melody_frames(mel, RATE));
        TEST_ASSERT_TRUE_MESSAGE(p > 0.5f && p <= 1.0f, melody_name(mel));
        // A strike lands after silence-ish: the second one's onset is loud.
        TEST_ASSERT_TRUE_MESSAGE(peak(mel, RATE / 10, RATE / 2) > 0.3f, melody_name(mel));
    }
}

static void test_names_round_trip(void) {
    for (int m = 0; m < MELODY_COUNT; m++)
        TEST_ASSERT_EQUAL_INT(m, melody_by_name(melody_name((Melody)m)));
    TEST_ASSERT_EQUAL_INT(MELODY_COUNT, melody_by_name("nope"));
    TEST_ASSERT_EQUAL_INT(MELODY_COUNT, melody_by_name(nullptr));
}

int main(int, char**) {
    UNITY_BEGIN();
    RUN_TEST(test_ding_is_the_ding);
    RUN_TEST(test_length_is_the_last_strike_plus_its_ring);
    RUN_TEST(test_every_strike_is_heard_and_nothing_clips);
    RUN_TEST(test_names_round_trip);
    return UNITY_END();
}
