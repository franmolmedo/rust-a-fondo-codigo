#include <stddef.h>
#include <stdint.h>

/* The caller provides length initialized bytes. The pointer is not retained.
   A zero length performs no reads. Unsigned addition is modulo 2^32. */
uint32_t appendix_byte_sum(const uint8_t *bytes, size_t length) {
    uint32_t sum = 0;
    for (size_t index = 0; index < length; ++index) {
        sum += bytes[index];
    }
    return sum;
}
