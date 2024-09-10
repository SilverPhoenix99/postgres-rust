
# Notes
* Don't implement these. They're just Rust's `String`/`Vec`:
  * `common/stringinfo.c` / `include/lib/stringinfo.h` 
  * `backend/utils/mb/stringinfo_mb.c` / `include/mb/stringinfo_mb.h`

# TO DO
* Support encodings besides UTF-8: `include/mb/pg_wchar.h`, `common/wchar.c`
  * For now:
    * Assume only UTF-8;
    * If any code uses encodings, throw an error (user input), or just implicitly use UTF-8 (internally);
    * If needed, make no-op code.
