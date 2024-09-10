#[inline(always)]
pub fn is_valid_unicode_codepoint(c: u32) -> bool {
    0 < c && c <= 0x10FFFF
}

pub fn decode_utf16(first: u32, second: u32) -> Option<char> {

    if !is_utf16_surrogate_first(first) || !is_utf16_surrogate_second(second) {
        return None
    }

    let first = first as u16;
    let second = second as u16;

    char::decode_utf16([first, second])
        .next()?
        .ok()
}

#[allow(clippy::manual_range_contains)]
#[inline(always)]
pub fn is_utf16_surrogate_first(c: u32) -> bool {
    0xD800 <= c && c <= 0xDBFF
}

#[allow(clippy::manual_range_contains)]
#[inline(always)]
pub fn is_utf16_surrogate_second(c: u32) -> bool {
    0xDC00 <= c && c <= 0xDFFF
}
