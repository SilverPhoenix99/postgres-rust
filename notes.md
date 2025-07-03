
# Notes
* Avoid returning `Option` from combinators where possible.
  * Instead, combinators should add `.optional()` to dependent combinators.
  * This way there's no guessing whether the combinator returns a required value or not.
  * There are exceptions, where `None` is represented by tokens, like `NULL` or `NONE`.
    * Examples include `string_or_null`, and `right_type`.
* Where a `Vec` can be empty, use `Option<Vec>`.
  * A lot of productions return non-empty Vecs, so Option makes it clearer when Vecs can be empty.
  * `Option<Vec>` has no overhead.
* Don't implement these. They're just Rust's `String`/`Vec`:
  * `common/stringinfo.c` / `include/lib/stringinfo.h`
  * `backend/utils/mb/stringinfo_mb.c` / `include/mb/stringinfo_mb.h`
* Check layout of structs and enums with `#![feature(rustc_attrs)]` + `#[rustc_layout(debug)]`.
  * Ref: https://doc.rust-lang.org/beta/unstable-book/language-features/rustc-attrs.html
* `indirection` and `opt_indirection` functions:
  * `check_func_name`: only allows `( '.' col_label )[1..]` == `( attrs )[1..]`
  * `check_indirection`: `*` is only allowed as the last item
  * `makeColumnRef`:
    * splits the name (`.`) from subscripts (`[]`)
    * `*` is only allowed as the last item of the whole source list, if there are no indexes.
    * `*` is allowed in any position after an index, and can show up multiple times.
    * examples:
      * `x` -> `(["x"], [])`
      * `x.y` -> `(["x", "y"], [])`
      * `x.y.*` -> `(["x", "y", All], [])`
      * `x.y[0].*.foo.bar.*` -> `(["x", "y"], [i(0), All, "foo", "bar", All])`
      * `*[foo]` -> `([All], [i("foo")])`
  * `makeRangeVarFromQualifiedName`: qualified name == `( attrs ){1, 3}`

# TO DO
* Allow case-insensitive search on Mphf, specifically to search for keywords.
  * Maybe use the `unicase` crate?
* Change `ParserResult.result` to be `Vec<ParseResult<RawStmt>>`.
  * Introduce a fail fast (compile time) flag.
  * In debug mode: fail fast == false, and returns all errors until EOF.
  * In release mode: fail fast == true, and returns the first error as the last element of the `Vec`.
  * Optional: Move `warnings` into `ParserResult`.
    * E.g.: `Vec<(ParseResult<RawStmt>, Vec<ParserWarningKind>)>`.
    * `struct { result: ParseResult<RawStmt>, warnings: Vec<ParserWarningKind> }`.
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
