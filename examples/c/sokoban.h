#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RedBlackTreeu64u64128 RedBlackTreeu64u64128;

void initialize(struct RedBlackTreeu64u64128 *slf);

struct RedBlackTreeu64u64128 *initialize_in(uint8_t *bytes, uintptr_t len);

uint32_t c_insert(struct RedBlackTreeu64u64128 *slf, uint64_t key, uint64_t value);

/**
 * Returns 0 if successful, u32::MAX if failure.
 *
 * If this fails, the given pointer will point to whatever was there before,
 * which is potentially uninitialized
 */
uint32_t c_get(struct RedBlackTreeu64u64128 *slf, const uint64_t *key, uint64_t *value);

/**
 * Returns 0 if successful, u32::MAX if failure.
 *
 * If this fails, the given pointer will point to whatever was there before,
 * which is potentially uninitialized
 */
uint32_t c_remove(struct RedBlackTreeu64u64128 *slf, const uint64_t *key, uint64_t *value);
