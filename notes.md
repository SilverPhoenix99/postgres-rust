
# Notes
* Don't implement these. They're just Rust's `String`/`Vec`:
  * `common/stringinfo.c` / `include/lib/stringinfo.h` 
  * `backend/utils/mb/stringinfo_mb.c` / `include/mb/stringinfo_mb.h`
* Check layout of structs and enums with `#![feature(rustc_attrs)]` + `#[rustc_layout(debug)]`.
  * Ref: https://doc.rust-lang.org/beta/unstable-book/language-features/rustc-attrs.html

# TO DO
* Wrap `ParserWarningKind` in a `LocatedErrorReport`.
* Merge `Parser.move_stmt` into `Parser.fetch_stmt`
* Test `NumericSpec`.
* Support encodings besides UTF-8: `include/mb/pg_wchar.h`, `common/wchar.c`, etc
  * For now:
    * Assume only UTF-8;
    * If any code uses encodings, throw an error (user input), or just implicitly use UTF-8 (internally);
    * If needed, make no-op code.
* Support i18n.
  * For now, all output will be EN.
* `Guc` 1st implementation will be read-only.
  * Options to change the settings will return an error or be ignored.
  * Every object will have access to the global instance, but that will change in the future.
* For now logging is reported via `Result`.
  * Later, this will be used to actually log out at certain end/finish/completion points.

# Build C Postgres

```sh
pacman -S flex bison icu-devel

mkdir pg_build
cd pg_build

time ../postgres/configure ICU_CFLAGS='-I/usr/include' ICU_LIBS='-L/usr/lib -licui18n -licuuc -licudata' --host=x86_64-w64-mingw32 --enable-debug --enable-cassert

#time make -j $(nproc)

for f in $(find -name Makefile -exec grep -lP '^generated-headers:' {} \+ | xargs dirname) ; do
  echo $f
  (
    cd $f
    make generated-headers
  )
done

( cd ./src/backend/parser/ ; make scan.c )

```
