#include <kos.h>

void println(const char* str, va_list vargs) {
    printf(str, vargs);
    fflush(stdout);
}