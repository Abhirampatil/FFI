// Rust can't call C++ function
// atcs as interface that both sides understand.

// Rust needs to know 
// What function exists, arguments they take, what is returned,  
// What ABI does it use?

// So we expose a C-compatible interface.

#ifndef PROXIE_WRAPPER_H
#define PROXIE_WRAPPER_H
#include <stddef.h>
#ifdef __cplusplus

// C++ has name mangling ( function name, parameter types,namespaces).
// extern "C" Expose these functions using the C ABI/linking conventions.  
// extern "C" gives us a stable C-compatible ABI for the functions we're exposing to Rust.
extern "C" { 
#endif


/* Returns the length of the string in bytes (like strlen). */
size_t str_length(const char* input);

/* Returns a newly allocated, byte-reversed copy of the string.
 * Caller must free the result with proxie_free_string(). */
char* str_reverse(const char* input);

/* Counts ASCII vowels in the string. */
int count_vowels(const char* input);

/* Returns a newly allocated copy of the string with ASCII letters
 * upper-cased. Bytes outside the ASCII range are left untouched.
 * Caller must free the result with proxie_free_string(). */
char* to_uppercase(const char* input);

/* Frees a buffer previously returned by str_reverse() or to_uppercase(). */
void proxie_free_string(char* ptr);

#ifdef __cplusplus
}
#endif
#endif /* PROXIE_WRAPPER_H */
