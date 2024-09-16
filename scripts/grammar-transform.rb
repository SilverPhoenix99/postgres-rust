class Grammar
  CODE_BLOCK = %r{
    (?<code_block>
      (?<!'){
        (?:
          (?> ("{"|"}"|[^{}])+ )
          | \g<code_block>
        )*
      }
    )
    |
    (?<comment>
      /\*
        (?:
          (?>
            (?:
                [^/*]
              | /(?!\*)
              | \*(?!/)
            )+
          )
          | \g<comment>
        )*
      \*/
    )
  }mx unless const_defined?(:CODE_BLOCK)

  PRECEDENCE = /%prec\s+\w+/ unless const_defined?(:PRECEDENCE)

  EMPTY_RULE = %r{([:|])\s*?([|;])} unless const_defined?(:EMPTY_RULE)

  TOKEN = %r{'.'|[^\w\s]+|\w+} unless const_defined?(:TOKEN)

  attr_reader :productions,
              :start_sym

  def self.tokenize(path)
    file = File.read(path);
    start = file.index(/^%%$/)
    if start
      stop  = file.rindex(/^%%$/)
      if stop != start
        start += 2
        stop -= 1
        file = file[start..stop];
      end
    end
    file.gsub!(CODE_BLOCK, '');
    file.gsub!(PRECEDENCE, '');
    # ensure the empty production is included
    file.gsub!(EMPTY_RULE, '\1 __empty \2');
    file.scan(TOKEN)
  end

  def self.load_bison(path)

    productions = tokenize(path)
      .chunk { |tok| tok == ';' }
      .filter_map { |delim, toks| !delim && toks }
      .each_with_object({}) do |(n, _, *syms), h|
        h[n.to_sym] = syms.chunk { |sym| sym == '|' }
          .filter_map do |delim, rule|
            next if delim
            rule.map { |sym| sym[0] == "'" ? sym[1..-2] : sym.to_sym }
          end
      end

    productions[:__empty] = [[:__empty]]

    productions

    new(productions)
  end

  def self.load_rb(path)
    file = File.read(path)
    productions = eval(file)
    new(productions)
  end

  def initialize(productions)
    @productions = productions
    @start_sym = productions.first.first
  end

  def save_bison(path)
    File.write path, to_bison
  end

  def save_rb(path)
    File.write path, to_rb
  end

  def terminals
    @terminals ||= @productions.values.flatten.
    each_with_object(Set.new) do |sym, s|
      s << sym unless @productions.key?(sym)
    end
  end

  def dup_productions
    @productions.each_with_object({}) do |(name, rules), h|
      h[name] = rules.map(&:dup)
    end
  end

  def remove_redundant_rules

    productions = @productions
    for i in 1..100 do
      redundant = productions.
        select do |name, rules|
          name != @start_sym &&
          name != :__empty &&
          rules.size == 1 &&
          rules.first.size == 1
        end.
        to_h do |name, rules|
          [name, rules.first.first]
        end
      break if redundant.empty?

      # ensure they don't reference each other,
      # otherwise they might get lost
      for j in 1..100 do
        break if redundant.select { |_, sym| redundant.key?(sym) }.
          each do |name, sym|
            redundant[name] = redundant[sym]
          end.
          empty?
      end

      productions = productions.each_with_object({}) do |(name, rules), h|
        next if redundant.key?(name)

        h[name] = rules.map do |rule|
          rule.map { |sym| redundant[sym] || sym }
        end
      end
    end

    Grammar.new(productions)
  end

  # collapse productions that only have 1 rule into their caller
  def inline_single_rule_productions

    productions = @productions;

    for i in 1..100 do

      single_prods = productions.select do |name, rules|
        name != :__empty &&
        rules.size == 1 &&
        !rules.first.include?(name) # recursive
      end;
      return Grammar.new(productions) if single_prods.empty?

      productions = productions.transform_values do |rules|
        rules.map do |rule|
          rule.flat_map do |sym|
            single_prods[sym]&.first || sym
          end
        end
      end;

      reacheable = productions.values.flatten.to_set;
      reacheable << @start_sym; # the start symbol is never reacheable per se

      reacheable = productions.select { |name, _| reacheable.include?(name) };

      return Grammar.new(productions) if reacheable.count == productions.count

      productions = reacheable;
    end

    Grammar.new(productions)
  end

  # collapse productions that only appear once on the right side into their caller
  def inline_singleton_productions

    productions = @productions

    for i in 1..100 do

      uniques = productions.
        flat_map do |name, rules|
          rules.flatten.select { |sym| productions.key?(sym) && sym != :__empty }
        end
        .tally
        .filter_map { |name, count| count == 1 && name } # only shows up once on the right hand side
        .to_set;

      if uniques.empty?
        i == 1 ? (return self) : break
      end

      productions = productions.
        each_with_object({}) do |(name, rules), prods|
          prods[name] = rules.
            map do |rule|
              h, *t = rule.each_with_index.
                select { |sym, _| uniques.include?(sym) }.
                map do |sym, idx|
                  productions[sym].map { |unique_rule| [idx, unique_rule] }
                end
              next [rule] unless h # h will be nil if no unique productions match
              idxs = h.product(*t).
                map do |unique_rules|
                  rule.dup.tap do |this|
                    unique_rules.each do |idx, unique_rule|
                      this[idx] = unique_rule
                    end
                    this.flatten!
                    # delete any __empty if it's not on its own
                    this.delete(:__empty) if this.size > 1
                  end
                end
            end
            .flatten(1)
        end;

        referenced = productions.
          flat_map do |name, rules|
            rules = rules.flatten.select { |sym| productions.key?(sym) }
            rules << name if name == @start_sym # start symbol is always referenced
            rules
          end.
          to_set;

        productions = productions.select { |name, _rules| referenced.include?(name) }
    end

    Grammar.new(productions)
  end

  def remove_direct_left_recursion
    direct_recursive = direct_left_recursive;
    return self unless direct_recursive

    productions = @productions.each_with_object({}) do |(name, rules), h|
      unless direct_recursive.productions.include?(name)
        h[name] = rules.map(&:dup)
        next
      end

      rec_rules, non_rec_rules = rules.partition { |rule| rule.first == name }
      rec_rules.map! { |rule| rule[1..] }

      h[name] = []
      num_names = 0

      rec_rule = if rec_rules.count > 1
        new_name = :"#{name}_#{(num_names += 1)}"
        h[new_name] = rec_rules
        [new_name]
      else
        rec_rules.first
      end

      non_rec_rule = if non_rec_rules.count > 1
        new_name = :"#{name}_#{(num_names += 1)}"
        h[new_name] = non_rec_rules
        [new_name]
      else
        non_rec_rules.first
      end

      if non_rec_rule == [:__empty] || rec_rule == non_rec_rule
        # a : a b | _  =>  a : b a | _
        # or
        # a : a b | b  =>  a : b a | b

        h[name].push(
          rec_rule << name,
          non_rec_rule
        )

      else
        # a : a b | c  =>  a  : c a' | c
        #                  a' : b a' | b

        new_name = :"#{name}_#{(num_names += 1)}"
        h[new_name] = [
          rec_rule.dup << new_name,
          rec_rule
        ]

        h[name].push(
          non_rec_rule.dup << new_name,
          non_rec_rule
        )
      end
    end;

    Grammar.new(productions)
  end

  # ! method is hacky
  # It can only fix indirect recursion when one of the rules is a single element
  # referencing the other indirect recursive one:
  #  a: [[:b], [...]]  <== [:b] is detected
  #  b: [[:a, ...], [...]]
  # It will replace indirect with direct left recursions.
  def remove_indirect_left_recursion!
    indirect_recursive = indirect_left_recursive;
    return self unless indirect_recursive

    indirect_productions = indirect_recursive.productions.each_with_object({}) do |(name, rules), h|

      rec_rules, non_rec_rules = rules.partition { |rule| indirect_recursive.productions.key?(rule.first) }
      rec_sym = rec_rules.map(&:first).uniq
      raise 'Cannot handle multiple indirect recursion from a single production' if rec_sym.count > 1
      rec_sym = rec_sym.first

      # only move rules out if it's not a production with a recursive singular rule
      if rec_rules.size == 1 && rec_rules.first.size == 1
        h[name] = { name => rules }
        next
      end

      h[name] = prods = { name => [] }
      num_names = 0

      rec_rule = if rec_rules.count > 1
        new_name = :"#{name}_#{(num_names += 1)}"
        prods[new_name] = rec_rules.map { |rule| rule[1..] }
        [rec_sym, new_name]
      else
        rec_rules.first
      end

      non_rec_rule = if non_rec_rules.count > 1
        new_name = :"#{name}_#{(num_names += 1)}"
        prods[new_name] = non_rec_rules.map(&:dup)
        [new_name]
      else
        non_rec_rules.first
      end

      prods[name].push(non_rec_rule, rec_rule)
    end;

    productions = @productions.each_with_object({}) do |(name, rules), h|
      rec_prods = indirect_productions[name]
      unless rec_prods
        h[name] = rules
        next
      end

      rec_prods.each do |n, rs|
        h[n] = rs
      end

      rec_rules = rec_prods[name]
      singular = rec_rules
        .find do |rule|
          rule.size == 1 &&
          indirect_productions.key?(rule.first)
        end
        &.first

      # nothing else to do
      next unless singular

      # inline recursive singular rules
      rec_rules.delete([singular])
      rec_rules.push(*indirect_productions[singular][singular])
    end;

    Grammar.new(productions)
  end

  def direct_left_recursive
    recursive = first_non_terms_graph.select { |name, syms| syms.include?(name) }
    return nil if recursive.empty?

    direct = @productions.select { |name, _| recursive.key?(name) }
    Grammar.new(recursive)
  end

  def indirect_left_recursive
    recursive = first_non_terms_graph.reject { |name, syms| syms.include?(name) }

    for i in 1..100 do
      first_syms = recursive.values.reduce(:|)
      g = recursive.reject { |name, _| !first_syms.include?(name) }
      g = g.transform_values { |syms| syms.select { |sym| g.key?(sym) }.to_set }
        .reject { |_, syms| syms.empty? }
      break if g == recursive
      recursive = g
    end

    return nil if recursive.empty?
    recursive = @productions.select { |name, _| recursive.key?(name) }
    Grammar.new(recursive)
  end

  def remove_opt_rules
    productions = @productions
    for i in 1..100 do

      nullable = productions.select do |name, rules|
        name != @start_sym && name != :__empty && rules.include?([:__empty])
      end;

      break if nullable.empty?

      productions = productions.to_h do |name, rules|
        if nullable.key?(name)
          [name, rules.dup.tap { |this| this.delete([:__empty]) }]
        else
          new_rules = []
          rules.each do |rule|
            rs = [rule]
            rule.select { |sym| nullable.key?(sym) }.each do |null_sym|
              rs = rs.map do |r|
                [
                  r,
                  r.dup.tap do |this|
                    this.delete(null_sym)
                    this << :__empty if this.empty?
                  end
                ]
              end.flatten(1)
            end
            new_rules.push(*rs)
          end
          [name, new_rules]
        end
      end;
    end
    Grammar.new(productions)
  end

  def first_non_terms_graph
    non_terms = @productions.keys.to_set.tap { |this| this.delete(:__empty) }
    @first_non_terms_graph ||= @productions
      .to_h do |name, rules|
        rules = rules.map(&:first).to_set & non_terms
        [name, rules]
      end
      .reject { |_, rules| rules.empty? }
  end

  def to_s(format: nil)
    if format == :bison
      to_bison
    else
      to_rb
    end
  end

  def to_rb
    return '{}' if @productions.empty?

    prods = @productions
      .map do |name, rules|

        rs = rules
          .map do |rule|
            r = rule.map(&:inspect).join(', ')
            "    [#{r}]"
          end
          .join(",\n")

        "  #{name}: [\n#{rs}\n  ]"
      end
      .join(",\n\n")

    "{\n#{prods}\n}"
  end

  def to_bison
    return '' if @productions.empty?

    @productions
      .map do |name, rules|

        rs = rules
          .map do |rule|
            rule.map { |sym| sym.is_a?(String) ? "'#{sym}'" : sym }.join(' ')
          end
          .join("\n  | ")

        "#{name} :\n    #{rs}\n;"
      end
      .join("\n\n")
  end
end


__END__

# assumes running in this directory

load './grammar-transform.rb';

grammar = Grammar.load_bison('../../postgres/src/backend/parser/gram.y');

grammar2 = grammar.remove_redundant_rules.
  remove_indirect_left_recursion!.
  remove_direct_left_recursion.
  inline_singleton_productions;
grammar2.save_bison './tmp/grammar2.bison.rb'

grammar = Grammar.load_bison('./tmp/grammar2.bison.rb');
