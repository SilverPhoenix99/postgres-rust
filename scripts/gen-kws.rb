# gen kws
x = File.readlines('../../postgres/src/include/parser/kwlist.h');

categories = {
  'UNRESERVED_KEYWORD' => 'Unreserved',
  'COL_NAME_KEYWORD' => 'ColumnName',
  'TYPE_FUNC_NAME_KEYWORD' => 'TypeFuncName',
  'RESERVED_KEYWORD' => 'Reserved',
}

y = x.map {
  _1.match(/^PG_KEYWORD\(\s*"(\w+)"\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*.*/)
}.
compact.map {
  kw = _1[2].sub(/_P$/, '').split('_').map(&:capitalize).join
  {
    text: _1[1],
    keyword: kw,
    category: 'KeywordCategory::' + categories[_1[3]],
    bare: _1[4] == 'BARE_LABEL',
  }
};

Clipboard.copy y.map { "#{_1[:keyword]}," }.join("\n")

Clipboard.copy(
  y.map do
    text = _1[:text]
    kw = _1[:keyword]
    category = _1[:category]
    bare = _1[:bare]
    "    b\"#{text}\" => keyword(#{kw}, b\"#{text}\", #{category}, #{bare}),"
  end.
  join("\n")
)

