
# Notes
* Don't implement these. They're just Rust's `String`/`Vec`:
  * `common/stringinfo.c` / `include/lib/stringinfo.h` 
  * `backend/utils/mb/stringinfo_mb.c` / `include/mb/stringinfo_mb.h`

# TO DO
* Refactor `AstNode` into finer grain types.
  * `Parser.parse()` shouldn't return a generic node.
  * E.g.: `AstStmt`
* Refactor to `TokenKind::Keyword(Keyword)`.
  * Add method `Keyword::details(&self)`.
  * Change to `Map<Keyword, &;static KeywordDetails>` (instead of `&'static str` key)
* Merge `Parser.move_stmt` into `Parser.fetch_stmt`
* Replace some `unwrap`s with `expect`s.
* Add notes to `unreachable!`
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
