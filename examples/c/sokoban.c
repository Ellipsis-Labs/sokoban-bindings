#include "sokoban.h"
#include <stdio.h>
#include <stdint.h>

#define UNWRAP_GET(function_call)                   \
    do                                              \
    {                                               \
        if ((function_call) != 0)                   \
        {                                           \
            printf("FAILED: %s\n", #function_call); \
            return -1;                              \
        }                                           \
    } while (0)

#define UNWRAP_INSERT(function_call)                 \
    do                                               \
    {                                                \
        uint32_t result = function_call;             \
        if ((result == 0) || (result == UINT32_MAX)) \
        {                                            \
            printf("FAILED: %s\n", #function_call);  \
        }                                            \
    } while (0)

int main()
{
    // Mock solana account. This will have u64 alignment
    u_int64_t *account_bytes = malloc(4128);

    RedBlackTreeu64u64128 *tree = (RedBlackTreeu64u64128 *)(account_bytes);
    initialize(tree);
    printf("init\n");

    UNWRAP_INSERT(c_insert(tree, (uint64_t)(5), (uint64_t)(3)));
    printf("inserted 5 and 3\n");

    UNWRAP_INSERT(c_insert(tree, (uint64_t)(6), (uint64_t)(4)));
    printf("insert 6 and 4\n");

    uint64_t key = 6;
    uint64_t value = 0;
    UNWRAP_GET(c_get(tree, &key, &value));
    printf("get %llu -> %llu\n", key, value);
}
