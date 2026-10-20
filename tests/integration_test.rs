
// 17 TOTAL TESTS

// 3 CATEGORIES:
// -Correctness -- Normal inputs produce the right output (e.g. "hello" → "olleh") (4)
// -EDGE CASES --Empty strings, single characters, no vowels, non-ASCII input, embedded null bytes (10)
// -STABILITY --Large input (100,000 chars) doesn't crash; 1,000 repeated alloc/free cycles don't leak or crash (3)

use proxie_ffi::{count_vowels, str_length, str_reverse, to_uppercase};

//CORRECTNESS -- Normal inputs produce the right output (e.g. "hello" → "olleh")

#[test]
fn test_str_length_basic() {
    assert_eq!(str_length("hello").unwrap(), 5);
    assert_eq!(str_length("Proxie Studio").unwrap(), 13);
}

#[test]
fn test_str_reverse_basic() {
    assert_eq!(str_reverse("hello").unwrap(), "olleh");
    assert_eq!(str_reverse("Rust").unwrap(), "tsuR");
}

#[test]
fn test_count_vowels_basic() {
    assert_eq!(count_vowels("hello world").unwrap(), 3); // e, o, o
    assert_eq!(count_vowels("PROXIE").unwrap(), 3); // O, I, E
}

#[test]
fn test_to_uppercase_basic() {
    assert_eq!(to_uppercase("hello").unwrap(), "HELLO");
    assert_eq!(to_uppercase("Rust FFI").unwrap(), "RUST FFI");
}

//Edge cases -- Empty strings, single characters, no vowels, non-ASCII input, embedded null bytes

#[test]
fn test_empty_string() {
    assert_eq!(str_length("").unwrap(), 0);
    assert_eq!(str_reverse("").unwrap(), "");
    assert_eq!(count_vowels("").unwrap(), 0);
    assert_eq!(to_uppercase("").unwrap(), "");
}

#[test]
fn test_single_character() {
    assert_eq!(str_length("a").unwrap(), 1);
    assert_eq!(str_reverse("a").unwrap(), "a");
    assert_eq!(count_vowels("a").unwrap(), 1);
    assert_eq!(count_vowels("b").unwrap(), 0);
    assert_eq!(to_uppercase("a").unwrap(), "A");
}

#[test]
fn test_no_vowels() {
    assert_eq!(count_vowels("xyz rhythm").unwrap(), 0);
}

#[test]
fn test_already_uppercase_is_idempotent() {
    assert_eq!(to_uppercase("ALREADY").unwrap(), "ALREADY");
}

#[test]
fn test_special_characters_ascii() {
    // Punctuation and digits should pass through str_reverse/to_uppercase
    // unchanged in *content*, only order changes for reverse.
    assert_eq!(str_reverse("a1! b2@").unwrap(), "@2b !1a");
    assert_eq!(to_uppercase("v1.0-beta!").unwrap(), "V1.0-BETA!");
    assert_eq!(str_length("a1! b2@").unwrap(), 7);
}

#[test]
fn test_unicode_to_uppercase_preserves_non_ascii_bytes() {
    // to_uppercase only touches ASCII bytes, so multi-byte UTF-8
    // characters must survive untouched and the result must stay valid UTF-8.
    let input = "café ☕ naïve";
    let result = to_uppercase(input).unwrap();
    assert_eq!(result, "CAFé ☕ NAïVE");
}

#[test]
fn test_unicode_str_length_is_byte_length_not_char_count() {
    // "café" is 4 chars but 5 bytes (é is 2 bytes in UTF-8).
    assert_eq!(str_length("café").unwrap(), 5);
}

#[test]
fn test_unicode_count_vowels_ignores_accented_letters() {
    // 'é' is not an ASCII vowel, so it should not be counted.
    assert_eq!(count_vowels("café").unwrap(), 1); // just the 'a'
}

#[test]
fn test_unicode_str_reverse_returns_error_instead_of_corrupt_string() {
    // Reversing raw bytes of a string with multi-byte UTF-8 characters
    // breaks those byte sequences apart. Our wrapper must detect this
    // and return an error rather than an invalid `String`.
    let result = str_reverse("café");
    assert!(
        result.is_err(),
        "expected an error for byte-reversal of multi-byte UTF-8 input, got {:?}",
        result
    );
}

#[test]
fn test_interior_nul_byte_is_rejected() {
    let with_nul = "hello\0world";
    assert!(str_length(with_nul).is_err());
    assert!(str_reverse(with_nul).is_err());
    assert!(count_vowels(with_nul).is_err());
    assert!(to_uppercase(with_nul).is_err());
}

//Stability --Large input (100,000 chars) doesn't crash; 1,000 repeated alloc/free cycles don't leak or crash


#[test]
fn test_large_input_no_panic_or_crash() {
    let large = "a".repeat(100_000);
    let len = str_length(&large).unwrap();
    assert_eq!(len, 100_000);

    let reversed = str_reverse(&large).unwrap();
    assert_eq!(reversed.len(), 100_000);
    assert!(reversed.chars().all(|c| c == 'a'));

    let vowels = count_vowels(&large).unwrap();
    assert_eq!(vowels, 100_000);
}

#[test]
fn test_repeated_calls_do_not_leak_or_crash() {
    // Runs many allocate/free cycles through the FFI boundary. If ownership
    // were mishandled (double free, leak causing OOM, use-after-free), this
    // loop is the kind of stress that would surface it under a tool like
    // valgrind/ASan. See README for how to run this under valgrind.
    for i in 0..1000 {
        let s = format!("iteration number {i}");
        let _ = str_reverse(&s).unwrap();
        let _ = to_uppercase(&s).unwrap();
        let _ = count_vowels(&s).unwrap();
        let _ = str_length(&s).unwrap();
    }
}

#[test]
fn test_whitespace_only_string() {
    assert_eq!(str_reverse("   ").unwrap(), "   ");
    assert_eq!(count_vowels("   ").unwrap(), 0);
    assert_eq!(to_uppercase("   ").unwrap(), "   ");
}
