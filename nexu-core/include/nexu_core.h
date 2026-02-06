#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * C语言调用的SHA256哈希函数，返回C字符串指针（需手动调用nexu_free释放）
 */
char *nexu_hash(const char *input, unsigned int len);

/**
 * C语言调用的内存释放函数，释放nexu_hash分配的C字符串指针
 */
void nexu_free(char *ptr);
