const WHITESPACE_FLAG:    u16 = 0x00_01;
const NEW_LINE_FLAG:      u16 = 0x00_02;
const OPERATOR_FLAG:      u16 = 0x00_04;
const PG_OPERATOR_FLAG:   u16 = 0x00_08;
const DECIMAL_DIGIT_FLAG: u16 = 0x00_10;
const HEX_DIGIT_FLAG:     u16 = 0x00_20;
const OCTAL_DIGIT_FLAG:   u16 = 0x00_40;
const ALPHABETIC_FLAG:    u16 = 0x00_80;
const IDENT_START_FLAG:   u16 = 0x01_00;

const LOWERCASE_OFFSET: usize = (b'a' - b'A') as usize;

const TABLE: [u16; 256] = const {
    let mut table = [0; 256];

    table[b'\t'   as usize] = WHITESPACE_FLAG;
    table[b'\n'   as usize] = WHITESPACE_FLAG | NEW_LINE_FLAG;
    table[b'\x0B' as usize] = WHITESPACE_FLAG;
    table[b'\x0C' as usize] = WHITESPACE_FLAG;
    table[b'\r'   as usize] = WHITESPACE_FLAG | NEW_LINE_FLAG;
    table[b' '    as usize] = WHITESPACE_FLAG;
    table[b'!'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'#'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'%'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'&'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'*'    as usize] = OPERATOR_FLAG;
    table[b'+'    as usize] = OPERATOR_FLAG;
    table[b'-'    as usize] = OPERATOR_FLAG;
    table[b'/'    as usize] = OPERATOR_FLAG;
    table[b'<'    as usize] = OPERATOR_FLAG;
    table[b'='    as usize] = OPERATOR_FLAG;
    table[b'>'    as usize] = OPERATOR_FLAG;
    table[b'?'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'@'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'^'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'_'    as usize] = IDENT_START_FLAG;
    table[b'`'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'|'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;
    table[b'~'    as usize] = OPERATOR_FLAG | PG_OPERATOR_FLAG;

    let mut c = b'0' as usize;
    while c <= b'7' as usize {
        table[c] = OCTAL_DIGIT_FLAG | DECIMAL_DIGIT_FLAG | HEX_DIGIT_FLAG;
        c += 1;
    }

    table[b'8' as usize] = DECIMAL_DIGIT_FLAG | HEX_DIGIT_FLAG;
    table[b'9' as usize] = DECIMAL_DIGIT_FLAG | HEX_DIGIT_FLAG;

    let flags = ALPHABETIC_FLAG | HEX_DIGIT_FLAG | IDENT_START_FLAG;
    let mut c = b'A' as usize;
    while c <= b'F' as usize {
        table[c] = flags;
        table[c + LOWERCASE_OFFSET] = flags;
        c += 1;
    }

    let flags: u16 = ALPHABETIC_FLAG | IDENT_START_FLAG;
    let mut c = b'G' as usize;
    while c <= b'Z' as usize {
        table[c] = flags;
        table[c + LOWERCASE_OFFSET] = flags;
        c += 1;
    }

    let mut c = b'\x80' as usize;
    while c <= b'\xFF' as usize {
        table[c] = IDENT_START_FLAG;
        c += 1;
    }

    table
};

#[inline(always)]
pub fn is_whitespace(c: u8) -> bool {
    TABLE[c as usize] & WHITESPACE_FLAG != 0
}

#[inline(always)]
pub fn is_new_line(c: u8) -> bool {
    TABLE[c as usize] & NEW_LINE_FLAG != 0
}

#[inline(always)]
pub fn is_op(c: u8) -> bool {
    TABLE[c as usize] & OPERATOR_FLAG != 0
}

#[inline(always)]
pub fn is_pg_op(c: u8) -> bool {
    TABLE[c as usize] & PG_OPERATOR_FLAG != 0
}

#[inline(always)]
pub fn is_sql_standard_op(c: u8) -> bool {
    // the comparison means: it's an op, but not a PG op
    TABLE[c as usize] & (OPERATOR_FLAG | PG_OPERATOR_FLAG) == OPERATOR_FLAG
}

#[inline(always)]
pub fn is_decimal_digit(c: u8) -> bool {
    TABLE[c as usize] & DECIMAL_DIGIT_FLAG != 0
}

#[inline(always)]
pub fn is_hex_digit(c: u8) -> bool {
    TABLE[c as usize] & HEX_DIGIT_FLAG != 0
}

#[inline(always)]
pub fn is_oct_digit(c: u8) -> bool {
    TABLE[c as usize] & OCTAL_DIGIT_FLAG != 0
}

#[inline(always)]
pub fn is_bin_digit(c: u8) -> bool {
    // b'0' = 0x30 == 0b0011_0000
    // b'1' = 0x31 == 0b0011_0001
    c & b'\xFE' == b'0'
}

#[inline(always)]
pub fn is_ident_start(c: u8) -> bool {
    TABLE[c as usize] & IDENT_START_FLAG != 0
}

#[inline(always)]
pub fn is_ident_cont(c: u8) -> bool {
    TABLE[c as usize] & (IDENT_START_FLAG | DECIMAL_DIGIT_FLAG) != 0 || c == b'$'
}

#[inline(always)]
pub fn is_dollar_quote_cont(c: u8) -> bool {
    TABLE[c as usize] & (IDENT_START_FLAG | DECIMAL_DIGIT_FLAG) != 0
}

#[inline(always)]
pub fn is_extended_ascii(c: u8) -> bool {
    c >= b'\x80'
}
