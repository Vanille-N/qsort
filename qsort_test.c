// Version: 1.2.0

#include "vendor/unity.h"

#define i64 int64_t
#define usize uint64_t

extern void swap (i64 * tab, usize i, usize j);
extern int choose_pivot (i64 * tab, usize i, usize j);
extern void partition (i64 * tab, usize lo, usize * sm, usize * eq, usize hi, i64 pv);
extern void qsort (i64 * tab, usize lo, usize hi);

void setUp(void) {
}

void tearDown(void) {
}

void test_swap(void) {
    i64 tab [] = { 0ull<<20, 1ull<<20, 5ull<<20, 3ull<<20, 4ull<<20, 2ull<<20, 6ull<<20, 7ull<<20 };
    swap(tab, 2, 5);

    i64 sol [] = { 0ull<<20, 1ull<<20, 2ull<<20, 3ull<<20, 4ull<<20, 5ull<<20, 6ull<<20, 7ull<<20 };
    // for (int i = 0; i < 8; i++) { printf("%ld ", tab[i]); } printf("\n");
    // for (int i = 0; i < 8; i++) { printf("%ld ", sol[i]); } printf("\n");
    TEST_ASSERT_EQUAL_INT64_ARRAY(sol, tab, 8);
}

void test_choose_pivot(void) {
    i64 tab [] = { 0<<20, 1<<20, 5<<20, 3<<20, 4<<20, 2<<20, 6<<20, 7<<20, 8<<20, 9<<20 };
    i64 pv = choose_pivot(tab, 3, 6);
    TEST_ASSERT_EQUAL_INT64(pv, 3<<20);
}

void test_partition_1(void) {
    i64 tab [] = { 0xA, 0xC, 0xD, 0x8, 0xF, 0x6, 0xB, 0xA, 0xE, 0xE, 0x6, 0xD, 0x7, 0xB };
    usize hi = 14;
    usize sm = 0, eq = 0;

    sm = 0; eq = 0;
    partition(tab, 0, &sm, &eq, hi, 0xA);
    printf("[<] ("); for (usize i = 0; i < sm; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[=] ("); for (usize i = sm; i < eq; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[>] ("); for (usize i = eq; i < hi; i++) { printf("%ld ", tab[i]); } printf(")\n");
}

void test_partition_2(void) {
    i64 tab [] = { 0xA, 0xC, 0xD, 0x8, 0xF, 0x6, 0xB, 0xA, 0xE, 0xE, 0x6, 0xD, 0x7, 0xB };
    usize hi = 14;
    usize sm = 0, eq = 0;
    partition(tab, 0, &sm, &eq, hi, 0x5);
    printf("[<] ("); for (usize i = 0; i < sm; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[=] ("); for (usize i = sm; i < eq; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[>] ("); for (usize i = eq; i < hi; i++) { printf("%ld ", tab[i]); } printf(")\n");
}

void test_partition_3(void) {
    i64 tab [] = { 0xA, 0xC, 0xD, 0x8, 0xF, 0x6, 0xB, 0xA, 0xE, 0xE, 0x6, 0xD, 0x7, 0xB };
    usize hi = 14;
    usize sm = 0, eq = 0;
    partition(tab, 0, &sm, &eq, hi, 0x9);
    printf("[<] ("); for (usize i = 0; i < sm; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[=] ("); for (usize i = sm; i < eq; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[>] ("); for (usize i = eq; i < hi; i++) { printf("%ld ", tab[i]); } printf(")\n");
}

void test_partition_4(void) {
    i64 tab [] = { 0xA, 0xC, 0xD, 0x8, 0xF, 0x6, 0xB, 0xA, 0xE, 0xE, 0x6, 0xD, 0x7, 0xB };
    usize hi = 14;
    usize sm = 0, eq = 0;
    partition(tab, 0, &sm, &eq, hi, 0xF);
    printf("[<] ("); for (usize i = 0; i < sm; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[=] ("); for (usize i = sm; i < eq; i++) { printf("%ld ", tab[i]); } printf(")\n");
    printf("[>] ("); for (usize i = eq; i < hi; i++) { printf("%ld ", tab[i]); } printf(")\n");
}

void test_qsort(void) {
    i64 tab [] = { 101, 105, 104, 5, 3, 9, 1, 7, 2, 5, 6, 7, 1, 2, -207, -203, -206 };
    qsort(tab, 3, 14);
    for (usize i = 0; i < 17; i++) { printf("%ld ", tab[i]); } printf("\n");
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_swap);
    RUN_TEST(test_choose_pivot);
    RUN_TEST(test_partition_1);
    RUN_TEST(test_partition_2);
    RUN_TEST(test_partition_3);
    RUN_TEST(test_partition_4);
    RUN_TEST(test_qsort);
    return UNITY_END();
}
