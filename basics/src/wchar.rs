#[inline(always)]
pub fn is_valid_unicode_codepoint(c: u32) -> bool {
    0 < c && c <= 0x10FFFF
}

pub fn decode_utf16(first: u16, second: u16) -> Option<char> {

    if !is_utf16_surrogate_first(first) || !is_utf16_surrogate_second(second) {
        return None
    }

    char::decode_utf16([first, second])
        .next()?
        .ok()
}

#[allow(clippy::manual_range_contains)]
#[inline(always)]
pub fn is_utf16_surrogate_first(c: u16) -> bool {
    0xD800 <= c && c <= 0xDBFF
}

#[allow(clippy::manual_range_contains)]
#[inline(always)]
pub fn is_utf16_surrogate_second(c: u16) -> bool {
    0xDC00 <= c && c <= 0xDFFF
}
