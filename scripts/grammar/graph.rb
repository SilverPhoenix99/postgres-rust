
require_relative '../grammar-transform'

class GrammarTransform::Grammar
  def to_graph(exclude: Graph::EXCLUDE)
    Graph.from_grammar(self, exclude:)
  end
end

module Graph

  EXCLUDE = %i[
    generic_reset generic_set var_list var_value
    opt_transaction
    opt_transaction_chain
    transaction_mode_list transaction_mode_list_1 transaction_mode_list_2 transaction_mode_item iso_level transaction_mode_list_or_empty
  ].to_set.freeze

  def self.run!(output: nil, exclude: EXCLUDE)
    grammar_input = Pathname(__dir__) / 'grammar.bison'
    grammar = GrammarTransform::Grammar.load_bison(grammar_input)
    graph = grammar.to_graph
    graphviz = graph.to_dot(subgraphs: true)

    if output
      File.write output, graphviz
    end

    graph
  end

  def self.from_grammar(grammar, exclude: Graph::EXCLUDE)

    graph = grammar.productions.each_with_object({}) do |(p, rules), g|
      g[p] = rules.flatten.to_set.delete(:__empty)
    end;

    non_terms = graph.keys.to_set;
    sources = graph[:stmt];

    graph.transform_values! { it & non_terms };

    # remove direct recursion
    recursive, graph = graph.partition { |p, cs| p.end_with?('_1') && cs.include?(p) }.map(&:to_h);

    recursive.each { |p, cs| cs.delete(p) }.transform_values!(&:to_a);

    graph.each do |p, cs|
      rs, non_rs = cs.partition { recursive.key?(it) }

      if rs.any?
        rs.map! { recursive[it] }
        rs.flatten!
        non_rs.push(*rs)
      end

      cs.replace(non_rs)
      cs.delete(p)
    end

    exclude = exclude&.to_set || Set.new

    # remove top level and PLSql sources
    exclude += %i[
      parse_toplevel
      stmtmulti
      stmtmulti_1
      toplevel_stmt
      PLpgSQL_Expr
      PLAssignStmt
      opt_distinct_clause
      plassign_target
      plassign_equals
    ]

    # exclude single terminal productions - these are inlined
    exclude += %i[
      from_in
      opt_all_clause
      opt_as
      opt_asymmetric
      opt_by
      opt_column
      opt_column_list
      opt_concurrently
      opt_default
      opt_equal
      opt_freeze
      opt_from_in
      opt_full
      opt_name_list
      opt_no
      opt_nowait
      opt_procedural
      opt_program
      opt_table
      opt_trusted
      opt_unique
      opt_using
      opt_varying
      opt_verbose
      opt_with
      path_opt
    ]

    # remove common (base package) sinks
    exclude += %i[
      bare_label_keyword
      col_name_keyword
      reserved_keyword
      type_func_name_keyword
      unreserved_keyword

      attrs
      BareColLabel
      type_function_name
      NonReservedWord
      opt_drop_behavior
      NonReservedWord_or_Sconst
      I_or_F_const

      ColId
      name
      opt_single_name
      columnList
      name_list
      var_name

      ColLabel
      attr_name

      any_name
      handler_name
      opt_qualified_name
      any_name_list

      qualified_name
      qualified_name_list

      relation_expr
      relation_expr_list

      role_list
      RoleSpec
      RoleId

      SignedIconst
      NumericOnly

      all_Op
      any_operator
      MathOp
      qual_all_Op
      qual_Op
      subquery_Op

      opt_analyze
      analyze_keyword

      opt_boolean_or_string
      copy_generic_opt_arg_list

      object_type_any_name
      object_type_name
      drop_type_name
      object_type_name_on_any_name
    ]

    graph.reject! { |p, _| exclude.include?(p) };
    graph.transform_values! { it - exclude };

    # delete the production, but it'll still show as a sink
    graph.delete(:stmt)

    # remove empty productions: they're pseudo-terminals now
    # graph.reject! { |_, cs| cs.empty? };

    graph
  end

  def to_dot(subgraphs: {})

    subgraphs ||= {}
    subgraphs = self.subgraphs if subgraphs == true

    source_roots = subgraphs.transpose.transform_values(&:first)

    edges = self
      .map do |parent, children|
        children.map { [parent, it] }
      end
      .flatten(1)
      .reject do |parent, child|
        next true if parent == :a_expr_3 && child == :a_expr_1
        pr = subgraphs.key?(parent) ? parent : source_roots[parent]
        cr = subgraphs.key?(child) ? child : source_roots[child]
        pr && pr == cr
      end
      .sort
      .map do |parent, child|
        if child == :stmt
          child = '{stmt [color=red penwidth=3]}'
        end
        "#{parent} -> #{child}"
      end

    subs = subgraphs.flat_map do |root, members|
      ms = members + [root]
      sub_edges = self.select { |parent, _| ms.include?(parent) }
        .transform_values { it & ms }
        .reject { |_, cs| cs.empty? }
        .sort_by { |parent, _| parent }
        .flat_map do |parent, children|
          children
            .map do |child|
              if child == :stmt
                child = '{stmt [color=red penwidth=3]}'
              end
              "#{parent} -> #{child}"
            end
        end

      [
        "subgraph cluster_#{root} {",
        *sub_edges.map { |it| "  #{it}" },
        '}',
        '',
      ]
    end

    sinks = self.sinks
      .reject { |s| source_roots.key?(s) }
      .to_set
      .sort

    lines = [
      'digraph Grammar {',
      '',
      'rankdir=LR',
      'concentrate=true',
      # 'splines=polyline',
      'pencolor=darkorchid4',
      'penwidth=3',
      'ranksep=3',
      'nodesep=1',
      ('compound=true' if subgraphs.any?),
      '',
      'bgcolor="#181818"',
      '',
      'node [',
      '  fontcolor = "#e6e6e6"',
      '  style = filled',
      '  color = "#e6e6e6"',
      '  fillcolor = "#333333"',
      ']',
      '',
      'edge [',
      '  color = "#e6e6e6"',
      '  fontcolor = "#e6e6e6"',
      ']',
      '',
      *edges,
      '',
      '// subgraphs',
      'edge [style=dotted]',
      '',
      *subs,
      '',
      *sinks.map { "#{it} [color=blue penwidth=3]" },
      '',
      '}'
    ].compact

    lines.map(&:rstrip).join("\n")

  end

  def transpose
    self.each_with_object({}) do |(p, cs), g|
      cs.each do
        (g[it] ||= Set.new) << p
      end
    end
  end

  def subgraphs
    @subgraphs ||= begin
      sources = self.transpose
      nodes = self.nodes

      source_roots = sources
        .each do |src, cs|
          visited = Set[src]
          while cs&.size == 1 && !visited.include?(cs.first)
            visited << cs.first
            sources[src] = cs
            cs = sources[cs.first]
          end
        end
        .reject { |_, cs| cs.size != 1 }

      subgraphs = {}
      for i in 1..100 do
        new_subgraphs = nodes.to_h { |n| [n, sources[n]] }
          .reject { |_, ps| ps.nil? || ps.empty? }
          .transform_values do |ps|
            ps.map { source_roots[it] || Set[it] }
              .reduce(Set.new, &:+)
          end
          .select { |_, ps| ps.size == 1 }
          .transpose

        break if subgraphs == new_subgraphs
        subgraphs = new_subgraphs
        source_roots = subgraphs.transpose
      end

      subgraphs.transform_values!(&:freeze).freeze
    end
  end

  def sinks
    @sinks ||= begin
      values.reduce(&:+)
        .reject { keys.include?(it) }
        .to_set
    end
  end

  def edges
    self
      .map do |parent, children|
        children.map { [parent, it] }
      end
      .flatten(1)
      .to_set
  end

  def nodes = @nodes ||= self.keys.to_set + self.values.reduce(&:+).freeze

  def clone = self.transform_values(&:dup)
end

Hash.include Graph

if __FILE__ == $PROGRAM_NAME
  output = $ARGV[0]
  graph = Graph.run!(output:)
  pp graph unless output
end

__END__

# irb -rclipboard

load './graph.rb'
grammar = GrammarTransform::Grammar.load_bison('./grammar.bison')
graph = grammar.to_graph

File.write 'grammar-graph.dot', graph.to_dot(subgraphs: true)

Clipboard.copy graph.to_dot(subgraphs: true)
