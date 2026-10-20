#include "wrapper.h"
#include <cstdlib>
#include <cstring>
#include <cctype>

size_t str_length(const char* input) {
    if (input == nullptr) return 0;
    return std::strlen(input);
}

char* str_reverse(const char* input) {
    if (input == nullptr) return nullptr;

    size_t len = std::strlen(input);
    char* result = static_cast<char*>(std::malloc(len + 1));
    if (result == nullptr) return nullptr; // allocation failure

    for (size_t i = 0; i < len; ++i) {
        result[i] = input[len - 1 - i];
    }
    result[len] = '\0';
    return result;
}

int count_vowels(const char* input) {
    if (input == nullptr) return 0;

    int count = 0;
    for (const char* p = input; *p != '\0'; ++p) {
        char c = static_cast<char>(std::tolower(static_cast<unsigned char>(*p)));
        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            ++count;
        }
    }
    return count;
}

char* to_uppercase(const char* input) {
    if (input == nullptr) return nullptr;

    size_t len = std::strlen(input);
    char* result = static_cast<char*>(std::malloc(len + 1));
    if (result == nullptr) return nullptr;

    for (size_t i = 0; i < len; ++i) {
        unsigned char c = static_cast<unsigned char>(input[i]);
        // Only touch ASCII range so multi-byte UTF-8 sequences (which use
        // bytes >= 0x80) are left byte-for-byte intact.
        if (c < 128) {
            result[i] = static_cast<char>(std::toupper(c));
        } else {
            result[i] = input[i];
        }
    }
    result[len] = '\0';
    return result;
}

void proxie_free_string(char* ptr) {
    std::free(ptr);
}
