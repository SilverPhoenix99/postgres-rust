parse_toplevel :
    stmtmulti
  | MODE_TYPE_NAME Typename
  | MODE_PLPGSQL_EXPR PLpgSQL_Expr
  | MODE_PLPGSQL_ASSIGN1 PLAssignStmt
  | MODE_PLPGSQL_ASSIGN2 PLAssignStmt
  | MODE_PLPGSQL_ASSIGN3 PLAssignStmt
;

stmtmulti :
    toplevel_stmt stmtmulti_1
  | toplevel_stmt
;

stmtmulti_1 :
    ';' toplevel_stmt stmtmulti_1
  | ';' toplevel_stmt
;

toplevel_stmt :
    stmt
  | TransactionStmtLegacy
;

stmt :
    AlterEventTrigStmt
  | AlterCollationStmt
  | AlterDatabaseStmt
  | AlterDatabaseSetStmt
  | AlterDefaultPrivilegesStmt
  | AlterDomainStmt
  | AlterEnumStmt
  | AlterExtensionStmt
  | AlterExtensionContentsStmt
  | AlterFdwStmt
  | AlterForeignServerStmt
  | AlterFunctionStmt
  | AlterGroupStmt
  | AlterObjectDependsStmt
  | AlterObjectSchemaStmt
  | AlterOwnerStmt
  | AlterOperatorStmt
  | AlterTypeStmt
  | AlterPolicyStmt
  | AlterSeqStmt
  | AlterSystemStmt
  | AlterTableStmt
  | AlterTblSpcStmt
  | AlterCompositeTypeStmt
  | AlterPublicationStmt
  | AlterRoleSetStmt
  | AlterRoleStmt
  | AlterSubscriptionStmt
  | AlterStatsStmt
  | AlterTSConfigurationStmt
  | AlterTSDictionaryStmt
  | AlterUserMappingStmt
  | AnalyzeStmt
  | CallStmt
  | CHECKPOINT
  | ClosePortalStmt
  | ClusterStmt
  | CommentStmt
  | ConstraintsSetStmt
  | CopyStmt
  | CreateAmStmt
  | CreateAsStmt
  | CreateAssertionStmt
  | CreateCastStmt
  | CreateConversionStmt
  | CreateDomainStmt
  | CreateExtensionStmt
  | CreateFdwStmt
  | CreateForeignServerStmt
  | CreateForeignTableStmt
  | CreateFunctionStmt
  | CreateGroupStmt
  | CreateMatViewStmt
  | CreateOpClassStmt
  | CreateOpFamilyStmt
  | CreatePublicationStmt
  | AlterOpFamilyStmt
  | CreatePolicyStmt
  | CreatePLangStmt
  | CreateSchemaStmt
  | CreateSeqStmt
  | CreateStmt
  | CreateSubscriptionStmt
  | CreateStatsStmt
  | CreateTableSpaceStmt
  | CreateTransformStmt
  | CreateTrigStmt
  | CreateEventTrigStmt
  | CreateRoleStmt
  | CreateUserStmt
  | CreateUserMappingStmt
  | CreatedbStmt
  | DeallocateStmt
  | DeclareCursorStmt
  | DefineStmt
  | DeleteStmt
  | DiscardStmt
  | DoStmt
  | DropCastStmt
  | DropOpClassStmt
  | DropOpFamilyStmt
  | DropOwnedStmt
  | DropStmt
  | DropSubscriptionStmt
  | DropTableSpaceStmt
  | DropTransformStmt
  | DropRoleStmt
  | DropUserMappingStmt
  | DropdbStmt
  | ExecuteStmt
  | ExplainStmt
  | FetchStmt
  | GrantStmt
  | GrantRoleStmt
  | ImportForeignSchemaStmt
  | IndexStmt
  | InsertStmt
  | ListenStmt
  | RefreshMatViewStmt
  | LoadStmt
  | LockStmt
  | MergeStmt
  | NotifyStmt
  | PrepareStmt
  | ReassignOwnedStmt
  | ReindexStmt
  | RemoveAggrStmt
  | RemoveFuncStmt
  | RemoveOperStmt
  | RenameStmt
  | RevokeStmt
  | RevokeRoleStmt
  | RuleStmt
  | SecLabelStmt
  | SelectStmt
  | TransactionStmt
  | TruncateStmt
  | UnlistenStmt
  | UpdateStmt
  | VacuumStmt
  | VariableResetStmt
  | VariableSetStmt
  | VariableShowStmt
  | ViewStmt
  | __empty
;

opt_single_name :
    ColId
  | __empty
;

opt_qualified_name :
    any_name
  | __empty
;

opt_concurrently :
    CONCURRENTLY
  | __empty
;

opt_drop_behavior :
    CASCADE
  | RESTRICT
  | __empty
;

CallStmt :
    CALL func_application
;

CreateRoleStmt :
    CREATE ROLE RoleId opt_with OptRoleList
;

opt_with :
    WITH
  | WITH_LA
  | __empty
;

OptRoleList :
    CreateOptRoleElem OptRoleList
  | __empty
;

AlterOptRoleList :
    AlterOptRoleElem AlterOptRoleList
  | __empty
;

AlterOptRoleElem :
    PASSWORD SCONST
  | PASSWORD NULL_P
  | ENCRYPTED PASSWORD SCONST
  | UNENCRYPTED PASSWORD SCONST
  | INHERIT
  | CONNECTION LIMIT SignedIconst
  | VALID UNTIL SCONST
  | USER role_list
  | IDENT
;

CreateOptRoleElem :
    AlterOptRoleElem
  | SYSID ICONST
  | ADMIN role_list
  | ROLE role_list
  | IN_P ROLE role_list
  | IN_P GROUP_P role_list
;

CreateUserStmt :
    CREATE USER RoleId opt_with OptRoleList
;

AlterRoleStmt :
    ALTER ROLE RoleSpec opt_with AlterOptRoleList
  | ALTER USER RoleSpec opt_with AlterOptRoleList
;

opt_in_database :
    __empty
  | IN_P DATABASE ColId
;

AlterRoleSetStmt :
    ALTER ROLE RoleSpec opt_in_database SetResetClause
  | ALTER ROLE ALL opt_in_database SetResetClause
  | ALTER USER RoleSpec opt_in_database SetResetClause
  | ALTER USER ALL opt_in_database SetResetClause
;

DropRoleStmt :
    DROP ROLE role_list
  | DROP ROLE IF_P EXISTS role_list
  | DROP USER role_list
  | DROP USER IF_P EXISTS role_list
  | DROP GROUP_P role_list
  | DROP GROUP_P IF_P EXISTS role_list
;

CreateGroupStmt :
    CREATE GROUP_P RoleId opt_with OptRoleList
;

AlterGroupStmt :
    ALTER GROUP_P RoleSpec add_drop USER role_list
;

add_drop :
    ADD_P
  | DROP
;

CreateSchemaStmt :
    CREATE SCHEMA opt_single_name AUTHORIZATION RoleSpec OptSchemaEltList
  | CREATE SCHEMA ColId OptSchemaEltList
  | CREATE SCHEMA IF_P NOT EXISTS opt_single_name AUTHORIZATION RoleSpec OptSchemaEltList
  | CREATE SCHEMA IF_P NOT EXISTS ColId OptSchemaEltList
;

OptSchemaEltList :
    schema_stmt OptSchemaEltList
  | __empty
;

schema_stmt :
    CreateStmt
  | IndexStmt
  | CreateSeqStmt
  | CreateTrigStmt
  | GrantStmt
  | ViewStmt
;

VariableSetStmt :
    SET set_rest
  | SET LOCAL set_rest
  | SET SESSION set_rest
;

set_rest :
    TRANSACTION transaction_mode_list
  | SESSION CHARACTERISTICS AS TRANSACTION transaction_mode_list
  | set_rest_more
;

generic_set :
    var_name TO var_list
  | var_name '=' var_list
  | var_name TO DEFAULT
  | var_name '=' DEFAULT
;

set_rest_more :
    generic_set
  | var_name FROM CURRENT_P
  | TIME ZONE zone_value
  | CATALOG_P SCONST
  | SCHEMA SCONST
  | NAMES opt_encoding
  | ROLE NonReservedWord_or_Sconst
  | SESSION AUTHORIZATION NonReservedWord_or_Sconst
  | SESSION AUTHORIZATION DEFAULT
  | XML_P OPTION document_or_content
  | TRANSACTION SNAPSHOT SCONST
;

var_name :
    ColId var_name_1
  | ColId
;

var_name_1 :
    '.' ColId var_name_1
  | '.' ColId
;

var_list :
    var_value var_list_1
  | var_value
;

var_list_1 :
    ',' var_value var_list_1
  | ',' var_value
;

var_value :
    opt_boolean_or_string
  | NumericOnly
;

iso_level :
    READ UNCOMMITTED
  | READ COMMITTED
  | REPEATABLE READ
  | SERIALIZABLE
;

opt_boolean_or_string :
    TRUE_P
  | FALSE_P
  | ON
  | NonReservedWord_or_Sconst
;

zone_value :
    SCONST
  | IDENT
  | INTERVAL SCONST opt_interval
  | INTERVAL '(' ICONST ')' SCONST
  | NumericOnly
  | DEFAULT
  | LOCAL
;

opt_encoding :
    SCONST
  | DEFAULT
  | __empty
;

NonReservedWord_or_Sconst :
    NonReservedWord
  | SCONST
;

VariableResetStmt :
    RESET reset_rest
;

reset_rest :
    generic_reset
  | TIME ZONE
  | TRANSACTION ISOLATION LEVEL
  | SESSION AUTHORIZATION
;

generic_reset :
    var_name
  | ALL
;

SetResetClause :
    SET set_rest
  | VariableResetStmt
;

FunctionSetResetClause :
    SET set_rest_more
  | VariableResetStmt
;

VariableShowStmt :
    SHOW var_name
  | SHOW TIME ZONE
  | SHOW TRANSACTION ISOLATION LEVEL
  | SHOW SESSION AUTHORIZATION
  | SHOW ALL
;

ConstraintsSetStmt :
    SET CONSTRAINTS constraints_set_list constraints_set_mode
;

constraints_set_list :
    ALL
  | qualified_name_list
;

constraints_set_mode :
    DEFERRED
  | IMMEDIATE
;

DiscardStmt :
    DISCARD ALL
  | DISCARD TEMP
  | DISCARD TEMPORARY
  | DISCARD PLANS
  | DISCARD SEQUENCES
;

AlterTableStmt :
    ALTER TABLE relation_expr alter_table_cmds
  | ALTER TABLE IF_P EXISTS relation_expr alter_table_cmds
  | ALTER TABLE relation_expr partition_cmd
  | ALTER TABLE IF_P EXISTS relation_expr partition_cmd
  | ALTER TABLE ALL IN_P TABLESPACE ColId SET TABLESPACE ColId opt_nowait
  | ALTER TABLE ALL IN_P TABLESPACE ColId OWNED BY role_list SET TABLESPACE ColId opt_nowait
  | ALTER INDEX qualified_name alter_table_cmds
  | ALTER INDEX IF_P EXISTS qualified_name alter_table_cmds
  | ALTER INDEX qualified_name index_partition_cmd
  | ALTER INDEX ALL IN_P TABLESPACE ColId SET TABLESPACE ColId opt_nowait
  | ALTER INDEX ALL IN_P TABLESPACE ColId OWNED BY role_list SET TABLESPACE ColId opt_nowait
  | ALTER SEQUENCE qualified_name alter_table_cmds
  | ALTER SEQUENCE IF_P EXISTS qualified_name alter_table_cmds
  | ALTER VIEW qualified_name alter_table_cmds
  | ALTER VIEW IF_P EXISTS qualified_name alter_table_cmds
  | ALTER MATERIALIZED VIEW qualified_name alter_table_cmds
  | ALTER MATERIALIZED VIEW IF_P EXISTS qualified_name alter_table_cmds
  | ALTER MATERIALIZED VIEW ALL IN_P TABLESPACE ColId SET TABLESPACE ColId opt_nowait
  | ALTER MATERIALIZED VIEW ALL IN_P TABLESPACE ColId OWNED BY role_list SET TABLESPACE ColId opt_nowait
  | ALTER FOREIGN TABLE relation_expr alter_table_cmds
  | ALTER FOREIGN TABLE IF_P EXISTS relation_expr alter_table_cmds
;

alter_table_cmds :
    alter_table_cmd alter_table_cmds_1
  | alter_table_cmd
;

alter_table_cmds_1 :
    ',' alter_table_cmd alter_table_cmds_1
  | ',' alter_table_cmd
;

partition_cmd :
    ATTACH PARTITION qualified_name PartitionBoundSpec
  | DETACH PARTITION qualified_name opt_concurrently
  | DETACH PARTITION qualified_name FINALIZE
;

index_partition_cmd :
    ATTACH PARTITION qualified_name
;

alter_table_cmd :
    ADD_P columnDef
  | ADD_P IF_P NOT EXISTS columnDef
  | ADD_P COLUMN columnDef
  | ADD_P COLUMN IF_P NOT EXISTS columnDef
  | ALTER opt_column ColId alter_column_default
  | ALTER opt_column ColId DROP NOT NULL_P
  | ALTER opt_column ColId SET NOT NULL_P
  | ALTER opt_column ColId SET EXPRESSION AS '(' a_expr ')'
  | ALTER opt_column ColId DROP EXPRESSION
  | ALTER opt_column ColId DROP EXPRESSION IF_P EXISTS
  | ALTER opt_column ColId SET STATISTICS set_statistics_value
  | ALTER opt_column ICONST SET STATISTICS set_statistics_value
  | ALTER opt_column ColId SET reloptions
  | ALTER opt_column ColId RESET reloptions
  | ALTER opt_column ColId SET column_storage
  | ALTER opt_column ColId SET column_compression
  | ALTER opt_column ColId ADD_P GENERATED generated_when AS IDENTITY_P OptParenthesizedSeqOptList
  | ALTER opt_column ColId alter_identity_column_option_list
  | ALTER opt_column ColId DROP IDENTITY_P
  | ALTER opt_column ColId DROP IDENTITY_P IF_P EXISTS
  | DROP opt_column IF_P EXISTS ColId opt_drop_behavior
  | DROP opt_column ColId opt_drop_behavior
  | ALTER opt_column ColId opt_set_data TYPE_P Typename opt_collate_clause alter_using
  | ALTER opt_column ColId alter_generic_options
  | ADD_P TableConstraint
  | ALTER CONSTRAINT ColId ConstraintAttributeSpec
  | ALTER CONSTRAINT ColId INHERIT
  | VALIDATE CONSTRAINT ColId
  | DROP CONSTRAINT IF_P EXISTS ColId opt_drop_behavior
  | DROP CONSTRAINT ColId opt_drop_behavior
  | SET WITHOUT OIDS
  | CLUSTER ON ColId
  | SET WITHOUT CLUSTER
  | SET LOGGED
  | SET UNLOGGED
  | ENABLE_P TRIGGER ColId
  | ENABLE_P ALWAYS TRIGGER ColId
  | ENABLE_P REPLICA TRIGGER ColId
  | ENABLE_P TRIGGER ALL
  | ENABLE_P TRIGGER USER
  | DISABLE_P TRIGGER ColId
  | DISABLE_P TRIGGER ALL
  | DISABLE_P TRIGGER USER
  | ENABLE_P RULE ColId
  | ENABLE_P ALWAYS RULE ColId
  | ENABLE_P REPLICA RULE ColId
  | DISABLE_P RULE ColId
  | INHERIT qualified_name
  | NO INHERIT qualified_name
  | OF any_name
  | NOT OF
  | OWNER TO RoleSpec
  | SET ACCESS METHOD set_access_method_name
  | SET TABLESPACE ColId
  | SET reloptions
  | RESET reloptions
  | REPLICA IDENTITY_P replica_identity
  | ENABLE_P ROW LEVEL SECURITY
  | DISABLE_P ROW LEVEL SECURITY
  | FORCE ROW LEVEL SECURITY
  | NO FORCE ROW LEVEL SECURITY
  | alter_generic_options
;

alter_column_default :
    SET DEFAULT a_expr
  | DROP DEFAULT
;

opt_collate_clause :
    COLLATE any_name
  | __empty
;

alter_using :
    USING a_expr
  | __empty
;

replica_identity :
    NOTHING
  | FULL
  | DEFAULT
  | USING INDEX ColId
;

reloptions :
    '(' reloption_list ')'
;

opt_reloptions :
    WITH reloptions
  | __empty
;

reloption_list :
    reloption_elem reloption_list_1
  | reloption_elem
;

reloption_list_1 :
    ',' reloption_elem reloption_list_1
  | ',' reloption_elem
;

reloption_elem :
    ColLabel '=' def_arg
  | ColLabel
  | ColLabel '.' ColLabel '=' def_arg
  | ColLabel '.' ColLabel
;

alter_identity_column_option_list :
    alter_identity_column_option alter_identity_column_option_list
  | alter_identity_column_option
;

alter_identity_column_option :
    RESTART
  | RESTART opt_with NumericOnly
  | SET SeqOptElem
  | SET GENERATED generated_when
;

set_statistics_value :
    SignedIconst
  | DEFAULT
;

set_access_method_name :
    ColId
  | DEFAULT
;

PartitionBoundSpec :
    FOR VALUES WITH '(' hash_partbound ')'
  | FOR VALUES IN_P '(' expr_list ')'
  | FOR VALUES FROM '(' expr_list ')' TO '(' expr_list ')'
  | DEFAULT
;

hash_partbound_elem :
    NonReservedWord ICONST
;

hash_partbound :
    hash_partbound_elem hash_partbound_1
  | hash_partbound_elem
;

hash_partbound_1 :
    ',' hash_partbound_elem hash_partbound_1
  | ',' hash_partbound_elem
;

AlterCompositeTypeStmt :
    ALTER TYPE_P any_name alter_type_cmds
;

alter_type_cmds :
    alter_type_cmd alter_type_cmds_1
  | alter_type_cmd
;

alter_type_cmds_1 :
    ',' alter_type_cmd alter_type_cmds_1
  | ',' alter_type_cmd
;

alter_type_cmd :
    ADD_P ATTRIBUTE TableFuncElement opt_drop_behavior
  | DROP ATTRIBUTE IF_P EXISTS ColId opt_drop_behavior
  | DROP ATTRIBUTE ColId opt_drop_behavior
  | ALTER ATTRIBUTE ColId opt_set_data TYPE_P Typename opt_collate_clause opt_drop_behavior
;

ClosePortalStmt :
    CLOSE ColId
  | CLOSE ALL
;

CopyStmt :
    COPY opt_binary qualified_name opt_column_list copy_from opt_program copy_file_name copy_delimiter opt_with copy_options where_clause
  | COPY '(' PreparableStmt ')' TO opt_program copy_file_name opt_with copy_options
;

copy_from :
    FROM
  | TO
;

opt_program :
    PROGRAM
  | __empty
;

copy_file_name :
    SCONST
  | STDIN
  | STDOUT
;

copy_options :
    copy_opt_list
  | '(' copy_generic_opt_list ')'
;

copy_opt_list :
    copy_opt_item copy_opt_list
  | __empty
;

copy_opt_item :
    BINARY
  | FREEZE
  | DELIMITER opt_as SCONST
  | NULL_P opt_as SCONST
  | CSV
  | HEADER_P
  | QUOTE opt_as SCONST
  | ESCAPE opt_as SCONST
  | FORCE QUOTE columnList
  | FORCE QUOTE '*'
  | FORCE NOT NULL_P columnList
  | FORCE NOT NULL_P '*'
  | FORCE NULL_P columnList
  | FORCE NULL_P '*'
  | ENCODING SCONST
;

opt_binary :
    BINARY
  | __empty
;

copy_delimiter :
    opt_using DELIMITERS SCONST
  | __empty
;

opt_using :
    USING
  | __empty
;

copy_generic_opt_list :
    copy_generic_opt_elem copy_generic_opt_list_1
  | copy_generic_opt_elem
;

copy_generic_opt_list_1 :
    ',' copy_generic_opt_elem copy_generic_opt_list_1
  | ',' copy_generic_opt_elem
;

copy_generic_opt_elem :
    ColLabel copy_generic_opt_arg
;

copy_generic_opt_arg :
    opt_boolean_or_string
  | NumericOnly
  | '*'
  | DEFAULT
  | '(' copy_generic_opt_arg_list ')'
  | __empty
;

copy_generic_opt_arg_list :
    opt_boolean_or_string copy_generic_opt_arg_list_1
  | opt_boolean_or_string
;

copy_generic_opt_arg_list_1 :
    ',' opt_boolean_or_string copy_generic_opt_arg_list_1
  | ',' opt_boolean_or_string
;

CreateStmt :
    CREATE OptTemp TABLE qualified_name '(' OptTableElementList ')' OptInherit OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
  | CREATE OptTemp TABLE IF_P NOT EXISTS qualified_name '(' OptTableElementList ')' OptInherit OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
  | CREATE OptTemp TABLE qualified_name OF any_name OptTypedTableElementList OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
  | CREATE OptTemp TABLE IF_P NOT EXISTS qualified_name OF any_name OptTypedTableElementList OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
  | CREATE OptTemp TABLE qualified_name PARTITION OF qualified_name OptTypedTableElementList PartitionBoundSpec OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
  | CREATE OptTemp TABLE IF_P NOT EXISTS qualified_name PARTITION OF qualified_name OptTypedTableElementList PartitionBoundSpec OptPartitionSpec table_access_method_clause OptWith OnCommitOption OptTableSpace
;

OptTemp :
    TEMPORARY
  | TEMP
  | LOCAL TEMPORARY
  | LOCAL TEMP
  | GLOBAL TEMPORARY
  | GLOBAL TEMP
  | UNLOGGED
  | __empty
;

OptTableElementList :
    TableElementList
  | __empty
;

OptTypedTableElementList :
    '(' TypedTableElementList ')'
  | __empty
;

TableElementList :
    TableElement TableElementList_1
  | TableElement
;

TableElementList_1 :
    ',' TableElement TableElementList_1
  | ',' TableElement
;

TypedTableElementList :
    TypedTableElement TypedTableElementList_1
  | TypedTableElement
;

TypedTableElementList_1 :
    ',' TypedTableElement TypedTableElementList_1
  | ',' TypedTableElement
;

TableElement :
    columnDef
  | TableLikeClause
  | TableConstraint
;

TypedTableElement :
    columnOptions
  | TableConstraint
;

columnDef :
    ColId Typename opt_column_storage opt_column_compression create_generic_options ColQualList
;

columnOptions :
    ColId ColQualList
  | ColId WITH OPTIONS ColQualList
;

column_compression :
    COMPRESSION ColId
  | COMPRESSION DEFAULT
;

opt_column_compression :
    column_compression
  | __empty
;

column_storage :
    STORAGE ColId
  | STORAGE DEFAULT
;

opt_column_storage :
    column_storage
  | __empty
;

ColQualList :
    ColConstraint ColQualList
  | __empty
;

ColConstraint :
    CONSTRAINT ColId ColConstraintElem
  | ColConstraintElem
  | ConstraintAttr
  | COLLATE any_name
;

ColConstraintElem :
    NOT NULL_P opt_no_inherit
  | NULL_P
  | UNIQUE opt_unique_null_treatment opt_definition OptConsTableSpace
  | PRIMARY KEY opt_definition OptConsTableSpace
  | CHECK '(' a_expr ')' opt_no_inherit
  | DEFAULT b_expr
  | GENERATED generated_when AS IDENTITY_P OptParenthesizedSeqOptList
  | GENERATED generated_when AS '(' a_expr ')' opt_virtual_or_stored
  | REFERENCES qualified_name opt_column_list key_match key_actions
;

opt_unique_null_treatment :
    NULLS_P DISTINCT
  | NULLS_P NOT DISTINCT
  | __empty
;

generated_when :
    ALWAYS
  | BY DEFAULT
;

opt_virtual_or_stored :
    STORED
  | VIRTUAL
  | __empty
;

ConstraintAttr :
    DEFERRABLE
  | NOT DEFERRABLE
  | INITIALLY DEFERRED
  | INITIALLY IMMEDIATE
  | ENFORCED
  | NOT ENFORCED
;

TableLikeClause :
    LIKE qualified_name TableLikeOptionList
;

TableLikeOptionList :
    TableLikeOptionList_1 TableLikeOptionList
  | __empty
;

TableLikeOptionList_1 :
    INCLUDING TableLikeOption
  | EXCLUDING TableLikeOption
;

TableLikeOption :
    COMMENTS
  | COMPRESSION
  | CONSTRAINTS
  | DEFAULTS
  | IDENTITY_P
  | GENERATED
  | INDEXES
  | STATISTICS
  | STORAGE
  | ALL
;

TableConstraint :
    CONSTRAINT ColId ConstraintElem
  | ConstraintElem
;

ConstraintElem :
    CHECK '(' a_expr ')' ConstraintAttributeSpec
  | NOT NULL_P ColId ConstraintAttributeSpec
  | UNIQUE opt_unique_null_treatment '(' columnList opt_without_overlaps ')' opt_c_include opt_definition OptConsTableSpace ConstraintAttributeSpec
  | UNIQUE ExistingIndex ConstraintAttributeSpec
  | PRIMARY KEY '(' columnList opt_without_overlaps ')' opt_c_include opt_definition OptConsTableSpace ConstraintAttributeSpec
  | PRIMARY KEY ExistingIndex ConstraintAttributeSpec
  | EXCLUDE access_method_clause '(' ExclusionConstraintList ')' opt_c_include opt_definition OptConsTableSpace OptWhereClause ConstraintAttributeSpec
  | FOREIGN KEY '(' columnList optionalPeriodName ')' REFERENCES qualified_name opt_column_and_period_list key_match key_actions ConstraintAttributeSpec
;

DomainConstraint :
    CONSTRAINT ColId DomainConstraintElem
  | DomainConstraintElem
;

DomainConstraintElem :
    CHECK '(' a_expr ')' ConstraintAttributeSpec
  | NOT NULL_P ConstraintAttributeSpec
;

opt_no_inherit :
    NO INHERIT
  | __empty
;

opt_without_overlaps :
    WITHOUT OVERLAPS
  | __empty
;

opt_column_list :
    '(' columnList ')'
  | __empty
;

columnList :
    ColId columnList_1
  | ColId
;

columnList_1 :
    ',' ColId columnList_1
  | ',' ColId
;

optionalPeriodName :
    ',' PERIOD ColId
  | __empty
;

opt_column_and_period_list :
    '(' columnList optionalPeriodName ')'
  | __empty
;

opt_c_include :
    INCLUDE '(' columnList ')'
  | __empty
;

key_match :
    MATCH FULL
  | MATCH PARTIAL
  | MATCH SIMPLE
  | __empty
;

ExclusionConstraintList :
    ExclusionConstraintElem ExclusionConstraintList_1
  | ExclusionConstraintElem
;

ExclusionConstraintList_1 :
    ',' ExclusionConstraintElem ExclusionConstraintList_1
  | ',' ExclusionConstraintElem
;

ExclusionConstraintElem :
    index_elem WITH any_operator
  | index_elem WITH OPERATOR '(' any_operator ')'
;

OptWhereClause :
    WHERE '(' a_expr ')'
  | __empty
;

key_actions :
    key_update
  | key_delete
  | key_update key_delete
  | key_delete key_update
  | __empty
;

key_update :
    ON UPDATE key_action
;

key_delete :
    ON DELETE_P key_action
;

key_action :
    NO ACTION
  | RESTRICT
  | CASCADE
  | SET NULL_P opt_column_list
  | SET DEFAULT opt_column_list
;

OptInherit :
    INHERITS '(' qualified_name_list ')'
  | __empty
;

OptPartitionSpec :
    PartitionSpec
  | __empty
;

PartitionSpec :
    PARTITION BY ColId '(' part_params ')'
;

part_params :
    part_elem part_params_1
  | part_elem
;

part_params_1 :
    ',' part_elem part_params_1
  | ',' part_elem
;

part_elem :
    ColId opt_collate opt_qualified_name
  | func_expr_windowless opt_collate opt_qualified_name
  | '(' a_expr ')' opt_collate opt_qualified_name
;

table_access_method_clause :
    USING ColId
  | __empty
;

OptWith :
    WITH reloptions
  | WITHOUT OIDS
  | __empty
;

OnCommitOption :
    ON COMMIT DROP
  | ON COMMIT DELETE_P ROWS
  | ON COMMIT PRESERVE ROWS
  | __empty
;

OptTableSpace :
    TABLESPACE ColId
  | __empty
;

OptConsTableSpace :
    USING INDEX TABLESPACE ColId
  | __empty
;

ExistingIndex :
    USING INDEX ColId
;

CreateStatsStmt :
    CREATE STATISTICS opt_qualified_name opt_name_list ON stats_params FROM from_list
  | CREATE STATISTICS IF_P NOT EXISTS any_name opt_name_list ON stats_params FROM from_list
;

stats_params :
    stats_param stats_params_1
  | stats_param
;

stats_params_1 :
    ',' stats_param stats_params_1
  | ',' stats_param
;

stats_param :
    ColId
  | func_expr_windowless
  | '(' a_expr ')'
;

AlterStatsStmt :
    ALTER STATISTICS any_name SET STATISTICS set_statistics_value
  | ALTER STATISTICS IF_P EXISTS any_name SET STATISTICS set_statistics_value
;

CreateAsStmt :
    CREATE OptTemp TABLE create_as_target AS SelectStmt opt_with_data
  | CREATE OptTemp TABLE IF_P NOT EXISTS create_as_target AS SelectStmt opt_with_data
;

create_as_target :
    qualified_name opt_column_list table_access_method_clause OptWith OnCommitOption OptTableSpace
;

opt_with_data :
    WITH DATA_P
  | WITH NO DATA_P
  | __empty
;

CreateMatViewStmt :
    CREATE OptNoLog MATERIALIZED VIEW create_mv_target AS SelectStmt opt_with_data
  | CREATE OptNoLog MATERIALIZED VIEW IF_P NOT EXISTS create_mv_target AS SelectStmt opt_with_data
;

create_mv_target :
    qualified_name opt_column_list table_access_method_clause opt_reloptions OptTableSpace
;

OptNoLog :
    UNLOGGED
  | __empty
;

RefreshMatViewStmt :
    REFRESH MATERIALIZED VIEW opt_concurrently qualified_name opt_with_data
;

CreateSeqStmt :
    CREATE OptTemp SEQUENCE qualified_name OptSeqOptList
  | CREATE OptTemp SEQUENCE IF_P NOT EXISTS qualified_name OptSeqOptList
;

AlterSeqStmt :
    ALTER SEQUENCE qualified_name SeqOptList
  | ALTER SEQUENCE IF_P EXISTS qualified_name SeqOptList
;

OptSeqOptList :
    SeqOptList
  | __empty
;

OptParenthesizedSeqOptList :
    '(' SeqOptList ')'
  | __empty
;

SeqOptList :
    SeqOptElem SeqOptList
  | SeqOptElem
;

SeqOptElem :
    AS SimpleTypename
  | CACHE NumericOnly
  | CYCLE
  | NO CYCLE
  | INCREMENT opt_by NumericOnly
  | LOGGED
  | MAXVALUE NumericOnly
  | MINVALUE NumericOnly
  | NO MAXVALUE
  | NO MINVALUE
  | OWNED BY any_name
  | SEQUENCE NAME_P any_name
  | START opt_with NumericOnly
  | RESTART
  | RESTART opt_with NumericOnly
  | UNLOGGED
;

opt_by :
    BY
  | __empty
;

NumericOnly :
    FCONST
  | '+' FCONST
  | '-' FCONST
  | SignedIconst
;

NumericOnly_list :
    NumericOnly NumericOnly_list_1
  | NumericOnly
;

NumericOnly_list_1 :
    ',' NumericOnly NumericOnly_list_1
  | ',' NumericOnly
;

CreatePLangStmt :
    CREATE opt_or_replace opt_trusted opt_procedural LANGUAGE ColId
  | CREATE opt_or_replace opt_trusted opt_procedural LANGUAGE ColId HANDLER handler_name opt_inline_handler opt_validator
;

opt_trusted :
    TRUSTED
  | __empty
;

handler_name :
    ColId
  | ColId attrs
;

opt_inline_handler :
    INLINE_P handler_name
  | __empty
;

validator_clause :
    VALIDATOR handler_name
  | NO VALIDATOR
;

opt_validator :
    validator_clause
  | __empty
;

opt_procedural :
    PROCEDURAL
  | __empty
;

CreateTableSpaceStmt :
    CREATE TABLESPACE ColId OptTableSpaceOwner LOCATION SCONST opt_reloptions
;

OptTableSpaceOwner :
    OWNER RoleSpec
  | __empty
;

DropTableSpaceStmt :
    DROP TABLESPACE ColId
  | DROP TABLESPACE IF_P EXISTS ColId
;

CreateExtensionStmt :
    CREATE EXTENSION ColId opt_with create_extension_opt_list
  | CREATE EXTENSION IF_P NOT EXISTS ColId opt_with create_extension_opt_list
;

create_extension_opt_list :
    create_extension_opt_item create_extension_opt_list
  | __empty
;

create_extension_opt_item :
    SCHEMA ColId
  | VERSION_P NonReservedWord_or_Sconst
  | FROM NonReservedWord_or_Sconst
  | CASCADE
;

AlterExtensionStmt :
    ALTER EXTENSION ColId UPDATE alter_extension_opt_list
;

alter_extension_opt_list :
    alter_extension_opt_item alter_extension_opt_list
  | __empty
;

alter_extension_opt_item :
    TO NonReservedWord_or_Sconst
;

AlterExtensionContentsStmt :
    ALTER EXTENSION ColId add_drop object_type_name ColId
  | ALTER EXTENSION ColId add_drop object_type_any_name any_name
  | ALTER EXTENSION ColId add_drop AGGREGATE aggregate_with_argtypes
  | ALTER EXTENSION ColId add_drop CAST '(' Typename AS Typename ')'
  | ALTER EXTENSION ColId add_drop DOMAIN_P Typename
  | ALTER EXTENSION ColId add_drop FUNCTION function_with_argtypes
  | ALTER EXTENSION ColId add_drop OPERATOR operator_with_argtypes
  | ALTER EXTENSION ColId add_drop OPERATOR CLASS any_name USING ColId
  | ALTER EXTENSION ColId add_drop OPERATOR FAMILY any_name USING ColId
  | ALTER EXTENSION ColId add_drop PROCEDURE function_with_argtypes
  | ALTER EXTENSION ColId add_drop ROUTINE function_with_argtypes
  | ALTER EXTENSION ColId add_drop TRANSFORM FOR Typename LANGUAGE ColId
  | ALTER EXTENSION ColId add_drop TYPE_P Typename
;

CreateFdwStmt :
    CREATE FOREIGN DATA_P WRAPPER ColId opt_fdw_options create_generic_options
;

fdw_option :
    HANDLER handler_name
  | NO HANDLER
  | VALIDATOR handler_name
  | NO VALIDATOR
;

fdw_options :
    fdw_option fdw_options
  | fdw_option
;

opt_fdw_options :
    fdw_options
  | __empty
;

AlterFdwStmt :
    ALTER FOREIGN DATA_P WRAPPER ColId opt_fdw_options alter_generic_options
  | ALTER FOREIGN DATA_P WRAPPER ColId fdw_options
;

create_generic_options :
    OPTIONS '(' generic_option_list ')'
  | __empty
;

generic_option_list :
    generic_option_elem generic_option_list_1
  | generic_option_elem
;

generic_option_list_1 :
    ',' generic_option_elem generic_option_list_1
  | ',' generic_option_elem
;

alter_generic_options :
    OPTIONS '(' alter_generic_option_list ')'
;

alter_generic_option_list :
    alter_generic_option_elem alter_generic_option_list_1
  | alter_generic_option_elem
;

alter_generic_option_list_1 :
    ',' alter_generic_option_elem alter_generic_option_list_1
  | ',' alter_generic_option_elem
;

alter_generic_option_elem :
    generic_option_elem
  | SET generic_option_elem
  | ADD_P generic_option_elem
  | DROP ColLabel
;

generic_option_elem :
    ColLabel SCONST
;

CreateForeignServerStmt :
    CREATE SERVER ColId opt_type opt_foreign_server_version FOREIGN DATA_P WRAPPER ColId create_generic_options
  | CREATE SERVER IF_P NOT EXISTS ColId opt_type opt_foreign_server_version FOREIGN DATA_P WRAPPER ColId create_generic_options
;

opt_type :
    TYPE_P SCONST
  | __empty
;

foreign_server_version :
    VERSION_P SCONST
  | VERSION_P NULL_P
;

opt_foreign_server_version :
    foreign_server_version
  | __empty
;

AlterForeignServerStmt :
    ALTER SERVER ColId foreign_server_version alter_generic_options
  | ALTER SERVER ColId foreign_server_version
  | ALTER SERVER ColId alter_generic_options
;

CreateForeignTableStmt :
    CREATE FOREIGN TABLE qualified_name '(' OptTableElementList ')' OptInherit SERVER ColId create_generic_options
  | CREATE FOREIGN TABLE IF_P NOT EXISTS qualified_name '(' OptTableElementList ')' OptInherit SERVER ColId create_generic_options
  | CREATE FOREIGN TABLE qualified_name PARTITION OF qualified_name OptTypedTableElementList PartitionBoundSpec SERVER ColId create_generic_options
  | CREATE FOREIGN TABLE IF_P NOT EXISTS qualified_name PARTITION OF qualified_name OptTypedTableElementList PartitionBoundSpec SERVER ColId create_generic_options
;

ImportForeignSchemaStmt :
    IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
;

import_qualification_type :
    LIMIT TO
  | EXCEPT
;

import_qualification :
    import_qualification_type '(' relation_expr_list ')'
  | __empty
;

CreateUserMappingStmt :
    CREATE USER MAPPING FOR auth_ident SERVER ColId create_generic_options
  | CREATE USER MAPPING IF_P NOT EXISTS FOR auth_ident SERVER ColId create_generic_options
;

auth_ident :
    RoleSpec
  | USER
;

DropUserMappingStmt :
    DROP USER MAPPING FOR auth_ident SERVER ColId
  | DROP USER MAPPING IF_P EXISTS FOR auth_ident SERVER ColId
;

AlterUserMappingStmt :
    ALTER USER MAPPING FOR auth_ident SERVER ColId alter_generic_options
;

CreatePolicyStmt :
    CREATE POLICY ColId ON qualified_name RowSecurityDefaultPermissive RowSecurityDefaultForCmd RowSecurityDefaultToRole RowSecurityOptionalExpr RowSecurityOptionalWithCheck
;

AlterPolicyStmt :
    ALTER POLICY ColId ON qualified_name RowSecurityOptionalToRole RowSecurityOptionalExpr RowSecurityOptionalWithCheck
;

RowSecurityOptionalExpr :
    USING '(' a_expr ')'
  | __empty
;

RowSecurityOptionalWithCheck :
    WITH CHECK '(' a_expr ')'
  | __empty
;

RowSecurityDefaultToRole :
    TO role_list
  | __empty
;

RowSecurityOptionalToRole :
    TO role_list
  | __empty
;

RowSecurityDefaultPermissive :
    AS IDENT
  | __empty
;

RowSecurityDefaultForCmd :
    FOR row_security_cmd
  | __empty
;

row_security_cmd :
    ALL
  | SELECT
  | INSERT
  | UPDATE
  | DELETE_P
;

CreateAmStmt :
    CREATE ACCESS METHOD ColId TYPE_P am_type HANDLER handler_name
;

am_type :
    INDEX
  | TABLE
;

CreateTrigStmt :
    CREATE opt_or_replace TRIGGER ColId TriggerActionTime TriggerEvents ON qualified_name TriggerReferencing TriggerForSpec TriggerWhen EXECUTE FUNCTION_or_PROCEDURE func_name '(' TriggerFuncArgs ')'
  | CREATE opt_or_replace CONSTRAINT TRIGGER ColId AFTER TriggerEvents ON qualified_name OptConstrFromTable ConstraintAttributeSpec FOR EACH ROW TriggerWhen EXECUTE FUNCTION_or_PROCEDURE func_name '(' TriggerFuncArgs ')'
;

TriggerActionTime :
    BEFORE
  | AFTER
  | INSTEAD OF
;

TriggerEvents :
    TriggerOneEvent TriggerEvents_1
  | TriggerOneEvent
;

TriggerEvents_1 :
    OR TriggerOneEvent TriggerEvents_1
  | OR TriggerOneEvent
;

TriggerOneEvent :
    INSERT
  | DELETE_P
  | UPDATE
  | UPDATE OF columnList
  | TRUNCATE
;

TriggerReferencing :
    REFERENCING TriggerTransitions
  | __empty
;

TriggerTransitions :
    TriggerTransition TriggerTransitions
  | TriggerTransition
;

TriggerTransition :
    TransitionOldOrNew TransitionRowOrTable opt_as ColId
;

TransitionOldOrNew :
    NEW
  | OLD
;

TransitionRowOrTable :
    TABLE
  | ROW
;

TriggerForSpec :
    FOR TriggerForOptEach TriggerForType
  | __empty
;

TriggerForOptEach :
    EACH
  | __empty
;

TriggerForType :
    ROW
  | STATEMENT
;

TriggerWhen :
    WHEN '(' a_expr ')'
  | __empty
;

FUNCTION_or_PROCEDURE :
    FUNCTION
  | PROCEDURE
;

TriggerFuncArgs :
    TriggerFuncArgs_1 TriggerFuncArgs_2
  | TriggerFuncArgs_1
;

TriggerFuncArgs_1 :
    TriggerFuncArg
  | __empty
;

TriggerFuncArgs_2 :
    ',' TriggerFuncArg TriggerFuncArgs_2
  | ',' TriggerFuncArg
;

TriggerFuncArg :
    ICONST
  | FCONST
  | SCONST
  | ColLabel
;

OptConstrFromTable :
    FROM qualified_name
  | __empty
;

ConstraintAttributeSpec :
    ConstraintAttributeElem ConstraintAttributeSpec
  | __empty
;

ConstraintAttributeElem :
    NOT DEFERRABLE
  | DEFERRABLE
  | INITIALLY IMMEDIATE
  | INITIALLY DEFERRED
  | NOT VALID
  | NO INHERIT
  | NOT ENFORCED
  | ENFORCED
;

CreateEventTrigStmt :
    CREATE EVENT TRIGGER ColId ON ColLabel EXECUTE FUNCTION_or_PROCEDURE func_name '(' ')'
  | CREATE EVENT TRIGGER ColId ON ColLabel WHEN event_trigger_when_list EXECUTE FUNCTION_or_PROCEDURE func_name '(' ')'
;

event_trigger_when_list :
    event_trigger_when_item event_trigger_when_list_1
  | event_trigger_when_item
;

event_trigger_when_list_1 :
    AND event_trigger_when_item event_trigger_when_list_1
  | AND event_trigger_when_item
;

event_trigger_when_item :
    ColId IN_P '(' event_trigger_value_list ')'
;

event_trigger_value_list :
    SCONST event_trigger_value_list_1
  | SCONST
;

event_trigger_value_list_1 :
    ',' SCONST event_trigger_value_list_1
  | ',' SCONST
;

AlterEventTrigStmt :
    ALTER EVENT TRIGGER ColId enable_trigger
;

enable_trigger :
    ENABLE_P
  | ENABLE_P REPLICA
  | ENABLE_P ALWAYS
  | DISABLE_P
;

CreateAssertionStmt :
    CREATE ASSERTION any_name CHECK '(' a_expr ')' ConstraintAttributeSpec
;

DefineStmt :
    CREATE opt_or_replace AGGREGATE func_name aggr_args definition
  | CREATE opt_or_replace AGGREGATE func_name old_aggr_definition
  | CREATE OPERATOR any_operator definition
  | CREATE TYPE_P any_name definition
  | CREATE TYPE_P any_name
  | CREATE TYPE_P any_name AS '(' OptTableFuncElementList ')'
  | CREATE TYPE_P any_name AS ENUM_P '(' opt_enum_val_list ')'
  | CREATE TYPE_P any_name AS RANGE definition
  | CREATE TEXT_P SEARCH PARSER any_name definition
  | CREATE TEXT_P SEARCH DICTIONARY any_name definition
  | CREATE TEXT_P SEARCH TEMPLATE any_name definition
  | CREATE TEXT_P SEARCH CONFIGURATION any_name definition
  | CREATE COLLATION any_name definition
  | CREATE COLLATION IF_P NOT EXISTS any_name definition
  | CREATE COLLATION any_name FROM any_name
  | CREATE COLLATION IF_P NOT EXISTS any_name FROM any_name
;

definition :
    '(' def_list ')'
;

def_list :
    def_elem def_list_1
  | def_elem
;

def_list_1 :
    ',' def_elem def_list_1
  | ',' def_elem
;

def_elem :
    ColLabel '=' def_arg
  | ColLabel
;

def_arg :
    func_type
  | reserved_keyword
  | qual_all_Op
  | NumericOnly
  | SCONST
  | NONE
;

old_aggr_definition :
    '(' old_aggr_list ')'
;

old_aggr_list :
    old_aggr_elem old_aggr_list_1
  | old_aggr_elem
;

old_aggr_list_1 :
    ',' old_aggr_elem old_aggr_list_1
  | ',' old_aggr_elem
;

old_aggr_elem :
    IDENT '=' def_arg
;

opt_enum_val_list :
    enum_val_list
  | __empty
;

enum_val_list :
    SCONST enum_val_list_1
  | SCONST
;

enum_val_list_1 :
    ',' SCONST enum_val_list_1
  | ',' SCONST
;

AlterEnumStmt :
    ALTER TYPE_P any_name ADD_P VALUE_P opt_if_not_exists SCONST
  | ALTER TYPE_P any_name ADD_P VALUE_P opt_if_not_exists SCONST BEFORE SCONST
  | ALTER TYPE_P any_name ADD_P VALUE_P opt_if_not_exists SCONST AFTER SCONST
  | ALTER TYPE_P any_name RENAME VALUE_P SCONST TO SCONST
  | ALTER TYPE_P any_name DROP VALUE_P SCONST
;

opt_if_not_exists :
    IF_P NOT EXISTS
  | __empty
;

CreateOpClassStmt :
    CREATE OPERATOR CLASS any_name opt_default FOR TYPE_P Typename USING ColId opt_opfamily AS opclass_item_list
;

opclass_item_list :
    opclass_item opclass_item_list_1
  | opclass_item
;

opclass_item_list_1 :
    ',' opclass_item opclass_item_list_1
  | ',' opclass_item
;

opclass_item :
    OPERATOR ICONST any_operator opclass_purpose
  | OPERATOR ICONST operator_with_argtypes opclass_purpose
  | FUNCTION ICONST function_with_argtypes
  | FUNCTION ICONST '(' type_list ')' function_with_argtypes
  | STORAGE Typename
;

opt_default :
    DEFAULT
  | __empty
;

opt_opfamily :
    FAMILY any_name
  | __empty
;

opclass_purpose :
    FOR SEARCH
  | FOR ORDER BY any_name
  | __empty
;

CreateOpFamilyStmt :
    CREATE OPERATOR FAMILY any_name USING ColId
;

AlterOpFamilyStmt :
    ALTER OPERATOR FAMILY any_name USING ColId ADD_P opclass_item_list
  | ALTER OPERATOR FAMILY any_name USING ColId DROP opclass_drop_list
;

opclass_drop_list :
    opclass_drop opclass_drop_list_1
  | opclass_drop
;

opclass_drop_list_1 :
    ',' opclass_drop opclass_drop_list_1
  | ',' opclass_drop
;

opclass_drop :
    OPERATOR ICONST '(' type_list ')'
  | FUNCTION ICONST '(' type_list ')'
;

DropOpClassStmt :
    DROP OPERATOR CLASS any_name USING ColId opt_drop_behavior
  | DROP OPERATOR CLASS IF_P EXISTS any_name USING ColId opt_drop_behavior
;

DropOpFamilyStmt :
    DROP OPERATOR FAMILY any_name USING ColId opt_drop_behavior
  | DROP OPERATOR FAMILY IF_P EXISTS any_name USING ColId opt_drop_behavior
;

DropOwnedStmt :
    DROP OWNED BY role_list opt_drop_behavior
;

ReassignOwnedStmt :
    REASSIGN OWNED BY role_list TO RoleSpec
;

DropStmt :
    DROP object_type_any_name IF_P EXISTS any_name_list opt_drop_behavior
  | DROP object_type_any_name any_name_list opt_drop_behavior
  | DROP drop_type_name IF_P EXISTS name_list opt_drop_behavior
  | DROP drop_type_name name_list opt_drop_behavior
  | DROP object_type_name_on_any_name ColId ON any_name opt_drop_behavior
  | DROP object_type_name_on_any_name IF_P EXISTS ColId ON any_name opt_drop_behavior
  | DROP TYPE_P type_name_list opt_drop_behavior
  | DROP TYPE_P IF_P EXISTS type_name_list opt_drop_behavior
  | DROP DOMAIN_P type_name_list opt_drop_behavior
  | DROP DOMAIN_P IF_P EXISTS type_name_list opt_drop_behavior
  | DROP INDEX CONCURRENTLY any_name_list opt_drop_behavior
  | DROP INDEX CONCURRENTLY IF_P EXISTS any_name_list opt_drop_behavior
;

object_type_any_name :
    TABLE
  | SEQUENCE
  | VIEW
  | MATERIALIZED VIEW
  | INDEX
  | FOREIGN TABLE
  | COLLATION
  | CONVERSION_P
  | STATISTICS
  | TEXT_P SEARCH PARSER
  | TEXT_P SEARCH DICTIONARY
  | TEXT_P SEARCH TEMPLATE
  | TEXT_P SEARCH CONFIGURATION
;

object_type_name :
    drop_type_name
  | DATABASE
  | ROLE
  | SUBSCRIPTION
  | TABLESPACE
;

drop_type_name :
    ACCESS METHOD
  | EVENT TRIGGER
  | EXTENSION
  | FOREIGN DATA_P WRAPPER
  | opt_procedural LANGUAGE
  | PUBLICATION
  | SCHEMA
  | SERVER
;

object_type_name_on_any_name :
    POLICY
  | RULE
  | TRIGGER
;

any_name_list :
    any_name any_name_list_1
  | any_name
;

any_name_list_1 :
    ',' any_name any_name_list_1
  | ',' any_name
;

any_name :
    ColId
  | ColId attrs
;

attrs :
    '.' ColLabel attrs
  | '.' ColLabel
;

type_name_list :
    Typename type_name_list_1
  | Typename
;

type_name_list_1 :
    ',' Typename type_name_list_1
  | ',' Typename
;

TruncateStmt :
    TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
;

opt_restart_seqs :
    CONTINUE_P IDENTITY_P
  | RESTART IDENTITY_P
  | __empty
;

CommentStmt :
    COMMENT ON object_type_any_name any_name IS comment_text
  | COMMENT ON COLUMN any_name IS comment_text
  | COMMENT ON object_type_name ColId IS comment_text
  | COMMENT ON TYPE_P Typename IS comment_text
  | COMMENT ON DOMAIN_P Typename IS comment_text
  | COMMENT ON AGGREGATE aggregate_with_argtypes IS comment_text
  | COMMENT ON FUNCTION function_with_argtypes IS comment_text
  | COMMENT ON OPERATOR operator_with_argtypes IS comment_text
  | COMMENT ON CONSTRAINT ColId ON any_name IS comment_text
  | COMMENT ON CONSTRAINT ColId ON DOMAIN_P any_name IS comment_text
  | COMMENT ON object_type_name_on_any_name ColId ON any_name IS comment_text
  | COMMENT ON PROCEDURE function_with_argtypes IS comment_text
  | COMMENT ON ROUTINE function_with_argtypes IS comment_text
  | COMMENT ON TRANSFORM FOR Typename LANGUAGE ColId IS comment_text
  | COMMENT ON OPERATOR CLASS any_name USING ColId IS comment_text
  | COMMENT ON OPERATOR FAMILY any_name USING ColId IS comment_text
  | COMMENT ON LARGE_P OBJECT_P NumericOnly IS comment_text
  | COMMENT ON CAST '(' Typename AS Typename ')' IS comment_text
;

comment_text :
    SCONST
  | NULL_P
;

SecLabelStmt :
    SECURITY LABEL opt_provider ON object_type_any_name any_name IS security_label
  | SECURITY LABEL opt_provider ON COLUMN any_name IS security_label
  | SECURITY LABEL opt_provider ON object_type_name ColId IS security_label
  | SECURITY LABEL opt_provider ON TYPE_P Typename IS security_label
  | SECURITY LABEL opt_provider ON DOMAIN_P Typename IS security_label
  | SECURITY LABEL opt_provider ON AGGREGATE aggregate_with_argtypes IS security_label
  | SECURITY LABEL opt_provider ON FUNCTION function_with_argtypes IS security_label
  | SECURITY LABEL opt_provider ON LARGE_P OBJECT_P NumericOnly IS security_label
  | SECURITY LABEL opt_provider ON PROCEDURE function_with_argtypes IS security_label
  | SECURITY LABEL opt_provider ON ROUTINE function_with_argtypes IS security_label
;

opt_provider :
    FOR NonReservedWord_or_Sconst
  | __empty
;

security_label :
    SCONST
  | NULL_P
;

FetchStmt :
    FETCH fetch_args
  | MOVE fetch_args
;

fetch_args :
    ColId
  | from_in ColId
  | NEXT opt_from_in ColId
  | PRIOR opt_from_in ColId
  | FIRST_P opt_from_in ColId
  | LAST_P opt_from_in ColId
  | ABSOLUTE_P SignedIconst opt_from_in ColId
  | RELATIVE_P SignedIconst opt_from_in ColId
  | SignedIconst opt_from_in ColId
  | ALL opt_from_in ColId
  | FORWARD opt_from_in ColId
  | FORWARD SignedIconst opt_from_in ColId
  | FORWARD ALL opt_from_in ColId
  | BACKWARD opt_from_in ColId
  | BACKWARD SignedIconst opt_from_in ColId
  | BACKWARD ALL opt_from_in ColId
;

from_in :
    FROM
  | IN_P
;

opt_from_in :
    from_in
  | __empty
;

GrantStmt :
    GRANT privileges ON privilege_target TO grantee_list opt_grant_grant_option opt_granted_by
;

RevokeStmt :
    REVOKE privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
  | REVOKE GRANT OPTION FOR privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
;

privileges :
    privilege_list
  | ALL
  | ALL PRIVILEGES
  | ALL '(' columnList ')'
  | ALL PRIVILEGES '(' columnList ')'
;

privilege_list :
    privilege privilege_list_1
  | privilege
;

privilege_list_1 :
    ',' privilege privilege_list_1
  | ',' privilege
;

privilege :
    SELECT opt_column_list
  | REFERENCES opt_column_list
  | CREATE opt_column_list
  | ALTER SYSTEM_P
  | ColId opt_column_list
;

parameter_name_list :
    parameter_name parameter_name_list_1
  | parameter_name
;

parameter_name_list_1 :
    ',' parameter_name parameter_name_list_1
  | ',' parameter_name
;

parameter_name :
    ColId parameter_name_1
  | ColId
;

parameter_name_1 :
    '.' ColId parameter_name_1
  | '.' ColId
;

privilege_target :
    qualified_name_list
  | TABLE qualified_name_list
  | SEQUENCE qualified_name_list
  | FOREIGN DATA_P WRAPPER name_list
  | FOREIGN SERVER name_list
  | FUNCTION function_with_argtypes_list
  | PROCEDURE function_with_argtypes_list
  | ROUTINE function_with_argtypes_list
  | DATABASE name_list
  | DOMAIN_P any_name_list
  | LANGUAGE name_list
  | LARGE_P OBJECT_P NumericOnly_list
  | PARAMETER parameter_name_list
  | SCHEMA name_list
  | TABLESPACE name_list
  | TYPE_P any_name_list
  | ALL TABLES IN_P SCHEMA name_list
  | ALL SEQUENCES IN_P SCHEMA name_list
  | ALL FUNCTIONS IN_P SCHEMA name_list
  | ALL PROCEDURES IN_P SCHEMA name_list
  | ALL ROUTINES IN_P SCHEMA name_list
;

grantee_list :
    grantee grantee_list_1
  | grantee
;

grantee_list_1 :
    ',' grantee grantee_list_1
  | ',' grantee
;

grantee :
    RoleSpec
  | GROUP_P RoleSpec
;

opt_grant_grant_option :
    WITH GRANT OPTION
  | __empty
;

GrantRoleStmt :
    GRANT privilege_list TO role_list opt_granted_by
  | GRANT privilege_list TO role_list WITH grant_role_opt_list opt_granted_by
;

RevokeRoleStmt :
    REVOKE privilege_list FROM role_list opt_granted_by opt_drop_behavior
  | REVOKE ColId OPTION FOR privilege_list FROM role_list opt_granted_by opt_drop_behavior
;

grant_role_opt_list :
    grant_role_opt grant_role_opt_list_1
  | grant_role_opt
;

grant_role_opt_list_1 :
    ',' grant_role_opt grant_role_opt_list_1
  | ',' grant_role_opt
;

grant_role_opt :
    ColLabel grant_role_opt_value
;

grant_role_opt_value :
    OPTION
  | TRUE_P
  | FALSE_P
;

opt_granted_by :
    GRANTED BY RoleSpec
  | __empty
;

AlterDefaultPrivilegesStmt :
    ALTER DEFAULT PRIVILEGES DefACLOptionList DefACLAction
;

DefACLOptionList :
    DefACLOption DefACLOptionList
  | __empty
;

DefACLOption :
    IN_P SCHEMA name_list
  | FOR ROLE role_list
  | FOR USER role_list
;

DefACLAction :
    GRANT privileges ON defacl_privilege_target TO grantee_list opt_grant_grant_option
  | REVOKE privileges ON defacl_privilege_target FROM grantee_list opt_drop_behavior
  | REVOKE GRANT OPTION FOR privileges ON defacl_privilege_target FROM grantee_list opt_drop_behavior
;

defacl_privilege_target :
    TABLES
  | FUNCTIONS
  | ROUTINES
  | SEQUENCES
  | TYPES_P
  | SCHEMAS
;

IndexStmt :
    CREATE opt_unique INDEX opt_concurrently opt_single_name ON relation_expr access_method_clause '(' index_params ')' opt_include opt_unique_null_treatment opt_reloptions OptTableSpace where_clause
  | CREATE opt_unique INDEX opt_concurrently IF_P NOT EXISTS ColId ON relation_expr access_method_clause '(' index_params ')' opt_include opt_unique_null_treatment opt_reloptions OptTableSpace where_clause
;

opt_unique :
    UNIQUE
  | __empty
;

access_method_clause :
    USING ColId
  | __empty
;

index_params :
    index_elem index_params_1
  | index_elem
;

index_params_1 :
    ',' index_elem index_params_1
  | ',' index_elem
;

index_elem_options :
    opt_collate opt_qualified_name opt_asc_desc opt_nulls_order
  | opt_collate any_name reloptions opt_asc_desc opt_nulls_order
;

index_elem :
    ColId index_elem_options
  | func_expr_windowless index_elem_options
  | '(' a_expr ')' index_elem_options
;

opt_include :
    INCLUDE '(' index_including_params ')'
  | __empty
;

index_including_params :
    index_elem index_including_params_1
  | index_elem
;

index_including_params_1 :
    ',' index_elem index_including_params_1
  | ',' index_elem
;

opt_collate :
    COLLATE any_name
  | __empty
;

opt_asc_desc :
    ASC
  | DESC
  | __empty
;

opt_nulls_order :
    NULLS_LA FIRST_P
  | NULLS_LA LAST_P
  | __empty
;

CreateFunctionStmt :
    CREATE opt_or_replace FUNCTION func_name func_args_with_defaults RETURNS func_type opt_createfunc_opt_list opt_routine_body
  | CREATE opt_or_replace FUNCTION func_name func_args_with_defaults RETURNS TABLE '(' table_func_column_list ')' opt_createfunc_opt_list opt_routine_body
  | CREATE opt_or_replace FUNCTION func_name func_args_with_defaults opt_createfunc_opt_list opt_routine_body
  | CREATE opt_or_replace PROCEDURE func_name func_args_with_defaults opt_createfunc_opt_list opt_routine_body
;

opt_or_replace :
    OR REPLACE
  | __empty
;

func_args :
    '(' func_args_list ')'
  | '(' ')'
;

func_args_list :
    func_arg func_args_list_1
  | func_arg
;

func_args_list_1 :
    ',' func_arg func_args_list_1
  | ',' func_arg
;

function_with_argtypes_list :
    function_with_argtypes function_with_argtypes_list_1
  | function_with_argtypes
;

function_with_argtypes_list_1 :
    ',' function_with_argtypes function_with_argtypes_list_1
  | ',' function_with_argtypes
;

function_with_argtypes :
    func_name func_args
  | type_func_name_keyword
  | ColId
  | ColId indirection
;

func_args_with_defaults :
    '(' func_args_with_defaults_list ')'
  | '(' ')'
;

func_args_with_defaults_list :
    func_arg_with_default func_args_with_defaults_list_1
  | func_arg_with_default
;

func_args_with_defaults_list_1 :
    ',' func_arg_with_default func_args_with_defaults_list_1
  | ',' func_arg_with_default
;

func_arg :
    arg_class type_function_name func_type
  | type_function_name arg_class func_type
  | type_function_name func_type
  | arg_class func_type
  | func_type
;

arg_class :
    IN_P
  | OUT_P
  | INOUT
  | IN_P OUT_P
  | VARIADIC
;

func_type :
    Typename
  | type_function_name attrs '%' TYPE_P
  | SETOF type_function_name attrs '%' TYPE_P
;

func_arg_with_default :
    func_arg
  | func_arg DEFAULT a_expr
  | func_arg '=' a_expr
;

aggr_arg :
    func_arg
;

aggr_args :
    '(' '*' ')'
  | '(' aggr_args_list ')'
  | '(' ORDER BY aggr_args_list ')'
  | '(' aggr_args_list ORDER BY aggr_args_list ')'
;

aggr_args_list :
    aggr_arg aggr_args_list_1
  | aggr_arg
;

aggr_args_list_1 :
    ',' aggr_arg aggr_args_list_1
  | ',' aggr_arg
;

aggregate_with_argtypes :
    func_name aggr_args
;

aggregate_with_argtypes_list :
    aggregate_with_argtypes aggregate_with_argtypes_list_1
  | aggregate_with_argtypes
;

aggregate_with_argtypes_list_1 :
    ',' aggregate_with_argtypes aggregate_with_argtypes_list_1
  | ',' aggregate_with_argtypes
;

opt_createfunc_opt_list :
    createfunc_opt_list
  | __empty
;

createfunc_opt_list :
    createfunc_opt_item createfunc_opt_list
  | createfunc_opt_item
;

common_func_opt_item :
    CALLED ON NULL_P INPUT_P
  | RETURNS NULL_P ON NULL_P INPUT_P
  | STRICT_P
  | IMMUTABLE
  | STABLE
  | VOLATILE
  | EXTERNAL SECURITY DEFINER
  | EXTERNAL SECURITY INVOKER
  | SECURITY DEFINER
  | SECURITY INVOKER
  | LEAKPROOF
  | NOT LEAKPROOF
  | COST NumericOnly
  | ROWS NumericOnly
  | SUPPORT any_name
  | FunctionSetResetClause
  | PARALLEL ColId
;

createfunc_opt_item :
    AS func_as
  | LANGUAGE NonReservedWord_or_Sconst
  | TRANSFORM transform_type_list
  | WINDOW
  | common_func_opt_item
;

func_as :
    SCONST
  | SCONST ',' SCONST
;

ReturnStmt :
    RETURN a_expr
;

opt_routine_body :
    ReturnStmt
  | BEGIN_P ATOMIC routine_body_stmt_list END_P
  | __empty
;

routine_body_stmt_list :
    routine_body_stmt ';' routine_body_stmt_list
  | __empty
;

routine_body_stmt :
    stmt
  | ReturnStmt
;

transform_type_list :
    FOR TYPE_P Typename transform_type_list_1
  | FOR TYPE_P Typename
;

transform_type_list_1 :
    ',' FOR TYPE_P Typename transform_type_list_1
  | ',' FOR TYPE_P Typename
;

opt_definition :
    WITH definition
  | __empty
;

table_func_column :
    type_function_name func_type
;

table_func_column_list :
    table_func_column table_func_column_list_1
  | table_func_column
;

table_func_column_list_1 :
    ',' table_func_column table_func_column_list_1
  | ',' table_func_column
;

AlterFunctionStmt :
    ALTER FUNCTION function_with_argtypes alterfunc_opt_list opt_restrict
  | ALTER PROCEDURE function_with_argtypes alterfunc_opt_list opt_restrict
  | ALTER ROUTINE function_with_argtypes alterfunc_opt_list opt_restrict
;

alterfunc_opt_list :
    common_func_opt_item alterfunc_opt_list
  | common_func_opt_item
;

opt_restrict :
    RESTRICT
  | __empty
;

RemoveFuncStmt :
    DROP FUNCTION function_with_argtypes_list opt_drop_behavior
  | DROP FUNCTION IF_P EXISTS function_with_argtypes_list opt_drop_behavior
  | DROP PROCEDURE function_with_argtypes_list opt_drop_behavior
  | DROP PROCEDURE IF_P EXISTS function_with_argtypes_list opt_drop_behavior
  | DROP ROUTINE function_with_argtypes_list opt_drop_behavior
  | DROP ROUTINE IF_P EXISTS function_with_argtypes_list opt_drop_behavior
;

RemoveAggrStmt :
    DROP AGGREGATE aggregate_with_argtypes_list opt_drop_behavior
  | DROP AGGREGATE IF_P EXISTS aggregate_with_argtypes_list opt_drop_behavior
;

RemoveOperStmt :
    DROP OPERATOR operator_with_argtypes_list opt_drop_behavior
  | DROP OPERATOR IF_P EXISTS operator_with_argtypes_list opt_drop_behavior
;

oper_argtypes :
    '(' Typename ')'
  | '(' Typename ',' Typename ')'
  | '(' NONE ',' Typename ')'
  | '(' Typename ',' NONE ')'
;

any_operator :
    all_Op
  | ColId '.' any_operator
;

operator_with_argtypes_list :
    operator_with_argtypes operator_with_argtypes_list_1
  | operator_with_argtypes
;

operator_with_argtypes_list_1 :
    ',' operator_with_argtypes operator_with_argtypes_list_1
  | ',' operator_with_argtypes
;

operator_with_argtypes :
    any_operator oper_argtypes
;

DoStmt :
    DO dostmt_opt_list
;

dostmt_opt_list :
    dostmt_opt_item dostmt_opt_list
  | dostmt_opt_item
;

dostmt_opt_item :
    SCONST
  | LANGUAGE NonReservedWord_or_Sconst
;

CreateCastStmt :
    CREATE CAST '(' Typename AS Typename ')' WITH FUNCTION function_with_argtypes cast_context
  | CREATE CAST '(' Typename AS Typename ')' WITHOUT FUNCTION cast_context
  | CREATE CAST '(' Typename AS Typename ')' WITH INOUT cast_context
;

cast_context :
    AS IMPLICIT_P
  | AS ASSIGNMENT
  | __empty
;

DropCastStmt :
    DROP CAST opt_if_exists '(' Typename AS Typename ')' opt_drop_behavior
;

opt_if_exists :
    IF_P EXISTS
  | __empty
;

CreateTransformStmt :
    CREATE opt_or_replace TRANSFORM FOR Typename LANGUAGE ColId '(' transform_element_list ')'
;

transform_element_list :
    FROM SQL_P WITH FUNCTION function_with_argtypes ',' TO SQL_P WITH FUNCTION function_with_argtypes
  | TO SQL_P WITH FUNCTION function_with_argtypes ',' FROM SQL_P WITH FUNCTION function_with_argtypes
  | FROM SQL_P WITH FUNCTION function_with_argtypes
  | TO SQL_P WITH FUNCTION function_with_argtypes
;

DropTransformStmt :
    DROP TRANSFORM opt_if_exists FOR Typename LANGUAGE ColId opt_drop_behavior
;

ReindexStmt :
    REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
  | REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
  | REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
;

reindex_target_relation :
    INDEX
  | TABLE
;

reindex_target_all :
    SYSTEM_P
  | DATABASE
;

opt_reindex_option_list :
    '(' utility_option_list ')'
  | __empty
;

AlterTblSpcStmt :
    ALTER TABLESPACE ColId SET reloptions
  | ALTER TABLESPACE ColId RESET reloptions
;

RenameStmt :
    ALTER AGGREGATE aggregate_with_argtypes RENAME TO ColId
  | ALTER COLLATION any_name RENAME TO ColId
  | ALTER CONVERSION_P any_name RENAME TO ColId
  | ALTER DATABASE ColId RENAME TO ColId
  | ALTER DOMAIN_P any_name RENAME TO ColId
  | ALTER DOMAIN_P any_name RENAME CONSTRAINT ColId TO ColId
  | ALTER FOREIGN DATA_P WRAPPER ColId RENAME TO ColId
  | ALTER FUNCTION function_with_argtypes RENAME TO ColId
  | ALTER GROUP_P RoleId RENAME TO RoleId
  | ALTER opt_procedural LANGUAGE ColId RENAME TO ColId
  | ALTER OPERATOR CLASS any_name USING ColId RENAME TO ColId
  | ALTER OPERATOR FAMILY any_name USING ColId RENAME TO ColId
  | ALTER POLICY ColId ON qualified_name RENAME TO ColId
  | ALTER POLICY IF_P EXISTS ColId ON qualified_name RENAME TO ColId
  | ALTER PROCEDURE function_with_argtypes RENAME TO ColId
  | ALTER PUBLICATION ColId RENAME TO ColId
  | ALTER ROUTINE function_with_argtypes RENAME TO ColId
  | ALTER SCHEMA ColId RENAME TO ColId
  | ALTER SERVER ColId RENAME TO ColId
  | ALTER SUBSCRIPTION ColId RENAME TO ColId
  | ALTER TABLE relation_expr RENAME TO ColId
  | ALTER TABLE IF_P EXISTS relation_expr RENAME TO ColId
  | ALTER SEQUENCE qualified_name RENAME TO ColId
  | ALTER SEQUENCE IF_P EXISTS qualified_name RENAME TO ColId
  | ALTER VIEW qualified_name RENAME TO ColId
  | ALTER VIEW IF_P EXISTS qualified_name RENAME TO ColId
  | ALTER MATERIALIZED VIEW qualified_name RENAME TO ColId
  | ALTER MATERIALIZED VIEW IF_P EXISTS qualified_name RENAME TO ColId
  | ALTER INDEX qualified_name RENAME TO ColId
  | ALTER INDEX IF_P EXISTS qualified_name RENAME TO ColId
  | ALTER FOREIGN TABLE relation_expr RENAME TO ColId
  | ALTER FOREIGN TABLE IF_P EXISTS relation_expr RENAME TO ColId
  | ALTER TABLE relation_expr RENAME opt_column ColId TO ColId
  | ALTER TABLE IF_P EXISTS relation_expr RENAME opt_column ColId TO ColId
  | ALTER VIEW qualified_name RENAME opt_column ColId TO ColId
  | ALTER VIEW IF_P EXISTS qualified_name RENAME opt_column ColId TO ColId
  | ALTER MATERIALIZED VIEW qualified_name RENAME opt_column ColId TO ColId
  | ALTER MATERIALIZED VIEW IF_P EXISTS qualified_name RENAME opt_column ColId TO ColId
  | ALTER TABLE relation_expr RENAME CONSTRAINT ColId TO ColId
  | ALTER TABLE IF_P EXISTS relation_expr RENAME CONSTRAINT ColId TO ColId
  | ALTER FOREIGN TABLE relation_expr RENAME opt_column ColId TO ColId
  | ALTER FOREIGN TABLE IF_P EXISTS relation_expr RENAME opt_column ColId TO ColId
  | ALTER RULE ColId ON qualified_name RENAME TO ColId
  | ALTER TRIGGER ColId ON qualified_name RENAME TO ColId
  | ALTER EVENT TRIGGER ColId RENAME TO ColId
  | ALTER ROLE RoleId RENAME TO RoleId
  | ALTER USER RoleId RENAME TO RoleId
  | ALTER TABLESPACE ColId RENAME TO ColId
  | ALTER STATISTICS any_name RENAME TO ColId
  | ALTER TEXT_P SEARCH PARSER any_name RENAME TO ColId
  | ALTER TEXT_P SEARCH DICTIONARY any_name RENAME TO ColId
  | ALTER TEXT_P SEARCH TEMPLATE any_name RENAME TO ColId
  | ALTER TEXT_P SEARCH CONFIGURATION any_name RENAME TO ColId
  | ALTER TYPE_P any_name RENAME TO ColId
  | ALTER TYPE_P any_name RENAME ATTRIBUTE ColId TO ColId opt_drop_behavior
;

opt_column :
    COLUMN
  | __empty
;

opt_set_data :
    SET DATA_P
  | __empty
;

AlterObjectDependsStmt :
    ALTER FUNCTION function_with_argtypes opt_no DEPENDS ON EXTENSION ColId
  | ALTER PROCEDURE function_with_argtypes opt_no DEPENDS ON EXTENSION ColId
  | ALTER ROUTINE function_with_argtypes opt_no DEPENDS ON EXTENSION ColId
  | ALTER TRIGGER ColId ON qualified_name opt_no DEPENDS ON EXTENSION ColId
  | ALTER MATERIALIZED VIEW qualified_name opt_no DEPENDS ON EXTENSION ColId
  | ALTER INDEX qualified_name opt_no DEPENDS ON EXTENSION ColId
;

opt_no :
    NO
  | __empty
;

AlterObjectSchemaStmt :
    ALTER AGGREGATE aggregate_with_argtypes SET SCHEMA ColId
  | ALTER COLLATION any_name SET SCHEMA ColId
  | ALTER CONVERSION_P any_name SET SCHEMA ColId
  | ALTER DOMAIN_P any_name SET SCHEMA ColId
  | ALTER EXTENSION ColId SET SCHEMA ColId
  | ALTER FUNCTION function_with_argtypes SET SCHEMA ColId
  | ALTER OPERATOR operator_with_argtypes SET SCHEMA ColId
  | ALTER OPERATOR CLASS any_name USING ColId SET SCHEMA ColId
  | ALTER OPERATOR FAMILY any_name USING ColId SET SCHEMA ColId
  | ALTER PROCEDURE function_with_argtypes SET SCHEMA ColId
  | ALTER ROUTINE function_with_argtypes SET SCHEMA ColId
  | ALTER TABLE relation_expr SET SCHEMA ColId
  | ALTER TABLE IF_P EXISTS relation_expr SET SCHEMA ColId
  | ALTER STATISTICS any_name SET SCHEMA ColId
  | ALTER TEXT_P SEARCH PARSER any_name SET SCHEMA ColId
  | ALTER TEXT_P SEARCH DICTIONARY any_name SET SCHEMA ColId
  | ALTER TEXT_P SEARCH TEMPLATE any_name SET SCHEMA ColId
  | ALTER TEXT_P SEARCH CONFIGURATION any_name SET SCHEMA ColId
  | ALTER SEQUENCE qualified_name SET SCHEMA ColId
  | ALTER SEQUENCE IF_P EXISTS qualified_name SET SCHEMA ColId
  | ALTER VIEW qualified_name SET SCHEMA ColId
  | ALTER VIEW IF_P EXISTS qualified_name SET SCHEMA ColId
  | ALTER MATERIALIZED VIEW qualified_name SET SCHEMA ColId
  | ALTER MATERIALIZED VIEW IF_P EXISTS qualified_name SET SCHEMA ColId
  | ALTER FOREIGN TABLE relation_expr SET SCHEMA ColId
  | ALTER FOREIGN TABLE IF_P EXISTS relation_expr SET SCHEMA ColId
  | ALTER TYPE_P any_name SET SCHEMA ColId
;

AlterOperatorStmt :
    ALTER OPERATOR operator_with_argtypes SET '(' operator_def_list ')'
;

operator_def_list :
    operator_def_elem operator_def_list_1
  | operator_def_elem
;

operator_def_list_1 :
    ',' operator_def_elem operator_def_list_1
  | ',' operator_def_elem
;

operator_def_elem :
    ColLabel '=' NONE
  | ColLabel '=' operator_def_arg
  | ColLabel
;

operator_def_arg :
    func_type
  | reserved_keyword
  | qual_all_Op
  | NumericOnly
  | SCONST
;

AlterTypeStmt :
    ALTER TYPE_P any_name SET '(' operator_def_list ')'
;

AlterOwnerStmt :
    ALTER AGGREGATE aggregate_with_argtypes OWNER TO RoleSpec
  | ALTER COLLATION any_name OWNER TO RoleSpec
  | ALTER CONVERSION_P any_name OWNER TO RoleSpec
  | ALTER DATABASE ColId OWNER TO RoleSpec
  | ALTER DOMAIN_P any_name OWNER TO RoleSpec
  | ALTER FUNCTION function_with_argtypes OWNER TO RoleSpec
  | ALTER opt_procedural LANGUAGE ColId OWNER TO RoleSpec
  | ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
  | ALTER OPERATOR operator_with_argtypes OWNER TO RoleSpec
  | ALTER OPERATOR CLASS any_name USING ColId OWNER TO RoleSpec
  | ALTER OPERATOR FAMILY any_name USING ColId OWNER TO RoleSpec
  | ALTER PROCEDURE function_with_argtypes OWNER TO RoleSpec
  | ALTER ROUTINE function_with_argtypes OWNER TO RoleSpec
  | ALTER SCHEMA ColId OWNER TO RoleSpec
  | ALTER TYPE_P any_name OWNER TO RoleSpec
  | ALTER TABLESPACE ColId OWNER TO RoleSpec
  | ALTER STATISTICS any_name OWNER TO RoleSpec
  | ALTER TEXT_P SEARCH DICTIONARY any_name OWNER TO RoleSpec
  | ALTER TEXT_P SEARCH CONFIGURATION any_name OWNER TO RoleSpec
  | ALTER FOREIGN DATA_P WRAPPER ColId OWNER TO RoleSpec
  | ALTER SERVER ColId OWNER TO RoleSpec
  | ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
  | ALTER PUBLICATION ColId OWNER TO RoleSpec
  | ALTER SUBSCRIPTION ColId OWNER TO RoleSpec
;

CreatePublicationStmt :
    CREATE PUBLICATION ColId opt_definition
  | CREATE PUBLICATION ColId FOR ALL TABLES opt_definition
  | CREATE PUBLICATION ColId FOR pub_obj_list opt_definition
;

PublicationObjSpec :
    TABLE relation_expr opt_column_list OptWhereClause
  | TABLES IN_P SCHEMA ColId
  | TABLES IN_P SCHEMA CURRENT_SCHEMA
  | ColId opt_column_list OptWhereClause
  | ColId indirection opt_column_list OptWhereClause
  | extended_relation_expr opt_column_list OptWhereClause
  | CURRENT_SCHEMA
;

pub_obj_list :
    PublicationObjSpec pub_obj_list_1
  | PublicationObjSpec
;

pub_obj_list_1 :
    ',' PublicationObjSpec pub_obj_list_1
  | ',' PublicationObjSpec
;

AlterPublicationStmt :
    ALTER PUBLICATION ColId SET definition
  | ALTER PUBLICATION ColId ADD_P pub_obj_list
  | ALTER PUBLICATION ColId SET pub_obj_list
  | ALTER PUBLICATION ColId DROP pub_obj_list
;

CreateSubscriptionStmt :
    CREATE SUBSCRIPTION ColId CONNECTION SCONST PUBLICATION name_list opt_definition
;

AlterSubscriptionStmt :
    ALTER SUBSCRIPTION ColId SET definition
  | ALTER SUBSCRIPTION ColId CONNECTION SCONST
  | ALTER SUBSCRIPTION ColId REFRESH PUBLICATION opt_definition
  | ALTER SUBSCRIPTION ColId ADD_P PUBLICATION name_list opt_definition
  | ALTER SUBSCRIPTION ColId DROP PUBLICATION name_list opt_definition
  | ALTER SUBSCRIPTION ColId SET PUBLICATION name_list opt_definition
  | ALTER SUBSCRIPTION ColId ENABLE_P
  | ALTER SUBSCRIPTION ColId DISABLE_P
  | ALTER SUBSCRIPTION ColId SKIP definition
;

DropSubscriptionStmt :
    DROP SUBSCRIPTION ColId opt_drop_behavior
  | DROP SUBSCRIPTION IF_P EXISTS ColId opt_drop_behavior
;

RuleStmt :
    CREATE opt_or_replace RULE ColId AS ON event TO qualified_name where_clause DO opt_instead RuleActionList
;

RuleActionList :
    NOTHING
  | RuleActionStmt
  | '(' RuleActionMulti ')'
;

RuleActionMulti :
    RuleActionStmtOrEmpty RuleActionMulti_1
  | RuleActionStmtOrEmpty
;

RuleActionMulti_1 :
    ';' RuleActionStmtOrEmpty RuleActionMulti_1
  | ';' RuleActionStmtOrEmpty
;

RuleActionStmt :
    SelectStmt
  | InsertStmt
  | UpdateStmt
  | DeleteStmt
  | NotifyStmt
;

RuleActionStmtOrEmpty :
    RuleActionStmt
  | __empty
;

event :
    SELECT
  | UPDATE
  | DELETE_P
  | INSERT
;

opt_instead :
    INSTEAD
  | ALSO
  | __empty
;

NotifyStmt :
    NOTIFY ColId notify_payload
;

notify_payload :
    ',' SCONST
  | __empty
;

ListenStmt :
    LISTEN ColId
;

UnlistenStmt :
    UNLISTEN ColId
  | UNLISTEN '*'
;

TransactionStmt :
    ABORT_P opt_transaction opt_transaction_chain
  | START TRANSACTION transaction_mode_list_or_empty
  | COMMIT opt_transaction opt_transaction_chain
  | ROLLBACK opt_transaction opt_transaction_chain
  | SAVEPOINT ColId
  | RELEASE SAVEPOINT ColId
  | RELEASE ColId
  | ROLLBACK opt_transaction TO SAVEPOINT ColId
  | ROLLBACK opt_transaction TO ColId
  | PREPARE TRANSACTION SCONST
  | COMMIT PREPARED SCONST
  | ROLLBACK PREPARED SCONST
;

TransactionStmtLegacy :
    BEGIN_P opt_transaction transaction_mode_list_or_empty
  | END_P opt_transaction opt_transaction_chain
;

opt_transaction :
    WORK
  | TRANSACTION
  | __empty
;

transaction_mode_item :
    ISOLATION LEVEL iso_level
  | READ ONLY
  | READ WRITE
  | DEFERRABLE
  | NOT DEFERRABLE
;

transaction_mode_list :
    transaction_mode_item transaction_mode_list_2
  | transaction_mode_item
;

transaction_mode_list_1 :
    ',' transaction_mode_item
  | transaction_mode_item
;

transaction_mode_list_2 :
    transaction_mode_list_1 transaction_mode_list_2
  | transaction_mode_list_1
;

transaction_mode_list_or_empty :
    transaction_mode_list
  | __empty
;

opt_transaction_chain :
    AND CHAIN
  | AND NO CHAIN
  | __empty
;

ViewStmt :
    CREATE OptTemp VIEW qualified_name opt_column_list opt_reloptions AS SelectStmt opt_check_option
  | CREATE OR REPLACE OptTemp VIEW qualified_name opt_column_list opt_reloptions AS SelectStmt opt_check_option
  | CREATE OptTemp RECURSIVE VIEW qualified_name '(' columnList ')' opt_reloptions AS SelectStmt opt_check_option
  | CREATE OR REPLACE OptTemp RECURSIVE VIEW qualified_name '(' columnList ')' opt_reloptions AS SelectStmt opt_check_option
;

opt_check_option :
    WITH CHECK OPTION
  | WITH CASCADED CHECK OPTION
  | WITH LOCAL CHECK OPTION
  | __empty
;

LoadStmt :
    LOAD SCONST
;

CreatedbStmt :
    CREATE DATABASE ColId opt_with createdb_opt_list
;

createdb_opt_list :
    createdb_opt_items
  | __empty
;

createdb_opt_items :
    createdb_opt_item createdb_opt_items
  | createdb_opt_item
;

createdb_opt_item :
    createdb_opt_name opt_equal NumericOnly
  | createdb_opt_name opt_equal opt_boolean_or_string
  | createdb_opt_name opt_equal DEFAULT
;

createdb_opt_name :
    IDENT
  | CONNECTION LIMIT
  | ENCODING
  | LOCATION
  | OWNER
  | TABLESPACE
  | TEMPLATE
;

opt_equal :
    '='
  | __empty
;

AlterDatabaseStmt :
    ALTER DATABASE ColId WITH createdb_opt_list
  | ALTER DATABASE ColId createdb_opt_list
  | ALTER DATABASE ColId SET TABLESPACE ColId
  | ALTER DATABASE ColId REFRESH COLLATION VERSION_P
;

AlterDatabaseSetStmt :
    ALTER DATABASE ColId SetResetClause
;

DropdbStmt :
    DROP DATABASE ColId
  | DROP DATABASE IF_P EXISTS ColId
  | DROP DATABASE ColId opt_with '(' drop_option_list ')'
  | DROP DATABASE IF_P EXISTS ColId opt_with '(' drop_option_list ')'
;

drop_option_list :
    FORCE drop_option_list_1
  | FORCE
;

drop_option_list_1 :
    ',' FORCE drop_option_list_1
  | ',' FORCE
;

AlterCollationStmt :
    ALTER COLLATION any_name REFRESH VERSION_P
;

AlterSystemStmt :
    ALTER SYSTEM_P SET generic_set
  | ALTER SYSTEM_P RESET generic_reset
;

CreateDomainStmt :
    CREATE DOMAIN_P any_name opt_as Typename ColQualList
;

AlterDomainStmt :
    ALTER DOMAIN_P any_name alter_column_default
  | ALTER DOMAIN_P any_name DROP NOT NULL_P
  | ALTER DOMAIN_P any_name SET NOT NULL_P
  | ALTER DOMAIN_P any_name ADD_P DomainConstraint
  | ALTER DOMAIN_P any_name DROP CONSTRAINT ColId opt_drop_behavior
  | ALTER DOMAIN_P any_name DROP CONSTRAINT IF_P EXISTS ColId opt_drop_behavior
  | ALTER DOMAIN_P any_name VALIDATE CONSTRAINT ColId
;

opt_as :
    AS
  | __empty
;

AlterTSDictionaryStmt :
    ALTER TEXT_P SEARCH DICTIONARY any_name definition
;

AlterTSConfigurationStmt :
    ALTER TEXT_P SEARCH CONFIGURATION any_name ADD_P MAPPING FOR name_list any_with any_name_list
  | ALTER TEXT_P SEARCH CONFIGURATION any_name ALTER MAPPING FOR name_list any_with any_name_list
  | ALTER TEXT_P SEARCH CONFIGURATION any_name ALTER MAPPING REPLACE any_name any_with any_name
  | ALTER TEXT_P SEARCH CONFIGURATION any_name ALTER MAPPING FOR name_list REPLACE any_name any_with any_name
  | ALTER TEXT_P SEARCH CONFIGURATION any_name DROP MAPPING FOR name_list
  | ALTER TEXT_P SEARCH CONFIGURATION any_name DROP MAPPING IF_P EXISTS FOR name_list
;

any_with :
    WITH
  | WITH_LA
;

CreateConversionStmt :
    CREATE opt_default CONVERSION_P any_name FOR SCONST TO SCONST FROM any_name
;

ClusterStmt :
    CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
  | CLUSTER '(' utility_option_list ')'
  | CLUSTER opt_verbose qualified_name cluster_index_specification
  | CLUSTER opt_verbose
  | CLUSTER opt_verbose ColId ON qualified_name
;

cluster_index_specification :
    USING ColId
  | __empty
;

VacuumStmt :
    VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
  | VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
;

AnalyzeStmt :
    analyze_keyword opt_verbose opt_vacuum_relation_list
  | analyze_keyword '(' utility_option_list ')' opt_vacuum_relation_list
;

utility_option_list :
    utility_option_elem utility_option_list_1
  | utility_option_elem
;

utility_option_list_1 :
    ',' utility_option_elem utility_option_list_1
  | ',' utility_option_elem
;

analyze_keyword :
    ANALYZE
  | ANALYSE
;

utility_option_elem :
    utility_option_name utility_option_arg
;

utility_option_name :
    NonReservedWord
  | analyze_keyword
  | FORMAT_LA
;

utility_option_arg :
    opt_boolean_or_string
  | NumericOnly
  | __empty
;

opt_analyze :
    analyze_keyword
  | __empty
;

opt_verbose :
    VERBOSE
  | __empty
;

opt_full :
    FULL
  | __empty
;

opt_freeze :
    FREEZE
  | __empty
;

opt_name_list :
    '(' name_list ')'
  | __empty
;

vacuum_relation :
    relation_expr opt_name_list
;

vacuum_relation_list :
    vacuum_relation vacuum_relation_list_1
  | vacuum_relation
;

vacuum_relation_list_1 :
    ',' vacuum_relation vacuum_relation_list_1
  | ',' vacuum_relation
;

opt_vacuum_relation_list :
    vacuum_relation_list
  | __empty
;

ExplainStmt :
    EXPLAIN ExplainableStmt
  | EXPLAIN analyze_keyword opt_verbose ExplainableStmt
  | EXPLAIN VERBOSE ExplainableStmt
  | EXPLAIN '(' utility_option_list ')' ExplainableStmt
;

ExplainableStmt :
    SelectStmt
  | InsertStmt
  | UpdateStmt
  | DeleteStmt
  | MergeStmt
  | DeclareCursorStmt
  | CreateAsStmt
  | CreateMatViewStmt
  | RefreshMatViewStmt
  | ExecuteStmt
;

PrepareStmt :
    PREPARE ColId prep_type_clause AS PreparableStmt
;

prep_type_clause :
    '(' type_list ')'
  | __empty
;

PreparableStmt :
    SelectStmt
  | InsertStmt
  | UpdateStmt
  | DeleteStmt
  | MergeStmt
;

ExecuteStmt :
    EXECUTE ColId execute_param_clause
  | CREATE OptTemp TABLE create_as_target AS EXECUTE ColId execute_param_clause opt_with_data
  | CREATE OptTemp TABLE IF_P NOT EXISTS create_as_target AS EXECUTE ColId execute_param_clause opt_with_data
;

execute_param_clause :
    '(' expr_list ')'
  | __empty
;

DeallocateStmt :
    DEALLOCATE ColId
  | DEALLOCATE PREPARE ColId
  | DEALLOCATE ALL
  | DEALLOCATE PREPARE ALL
;

InsertStmt :
    opt_with_clause INSERT INTO insert_target insert_rest opt_on_conflict returning_clause
;

insert_target :
    qualified_name
  | qualified_name AS ColId
;

insert_rest :
    SelectStmt
  | OVERRIDING override_kind VALUE_P SelectStmt
  | '(' insert_column_list ')' SelectStmt
  | '(' insert_column_list ')' OVERRIDING override_kind VALUE_P SelectStmt
  | DEFAULT VALUES
;

override_kind :
    USER
  | SYSTEM_P
;

insert_column_list :
    insert_column_item insert_column_list_1
  | insert_column_item
;

insert_column_list_1 :
    ',' insert_column_item insert_column_list_1
  | ',' insert_column_item
;

insert_column_item :
    ColId opt_indirection
;

opt_on_conflict :
    ON CONFLICT opt_conf_expr DO UPDATE SET set_clause_list where_clause
  | ON CONFLICT opt_conf_expr DO NOTHING
  | __empty
;

opt_conf_expr :
    '(' index_params ')' where_clause
  | ON CONSTRAINT ColId
  | __empty
;

returning_clause :
    RETURNING returning_with_clause target_list
  | __empty
;

returning_with_clause :
    WITH '(' returning_options ')'
  | __empty
;

returning_options :
    returning_option returning_options_1
  | returning_option
;

returning_options_1 :
    ',' returning_option returning_options_1
  | ',' returning_option
;

returning_option :
    returning_option_kind AS ColId
;

returning_option_kind :
    OLD
  | NEW
;

DeleteStmt :
    opt_with_clause DELETE_P FROM relation_expr_opt_alias using_clause where_or_current_clause returning_clause
;

using_clause :
    USING from_list
  | __empty
;

LockStmt :
    LOCK_P opt_table relation_expr_list opt_lock opt_nowait
;

opt_lock :
    IN_P lock_type MODE
  | __empty
;

lock_type :
    ACCESS SHARE
  | ROW SHARE
  | ROW EXCLUSIVE
  | SHARE UPDATE EXCLUSIVE
  | SHARE
  | SHARE ROW EXCLUSIVE
  | EXCLUSIVE
  | ACCESS EXCLUSIVE
;

opt_nowait :
    NOWAIT
  | __empty
;

opt_nowait_or_skip :
    NOWAIT
  | SKIP LOCKED
  | __empty
;

UpdateStmt :
    opt_with_clause UPDATE relation_expr_opt_alias SET set_clause_list from_clause where_or_current_clause returning_clause
;

set_clause_list :
    set_clause set_clause_list_1
  | set_clause
;

set_clause_list_1 :
    ',' set_clause set_clause_list_1
  | ',' set_clause
;

set_clause :
    set_target '=' a_expr
  | '(' set_target_list ')' '=' a_expr
;

set_target :
    ColId opt_indirection
;

set_target_list :
    set_target set_target_list_1
  | set_target
;

set_target_list_1 :
    ',' set_target set_target_list_1
  | ',' set_target
;

MergeStmt :
    opt_with_clause MERGE INTO relation_expr_opt_alias USING table_ref ON a_expr merge_when_list returning_clause
;

merge_when_list :
    merge_when_clause merge_when_list
  | merge_when_clause
;

merge_when_clause :
    merge_when_tgt_matched opt_merge_when_condition THEN merge_update
  | merge_when_tgt_matched opt_merge_when_condition THEN DELETE_P
  | merge_when_tgt_not_matched opt_merge_when_condition THEN merge_insert
  | merge_when_tgt_matched opt_merge_when_condition THEN DO NOTHING
  | merge_when_tgt_not_matched opt_merge_when_condition THEN DO NOTHING
;

merge_when_tgt_matched :
    WHEN MATCHED
  | WHEN NOT MATCHED BY SOURCE
;

merge_when_tgt_not_matched :
    WHEN NOT MATCHED
  | WHEN NOT MATCHED BY TARGET
;

opt_merge_when_condition :
    AND a_expr
  | __empty
;

merge_update :
    UPDATE SET set_clause_list
;

merge_insert :
    INSERT merge_values_clause
  | INSERT OVERRIDING override_kind VALUE_P merge_values_clause
  | INSERT '(' insert_column_list ')' merge_values_clause
  | INSERT '(' insert_column_list ')' OVERRIDING override_kind VALUE_P merge_values_clause
  | INSERT DEFAULT VALUES
;

merge_values_clause :
    VALUES '(' expr_list ')'
;

DeclareCursorStmt :
    DECLARE ColId cursor_options CURSOR opt_hold FOR SelectStmt
;

cursor_options :
    cursor_options_1 cursor_options
  | __empty
;

cursor_options_1 :
    NO SCROLL
  | SCROLL
  | BINARY
  | ASENSITIVE
  | INSENSITIVE
;

opt_hold :
    __empty
  | WITH HOLD
  | WITHOUT HOLD
;

SelectStmt :
    select_no_parens
  | select_with_parens
;

select_with_parens :
    '(' select_no_parens ')'
  | '(' select_with_parens ')'
;

select_no_parens :
    simple_select
  | select_clause sort_clause
  | select_clause opt_sort_clause for_locking_clause opt_select_limit
  | select_clause opt_sort_clause select_limit opt_for_locking_clause
  | with_clause select_clause
  | with_clause select_clause sort_clause
  | with_clause select_clause opt_sort_clause for_locking_clause opt_select_limit
  | with_clause select_clause opt_sort_clause select_limit opt_for_locking_clause
;

select_clause :
    simple_select
  | select_with_parens
;

simple_select :
    SELECT opt_all_clause opt_target_list into_clause from_clause where_clause group_clause having_clause window_clause
  | SELECT distinct_clause target_list into_clause from_clause where_clause group_clause having_clause window_clause
  | values_clause
  | TABLE relation_expr
  | select_clause UNION set_quantifier select_clause
  | select_clause INTERSECT set_quantifier select_clause
  | select_clause EXCEPT set_quantifier select_clause
;

with_clause :
    WITH cte_list
  | WITH_LA cte_list
  | WITH RECURSIVE cte_list
;

cte_list :
    common_table_expr cte_list_1
  | common_table_expr
;

cte_list_1 :
    ',' common_table_expr cte_list_1
  | ',' common_table_expr
;

common_table_expr :
    ColId opt_name_list AS opt_materialized '(' PreparableStmt ')' opt_search_clause opt_cycle_clause
;

opt_materialized :
    MATERIALIZED
  | NOT MATERIALIZED
  | __empty
;

opt_search_clause :
    SEARCH DEPTH FIRST_P BY columnList SET ColId
  | SEARCH BREADTH FIRST_P BY columnList SET ColId
  | __empty
;

opt_cycle_clause :
    CYCLE columnList SET ColId TO AexprConst DEFAULT AexprConst USING ColId
  | CYCLE columnList SET ColId USING ColId
  | __empty
;

opt_with_clause :
    with_clause
  | __empty
;

into_clause :
    INTO OptTempTableName
  | __empty
;

OptTempTableName :
    TEMPORARY opt_table qualified_name
  | TEMP opt_table qualified_name
  | LOCAL TEMPORARY opt_table qualified_name
  | LOCAL TEMP opt_table qualified_name
  | GLOBAL TEMPORARY opt_table qualified_name
  | GLOBAL TEMP opt_table qualified_name
  | UNLOGGED opt_table qualified_name
  | TABLE qualified_name
  | qualified_name
;

opt_table :
    TABLE
  | __empty
;

set_quantifier :
    ALL
  | DISTINCT
  | __empty
;

distinct_clause :
    DISTINCT
  | DISTINCT ON '(' expr_list ')'
;

opt_all_clause :
    ALL
  | __empty
;

opt_distinct_clause :
    distinct_clause
  | opt_all_clause
;

opt_sort_clause :
    sort_clause
  | __empty
;

sort_clause :
    ORDER BY sortby_list
;

sortby_list :
    sortby sortby_list_1
  | sortby
;

sortby_list_1 :
    ',' sortby sortby_list_1
  | ',' sortby
;

sortby :
    a_expr USING qual_all_Op opt_nulls_order
  | a_expr opt_asc_desc opt_nulls_order
;

select_limit :
    limit_clause offset_clause
  | offset_clause limit_clause
  | limit_clause
  | offset_clause
;

opt_select_limit :
    select_limit
  | __empty
;

limit_clause :
    LIMIT select_limit_value
  | LIMIT select_limit_value ',' a_expr
  | FETCH first_or_next select_fetch_first_value row_or_rows ONLY
  | FETCH first_or_next select_fetch_first_value row_or_rows WITH TIES
  | FETCH first_or_next row_or_rows ONLY
  | FETCH first_or_next row_or_rows WITH TIES
;

offset_clause :
    OFFSET a_expr
  | OFFSET select_fetch_first_value row_or_rows
;

select_limit_value :
    a_expr
  | ALL
;

select_fetch_first_value :
    c_expr
  | '+' I_or_F_const
  | '-' I_or_F_const
;

I_or_F_const :
    ICONST
  | FCONST
;

row_or_rows :
    ROW
  | ROWS
;

first_or_next :
    FIRST_P
  | NEXT
;

group_clause :
    GROUP_P BY set_quantifier group_by_list
  | __empty
;

group_by_list :
    group_by_item group_by_list_1
  | group_by_item
;

group_by_list_1 :
    ',' group_by_item group_by_list_1
  | ',' group_by_item
;

group_by_item :
    a_expr
  | empty_grouping_set
  | cube_clause
  | rollup_clause
  | grouping_sets_clause
;

empty_grouping_set :
    '(' ')'
;

rollup_clause :
    ROLLUP '(' expr_list ')'
;

cube_clause :
    CUBE '(' expr_list ')'
;

grouping_sets_clause :
    GROUPING SETS '(' group_by_list ')'
;

having_clause :
    HAVING a_expr
  | __empty
;

for_locking_clause :
    for_locking_items
  | FOR READ ONLY
;

opt_for_locking_clause :
    for_locking_clause
  | __empty
;

for_locking_items :
    for_locking_item for_locking_items
  | for_locking_item
;

for_locking_item :
    for_locking_strength locked_rels_list opt_nowait_or_skip
;

for_locking_strength :
    FOR UPDATE
  | FOR NO KEY UPDATE
  | FOR SHARE
  | FOR KEY SHARE
;

locked_rels_list :
    OF qualified_name_list
  | __empty
;

values_clause :
    VALUES '(' expr_list ')' values_clause_1
  | VALUES '(' expr_list ')'
;

values_clause_1 :
    ',' '(' expr_list ')' values_clause_1
  | ',' '(' expr_list ')'
;

from_clause :
    FROM from_list
  | __empty
;

from_list :
    table_ref from_list_1
  | table_ref
;

from_list_1 :
    ',' table_ref from_list_1
  | ',' table_ref
;

table_ref :
    relation_expr opt_alias_clause
  | relation_expr opt_alias_clause tablesample_clause
  | func_table func_alias_clause
  | LATERAL_P func_table func_alias_clause
  | xmltable opt_alias_clause
  | LATERAL_P xmltable opt_alias_clause
  | select_with_parens opt_alias_clause
  | LATERAL_P select_with_parens opt_alias_clause
  | joined_table
  | '(' joined_table ')' alias_clause
  | json_table opt_alias_clause
  | LATERAL_P json_table opt_alias_clause
;

joined_table :
    '(' joined_table ')'
  | table_ref CROSS JOIN table_ref
  | table_ref join_type JOIN table_ref join_qual
  | table_ref JOIN table_ref join_qual
  | table_ref NATURAL join_type JOIN table_ref
  | table_ref NATURAL JOIN table_ref
;

alias_clause :
    AS ColId '(' name_list ')'
  | AS ColId
  | ColId '(' name_list ')'
  | ColId
;

opt_alias_clause :
    alias_clause
  | __empty
;

opt_alias_clause_for_join_using :
    AS ColId
  | __empty
;

func_alias_clause :
    alias_clause
  | AS '(' TableFuncElementList ')'
  | AS ColId '(' TableFuncElementList ')'
  | ColId '(' TableFuncElementList ')'
  | __empty
;

join_type :
    FULL opt_outer
  | LEFT opt_outer
  | RIGHT opt_outer
  | INNER_P
;

opt_outer :
    OUTER_P
  | __empty
;

join_qual :
    USING '(' name_list ')' opt_alias_clause_for_join_using
  | ON a_expr
;

relation_expr :
    qualified_name
  | extended_relation_expr
;

extended_relation_expr :
    qualified_name '*'
  | ONLY qualified_name
  | ONLY '(' qualified_name ')'
;

relation_expr_list :
    relation_expr relation_expr_list_1
  | relation_expr
;

relation_expr_list_1 :
    ',' relation_expr relation_expr_list_1
  | ',' relation_expr
;

relation_expr_opt_alias :
    relation_expr
  | relation_expr ColId
  | relation_expr AS ColId
;

tablesample_clause :
    TABLESAMPLE func_name '(' expr_list ')' opt_repeatable_clause
;

opt_repeatable_clause :
    REPEATABLE '(' a_expr ')'
  | __empty
;

func_table :
    func_expr_windowless opt_ordinality
  | ROWS FROM '(' rowsfrom_list ')' opt_ordinality
;

rowsfrom_item :
    func_expr_windowless opt_col_def_list
;

rowsfrom_list :
    rowsfrom_item rowsfrom_list_1
  | rowsfrom_item
;

rowsfrom_list_1 :
    ',' rowsfrom_item rowsfrom_list_1
  | ',' rowsfrom_item
;

opt_col_def_list :
    AS '(' TableFuncElementList ')'
  | __empty
;

opt_ordinality :
    WITH_LA ORDINALITY
  | __empty
;

where_clause :
    WHERE a_expr
  | __empty
;

where_or_current_clause :
    WHERE a_expr
  | WHERE CURRENT_P OF ColId
  | __empty
;

OptTableFuncElementList :
    TableFuncElementList
  | __empty
;

TableFuncElementList :
    TableFuncElement TableFuncElementList_1
  | TableFuncElement
;

TableFuncElementList_1 :
    ',' TableFuncElement TableFuncElementList_1
  | ',' TableFuncElement
;

TableFuncElement :
    ColId Typename opt_collate_clause
;

xmltable :
    XMLTABLE '(' c_expr xmlexists_argument COLUMNS xmltable_column_list ')'
  | XMLTABLE '(' XMLNAMESPACES '(' xml_namespace_list ')' ',' c_expr xmlexists_argument COLUMNS xmltable_column_list ')'
;

xmltable_column_list :
    xmltable_column_el xmltable_column_list_1
  | xmltable_column_el
;

xmltable_column_list_1 :
    ',' xmltable_column_el xmltable_column_list_1
  | ',' xmltable_column_el
;

xmltable_column_el :
    ColId Typename
  | ColId Typename xmltable_column_option_list
  | ColId FOR ORDINALITY
;

xmltable_column_option_list :
    xmltable_column_option_el xmltable_column_option_list
  | xmltable_column_option_el
;

xmltable_column_option_el :
    IDENT b_expr
  | DEFAULT b_expr
  | NOT NULL_P
  | NULL_P
  | PATH b_expr
;

xml_namespace_list :
    xml_namespace_el xml_namespace_list_1
  | xml_namespace_el
;

xml_namespace_list_1 :
    ',' xml_namespace_el xml_namespace_list_1
  | ',' xml_namespace_el
;

xml_namespace_el :
    b_expr AS ColLabel
  | DEFAULT b_expr
;

json_table :
    JSON_TABLE '(' json_value_expr ',' a_expr json_table_path_name_opt json_passing_clause_opt COLUMNS '(' json_table_column_definition_list ')' json_on_error_clause_opt ')'
;

json_table_path_name_opt :
    AS ColId
  | __empty
;

json_table_column_definition_list :
    json_table_column_definition json_table_column_definition_list_1
  | json_table_column_definition
;

json_table_column_definition_list_1 :
    ',' json_table_column_definition json_table_column_definition_list_1
  | ',' json_table_column_definition
;

json_table_column_definition :
    ColId FOR ORDINALITY
  | ColId Typename json_table_column_path_clause_opt json_wrapper_behavior json_quotes_clause_opt json_behavior_clause_opt
  | ColId Typename json_format_clause json_table_column_path_clause_opt json_wrapper_behavior json_quotes_clause_opt json_behavior_clause_opt
  | ColId Typename EXISTS json_table_column_path_clause_opt json_on_error_clause_opt
  | NESTED path_opt SCONST COLUMNS '(' json_table_column_definition_list ')'
  | NESTED path_opt SCONST AS ColId COLUMNS '(' json_table_column_definition_list ')'
;

path_opt :
    PATH
  | __empty
;

json_table_column_path_clause_opt :
    PATH SCONST
  | __empty
;

Typename :
    SimpleTypename opt_array_bounds
  | SETOF SimpleTypename opt_array_bounds
  | SimpleTypename ARRAY '[' ICONST ']'
  | SETOF SimpleTypename ARRAY '[' ICONST ']'
  | SimpleTypename ARRAY
  | SETOF SimpleTypename ARRAY
;

opt_array_bounds :
    opt_array_bounds_1 opt_array_bounds
  | __empty
;

opt_array_bounds_1 :
    '[' ']'
  | '[' ICONST ']'
;

SimpleTypename :
    GenericType
  | Numeric
  | Bit
  | Character
  | ConstDatetime
  | INTERVAL opt_interval
  | INTERVAL '(' ICONST ')'
  | JSON
;

ConstTypename :
    Numeric
  | ConstBit
  | ConstCharacter
  | ConstDatetime
  | JSON
;

GenericType :
    type_function_name opt_type_modifiers
  | type_function_name attrs opt_type_modifiers
;

opt_type_modifiers :
    '(' expr_list ')'
  | __empty
;

Numeric :
    INT_P
  | INTEGER
  | SMALLINT
  | BIGINT
  | REAL
  | FLOAT_P opt_float
  | DOUBLE_P PRECISION
  | DECIMAL_P opt_type_modifiers
  | DEC opt_type_modifiers
  | NUMERIC opt_type_modifiers
  | BOOLEAN_P
;

opt_float :
    '(' ICONST ')'
  | __empty
;

Bit :
    BitWithLength
  | BitWithoutLength
;

ConstBit :
    BitWithLength
  | BitWithoutLength
;

BitWithLength :
    BIT opt_varying '(' expr_list ')'
;

BitWithoutLength :
    BIT opt_varying
;

Character :
    CharacterWithLength
  | CharacterWithoutLength
;

ConstCharacter :
    CharacterWithLength
  | CharacterWithoutLength
;

CharacterWithLength :
    character '(' ICONST ')'
;

CharacterWithoutLength :
    character
;

character :
    CHARACTER opt_varying
  | CHAR_P opt_varying
  | VARCHAR
  | NATIONAL CHARACTER opt_varying
  | NATIONAL CHAR_P opt_varying
  | NCHAR opt_varying
;

opt_varying :
    VARYING
  | __empty
;

ConstDatetime :
    TIMESTAMP '(' ICONST ')' opt_timezone
  | TIMESTAMP opt_timezone
  | TIME '(' ICONST ')' opt_timezone
  | TIME opt_timezone
;

opt_timezone :
    WITH_LA TIME ZONE
  | WITHOUT_LA TIME ZONE
  | __empty
;

opt_interval :
    YEAR_P
  | MONTH_P
  | DAY_P
  | HOUR_P
  | MINUTE_P
  | interval_second
  | YEAR_P TO MONTH_P
  | DAY_P TO HOUR_P
  | DAY_P TO MINUTE_P
  | DAY_P TO interval_second
  | HOUR_P TO MINUTE_P
  | HOUR_P TO interval_second
  | MINUTE_P TO interval_second
  | __empty
;

interval_second :
    SECOND_P
  | SECOND_P '(' ICONST ')'
;

a_expr :
    a_expr_2 a_expr_3
  | a_expr_2
;

a_expr_1 :
    TYPECAST Typename
  | COLLATE any_name
  | AT TIME ZONE a_expr
  | AT LOCAL
  | '+' a_expr
  | '-' a_expr
  | '*' a_expr
  | '/' a_expr
  | '%' a_expr
  | '^' a_expr
  | '<' a_expr
  | '>' a_expr
  | '=' a_expr
  | LESS_EQUALS a_expr
  | GREATER_EQUALS a_expr
  | NOT_EQUALS a_expr
  | qual_Op a_expr
  | AND a_expr
  | OR a_expr
  | LIKE a_expr
  | LIKE a_expr ESCAPE a_expr
  | NOT_LA LIKE a_expr
  | NOT_LA LIKE a_expr ESCAPE a_expr
  | ILIKE a_expr
  | ILIKE a_expr ESCAPE a_expr
  | NOT_LA ILIKE a_expr
  | NOT_LA ILIKE a_expr ESCAPE a_expr
  | SIMILAR TO a_expr
  | SIMILAR TO a_expr ESCAPE a_expr
  | NOT_LA SIMILAR TO a_expr
  | NOT_LA SIMILAR TO a_expr ESCAPE a_expr
  | IS NULL_P
  | ISNULL
  | IS NOT NULL_P
  | NOTNULL
  | IS TRUE_P
  | IS NOT TRUE_P
  | IS FALSE_P
  | IS NOT FALSE_P
  | IS UNKNOWN
  | IS NOT UNKNOWN
  | IS DISTINCT FROM a_expr
  | IS NOT DISTINCT FROM a_expr
  | BETWEEN opt_asymmetric b_expr AND a_expr
  | NOT_LA BETWEEN opt_asymmetric b_expr AND a_expr
  | BETWEEN SYMMETRIC b_expr AND a_expr
  | NOT_LA BETWEEN SYMMETRIC b_expr AND a_expr
  | IN_P in_expr
  | NOT_LA IN_P in_expr
  | subquery_Op sub_type select_with_parens
  | subquery_Op sub_type '(' a_expr ')'
  | IS DOCUMENT_P
  | IS NOT DOCUMENT_P
  | IS NORMALIZED
  | IS unicode_normal_form NORMALIZED
  | IS NOT NORMALIZED
  | IS NOT unicode_normal_form NORMALIZED
  | IS json_predicate_type_constraint json_key_uniqueness_constraint_opt
  | IS NOT json_predicate_type_constraint json_key_uniqueness_constraint_opt
;

a_expr_2 :
    c_expr
  | '+' a_expr
  | '-' a_expr
  | qual_Op a_expr
  | NOT a_expr
  | NOT_LA a_expr
  | row OVERLAPS row
  | UNIQUE opt_unique_null_treatment select_with_parens
  | DEFAULT
;

a_expr_3 :
    a_expr_1 a_expr_3
  | a_expr_1
;

b_expr :
    b_expr_2 b_expr_3
  | b_expr_2
;

b_expr_1 :
    TYPECAST Typename
  | '+' b_expr
  | '-' b_expr
  | '*' b_expr
  | '/' b_expr
  | '%' b_expr
  | '^' b_expr
  | '<' b_expr
  | '>' b_expr
  | '=' b_expr
  | LESS_EQUALS b_expr
  | GREATER_EQUALS b_expr
  | NOT_EQUALS b_expr
  | qual_Op b_expr
  | IS DISTINCT FROM b_expr
  | IS NOT DISTINCT FROM b_expr
  | IS DOCUMENT_P
  | IS NOT DOCUMENT_P
;

b_expr_2 :
    c_expr
  | '+' b_expr
  | '-' b_expr
  | qual_Op b_expr
;

b_expr_3 :
    b_expr_1 b_expr_3
  | b_expr_1
;

c_expr :
    columnref
  | AexprConst
  | PARAM opt_indirection
  | '(' a_expr ')' opt_indirection
  | case_expr
  | func_expr
  | select_with_parens
  | select_with_parens indirection
  | EXISTS select_with_parens
  | ARRAY select_with_parens
  | ARRAY array_expr
  | explicit_row
  | implicit_row
  | GROUPING '(' expr_list ')'
;

func_application :
    func_name '(' ')'
  | func_name '(' func_arg_list opt_sort_clause ')'
  | func_name '(' VARIADIC func_arg_expr opt_sort_clause ')'
  | func_name '(' func_arg_list ',' VARIADIC func_arg_expr opt_sort_clause ')'
  | func_name '(' ALL func_arg_list opt_sort_clause ')'
  | func_name '(' DISTINCT func_arg_list opt_sort_clause ')'
  | func_name '(' '*' ')'
;

func_expr :
    func_application within_group_clause filter_clause over_clause
  | json_aggregate_func filter_clause over_clause
  | func_expr_common_subexpr
;

func_expr_windowless :
    func_application
  | func_expr_common_subexpr
  | json_aggregate_func
;

func_expr_common_subexpr :
    COLLATION FOR '(' a_expr ')'
  | CURRENT_DATE
  | CURRENT_TIME
  | CURRENT_TIME '(' ICONST ')'
  | CURRENT_TIMESTAMP
  | CURRENT_TIMESTAMP '(' ICONST ')'
  | LOCALTIME
  | LOCALTIME '(' ICONST ')'
  | LOCALTIMESTAMP
  | LOCALTIMESTAMP '(' ICONST ')'
  | CURRENT_ROLE
  | CURRENT_USER
  | SESSION_USER
  | SYSTEM_USER
  | USER
  | CURRENT_CATALOG
  | CURRENT_SCHEMA
  | CAST '(' a_expr AS Typename ')'
  | EXTRACT '(' extract_list ')'
  | NORMALIZE '(' a_expr ')'
  | NORMALIZE '(' a_expr ',' unicode_normal_form ')'
  | OVERLAY '(' overlay_list ')'
  | OVERLAY '(' func_arg_list_opt ')'
  | POSITION '(' position_list ')'
  | SUBSTRING '(' substr_list ')'
  | SUBSTRING '(' func_arg_list_opt ')'
  | TREAT '(' a_expr AS Typename ')'
  | TRIM '(' BOTH trim_list ')'
  | TRIM '(' LEADING trim_list ')'
  | TRIM '(' TRAILING trim_list ')'
  | TRIM '(' trim_list ')'
  | NULLIF '(' a_expr ',' a_expr ')'
  | COALESCE '(' expr_list ')'
  | GREATEST '(' expr_list ')'
  | LEAST '(' expr_list ')'
  | XMLCONCAT '(' expr_list ')'
  | XMLELEMENT '(' NAME_P ColLabel ')'
  | XMLELEMENT '(' NAME_P ColLabel ',' xml_attributes ')'
  | XMLELEMENT '(' NAME_P ColLabel ',' expr_list ')'
  | XMLELEMENT '(' NAME_P ColLabel ',' xml_attributes ',' expr_list ')'
  | XMLEXISTS '(' c_expr xmlexists_argument ')'
  | XMLFOREST '(' xml_attribute_list ')'
  | XMLPARSE '(' document_or_content a_expr xml_whitespace_option ')'
  | XMLPI '(' NAME_P ColLabel ')'
  | XMLPI '(' NAME_P ColLabel ',' a_expr ')'
  | XMLROOT '(' a_expr ',' xml_root_version opt_xml_root_standalone ')'
  | XMLSERIALIZE '(' document_or_content a_expr AS SimpleTypename xml_indent_option ')'
  | JSON_OBJECT '(' func_arg_list ')'
  | JSON_OBJECT '(' json_name_and_value_list json_object_constructor_null_clause_opt json_key_uniqueness_constraint_opt json_returning_clause_opt ')'
  | JSON_OBJECT '(' json_returning_clause_opt ')'
  | JSON_ARRAY '(' json_value_expr_list json_array_constructor_null_clause_opt json_returning_clause_opt ')'
  | JSON_ARRAY '(' select_no_parens json_format_clause_opt json_returning_clause_opt ')'
  | JSON_ARRAY '(' json_returning_clause_opt ')'
  | JSON '(' json_value_expr json_key_uniqueness_constraint_opt ')'
  | JSON_SCALAR '(' a_expr ')'
  | JSON_SERIALIZE '(' json_value_expr json_returning_clause_opt ')'
  | MERGE_ACTION '(' ')'
  | JSON_QUERY '(' json_value_expr ',' a_expr json_passing_clause_opt json_returning_clause_opt json_wrapper_behavior json_quotes_clause_opt json_behavior_clause_opt ')'
  | JSON_EXISTS '(' json_value_expr ',' a_expr json_passing_clause_opt json_on_error_clause_opt ')'
  | JSON_VALUE '(' json_value_expr ',' a_expr json_passing_clause_opt json_returning_clause_opt json_behavior_clause_opt ')'
;

xml_root_version :
    VERSION_P a_expr
  | VERSION_P NO VALUE_P
;

opt_xml_root_standalone :
    ',' STANDALONE_P YES_P
  | ',' STANDALONE_P NO
  | ',' STANDALONE_P NO VALUE_P
  | __empty
;

xml_attributes :
    XMLATTRIBUTES '(' xml_attribute_list ')'
;

xml_attribute_list :
    xml_attribute_el xml_attribute_list_1
  | xml_attribute_el
;

xml_attribute_list_1 :
    ',' xml_attribute_el xml_attribute_list_1
  | ',' xml_attribute_el
;

xml_attribute_el :
    a_expr AS ColLabel
  | a_expr
;

document_or_content :
    DOCUMENT_P
  | CONTENT_P
;

xml_indent_option :
    INDENT
  | NO INDENT
  | __empty
;

xml_whitespace_option :
    PRESERVE WHITESPACE_P
  | STRIP_P WHITESPACE_P
  | __empty
;

xmlexists_argument :
    PASSING c_expr
  | PASSING c_expr xml_passing_mech
  | PASSING xml_passing_mech c_expr
  | PASSING xml_passing_mech c_expr xml_passing_mech
;

xml_passing_mech :
    BY REF_P
  | BY VALUE_P
;

within_group_clause :
    WITHIN GROUP_P '(' sort_clause ')'
  | __empty
;

filter_clause :
    FILTER '(' WHERE a_expr ')'
  | __empty
;

window_clause :
    WINDOW window_definition_list
  | __empty
;

window_definition_list :
    window_definition window_definition_list_1
  | window_definition
;

window_definition_list_1 :
    ',' window_definition window_definition_list_1
  | ',' window_definition
;

window_definition :
    ColId AS window_specification
;

over_clause :
    OVER window_specification
  | OVER ColId
  | __empty
;

window_specification :
    '(' opt_existing_window_name opt_partition_clause opt_sort_clause opt_frame_clause ')'
;

opt_existing_window_name :
    ColId
  | __empty
;

opt_partition_clause :
    PARTITION BY expr_list
  | __empty
;

opt_frame_clause :
    RANGE frame_extent opt_window_exclusion_clause
  | ROWS frame_extent opt_window_exclusion_clause
  | GROUPS frame_extent opt_window_exclusion_clause
  | __empty
;

frame_extent :
    frame_bound
  | BETWEEN frame_bound AND frame_bound
;

frame_bound :
    UNBOUNDED PRECEDING
  | UNBOUNDED FOLLOWING
  | CURRENT_P ROW
  | a_expr PRECEDING
  | a_expr FOLLOWING
;

opt_window_exclusion_clause :
    EXCLUDE CURRENT_P ROW
  | EXCLUDE GROUP_P
  | EXCLUDE TIES
  | EXCLUDE NO OTHERS
  | __empty
;

row :
    ROW '(' expr_list ')'
  | ROW '(' ')'
  | '(' expr_list ',' a_expr ')'
;

explicit_row :
    ROW '(' expr_list ')'
  | ROW '(' ')'
;

implicit_row :
    '(' expr_list ',' a_expr ')'
;

sub_type :
    ANY
  | SOME
  | ALL
;

all_Op :
    Op
  | MathOp
;

MathOp :
    '+'
  | '-'
  | '*'
  | '/'
  | '%'
  | '^'
  | '<'
  | '>'
  | '='
  | LESS_EQUALS
  | GREATER_EQUALS
  | NOT_EQUALS
;

qual_Op :
    Op
  | OPERATOR '(' any_operator ')'
;

qual_all_Op :
    all_Op
  | OPERATOR '(' any_operator ')'
;

subquery_Op :
    all_Op
  | OPERATOR '(' any_operator ')'
  | LIKE
  | NOT_LA LIKE
  | ILIKE
  | NOT_LA ILIKE
;

expr_list :
    a_expr expr_list_1
  | a_expr
;

expr_list_1 :
    ',' a_expr expr_list_1
  | ',' a_expr
;

func_arg_list :
    func_arg_expr func_arg_list_1
  | func_arg_expr
;

func_arg_list_1 :
    ',' func_arg_expr func_arg_list_1
  | ',' func_arg_expr
;

func_arg_expr :
    a_expr
  | type_function_name COLON_EQUALS a_expr
  | type_function_name EQUALS_GREATER a_expr
;

func_arg_list_opt :
    func_arg_list
  | __empty
;

type_list :
    Typename type_list_1
  | Typename
;

type_list_1 :
    ',' Typename type_list_1
  | ',' Typename
;

array_expr :
    '[' expr_list ']'
  | '[' array_expr_list ']'
  | '[' ']'
;

array_expr_list :
    array_expr array_expr_list_1
  | array_expr
;

array_expr_list_1 :
    ',' array_expr array_expr_list_1
  | ',' array_expr
;

extract_list :
    extract_arg FROM a_expr
;

extract_arg :
    IDENT
  | YEAR_P
  | MONTH_P
  | DAY_P
  | HOUR_P
  | MINUTE_P
  | SECOND_P
  | SCONST
;

unicode_normal_form :
    NFC
  | NFD
  | NFKC
  | NFKD
;

overlay_list :
    a_expr PLACING a_expr FROM a_expr FOR a_expr
  | a_expr PLACING a_expr FROM a_expr
;

position_list :
    b_expr IN_P b_expr
;

substr_list :
    a_expr FROM a_expr FOR a_expr
  | a_expr FOR a_expr FROM a_expr
  | a_expr FROM a_expr
  | a_expr FOR a_expr
  | a_expr SIMILAR a_expr ESCAPE a_expr
;

trim_list :
    a_expr FROM expr_list
  | FROM expr_list
  | expr_list
;

in_expr :
    select_with_parens
  | '(' expr_list ')'
;

case_expr :
    CASE case_arg when_clause_list case_default END_P
;

when_clause_list :
    when_clause when_clause_list
  | when_clause
;

when_clause :
    WHEN a_expr THEN a_expr
;

case_default :
    ELSE a_expr
  | __empty
;

case_arg :
    a_expr
  | __empty
;

columnref :
    ColId
  | ColId indirection
;

indirection_el :
    '.' ColLabel
  | '.' '*'
  | '[' a_expr ']'
  | '[' opt_slice_bound ':' opt_slice_bound ']'
;

opt_slice_bound :
    a_expr
  | __empty
;

indirection :
    indirection_el indirection
  | indirection_el
;

opt_indirection :
    indirection_el opt_indirection
  | __empty
;

opt_asymmetric :
    ASYMMETRIC
  | __empty
;

json_passing_clause_opt :
    PASSING json_arguments
  | __empty
;

json_arguments :
    json_argument json_arguments_1
  | json_argument
;

json_arguments_1 :
    ',' json_argument json_arguments_1
  | ',' json_argument
;

json_argument :
    json_value_expr AS ColLabel
;

json_wrapper_behavior :
    WITHOUT WRAPPER
  | WITHOUT ARRAY WRAPPER
  | WITH WRAPPER
  | WITH ARRAY WRAPPER
  | WITH CONDITIONAL ARRAY WRAPPER
  | WITH UNCONDITIONAL ARRAY WRAPPER
  | WITH CONDITIONAL WRAPPER
  | WITH UNCONDITIONAL WRAPPER
  | __empty
;

json_behavior :
    DEFAULT a_expr
  | json_behavior_type
;

json_behavior_type :
    ERROR_P
  | NULL_P
  | TRUE_P
  | FALSE_P
  | UNKNOWN
  | EMPTY_P ARRAY
  | EMPTY_P OBJECT_P
  | EMPTY_P
;

json_behavior_clause_opt :
    json_behavior ON EMPTY_P
  | json_behavior ON ERROR_P
  | json_behavior ON EMPTY_P json_behavior ON ERROR_P
  | __empty
;

json_on_error_clause_opt :
    json_behavior ON ERROR_P
  | __empty
;

json_value_expr :
    a_expr json_format_clause_opt
;

json_format_clause :
    FORMAT_LA JSON ENCODING ColId
  | FORMAT_LA JSON
;

json_format_clause_opt :
    json_format_clause
  | __empty
;

json_quotes_clause_opt :
    KEEP QUOTES ON SCALAR STRING_P
  | KEEP QUOTES
  | OMIT QUOTES ON SCALAR STRING_P
  | OMIT QUOTES
  | __empty
;

json_returning_clause_opt :
    RETURNING Typename json_format_clause_opt
  | __empty
;

json_predicate_type_constraint :
    JSON
  | JSON VALUE_P
  | JSON ARRAY
  | JSON OBJECT_P
  | JSON SCALAR
;

json_key_uniqueness_constraint_opt :
    WITH UNIQUE KEYS
  | WITH UNIQUE
  | WITHOUT UNIQUE KEYS
  | WITHOUT UNIQUE
  | __empty
;

json_name_and_value_list :
    json_name_and_value json_name_and_value_list_1
  | json_name_and_value
;

json_name_and_value_list_1 :
    ',' json_name_and_value json_name_and_value_list_1
  | ',' json_name_and_value
;

json_name_and_value :
    c_expr VALUE_P json_value_expr
  | a_expr ':' json_value_expr
;

json_object_constructor_null_clause_opt :
    NULL_P ON NULL_P
  | ABSENT ON NULL_P
  | __empty
;

json_array_constructor_null_clause_opt :
    NULL_P ON NULL_P
  | ABSENT ON NULL_P
  | __empty
;

json_value_expr_list :
    json_value_expr json_value_expr_list_1
  | json_value_expr
;

json_value_expr_list_1 :
    ',' json_value_expr json_value_expr_list_1
  | ',' json_value_expr
;

json_aggregate_func :
    JSON_OBJECTAGG '(' json_name_and_value json_object_constructor_null_clause_opt json_key_uniqueness_constraint_opt json_returning_clause_opt ')'
  | JSON_ARRAYAGG '(' json_value_expr json_array_aggregate_order_by_clause_opt json_array_constructor_null_clause_opt json_returning_clause_opt ')'
;

json_array_aggregate_order_by_clause_opt :
    ORDER BY sortby_list
  | __empty
;

opt_target_list :
    target_list
  | __empty
;

target_list :
    target_el target_list_1
  | target_el
;

target_list_1 :
    ',' target_el target_list_1
  | ',' target_el
;

target_el :
    a_expr AS ColLabel
  | a_expr BareColLabel
  | a_expr
  | '*'
;

qualified_name_list :
    qualified_name qualified_name_list_1
  | qualified_name
;

qualified_name_list_1 :
    ',' qualified_name qualified_name_list_1
  | ',' qualified_name
;

qualified_name :
    ColId
  | ColId indirection
;

name_list :
    ColId name_list_1
  | ColId
;

name_list_1 :
    ',' ColId name_list_1
  | ',' ColId
;

func_name :
    type_function_name
  | ColId indirection
;

AexprConst :
    ICONST
  | FCONST
  | SCONST
  | BCONST
  | XCONST
  | func_name SCONST
  | func_name '(' func_arg_list opt_sort_clause ')' SCONST
  | ConstTypename SCONST
  | INTERVAL SCONST opt_interval
  | INTERVAL '(' ICONST ')' SCONST
  | TRUE_P
  | FALSE_P
  | NULL_P
;

SignedIconst :
    ICONST
  | '+' ICONST
  | '-' ICONST
;

RoleId :
    RoleSpec
;

RoleSpec :
    NonReservedWord
  | CURRENT_ROLE
  | CURRENT_USER
  | SESSION_USER
;

role_list :
    RoleSpec role_list_1
  | RoleSpec
;

role_list_1 :
    ',' RoleSpec role_list_1
  | ',' RoleSpec
;

PLpgSQL_Expr :
    opt_distinct_clause opt_target_list from_clause where_clause group_clause having_clause window_clause opt_sort_clause opt_select_limit opt_for_locking_clause
;

PLAssignStmt :
    plassign_target opt_indirection plassign_equals PLpgSQL_Expr
;

plassign_target :
    ColId
  | PARAM
;

plassign_equals :
    COLON_EQUALS
  | '='
;

ColId :
    IDENT
  | unreserved_keyword
  | col_name_keyword
;

type_function_name :
    IDENT
  | unreserved_keyword
  | type_func_name_keyword
;

NonReservedWord :
    IDENT
  | unreserved_keyword
  | col_name_keyword
  | type_func_name_keyword
;

ColLabel :
    IDENT
  | unreserved_keyword
  | col_name_keyword
  | type_func_name_keyword
  | reserved_keyword
;

BareColLabel :
    IDENT
  | bare_label_keyword
;

unreserved_keyword :
    ABORT_P
  | ABSENT
  | ABSOLUTE_P
  | ACCESS
  | ACTION
  | ADD_P
  | ADMIN
  | AFTER
  | AGGREGATE
  | ALSO
  | ALTER
  | ALWAYS
  | ASENSITIVE
  | ASSERTION
  | ASSIGNMENT
  | AT
  | ATOMIC
  | ATTACH
  | ATTRIBUTE
  | BACKWARD
  | BEFORE
  | BEGIN_P
  | BREADTH
  | BY
  | CACHE
  | CALL
  | CALLED
  | CASCADE
  | CASCADED
  | CATALOG_P
  | CHAIN
  | CHARACTERISTICS
  | CHECKPOINT
  | CLASS
  | CLOSE
  | CLUSTER
  | COLUMNS
  | COMMENT
  | COMMENTS
  | COMMIT
  | COMMITTED
  | COMPRESSION
  | CONDITIONAL
  | CONFIGURATION
  | CONFLICT
  | CONNECTION
  | CONSTRAINTS
  | CONTENT_P
  | CONTINUE_P
  | CONVERSION_P
  | COPY
  | COST
  | CSV
  | CUBE
  | CURRENT_P
  | CURSOR
  | CYCLE
  | DATA_P
  | DATABASE
  | DAY_P
  | DEALLOCATE
  | DECLARE
  | DEFAULTS
  | DEFERRED
  | DEFINER
  | DELETE_P
  | DELIMITER
  | DELIMITERS
  | DEPENDS
  | DEPTH
  | DETACH
  | DICTIONARY
  | DISABLE_P
  | DISCARD
  | DOCUMENT_P
  | DOMAIN_P
  | DOUBLE_P
  | DROP
  | EACH
  | EMPTY_P
  | ENABLE_P
  | ENCODING
  | ENCRYPTED
  | ENFORCED
  | ENUM_P
  | ERROR_P
  | ESCAPE
  | EVENT
  | EXCLUDE
  | EXCLUDING
  | EXCLUSIVE
  | EXECUTE
  | EXPLAIN
  | EXPRESSION
  | EXTENSION
  | EXTERNAL
  | FAMILY
  | FILTER
  | FINALIZE
  | FIRST_P
  | FOLLOWING
  | FORCE
  | FORMAT
  | FORWARD
  | FUNCTION
  | FUNCTIONS
  | GENERATED
  | GLOBAL
  | GRANTED
  | GROUPS
  | HANDLER
  | HEADER_P
  | HOLD
  | HOUR_P
  | IDENTITY_P
  | IF_P
  | IMMEDIATE
  | IMMUTABLE
  | IMPLICIT_P
  | IMPORT_P
  | INCLUDE
  | INCLUDING
  | INCREMENT
  | INDENT
  | INDEX
  | INDEXES
  | INHERIT
  | INHERITS
  | INLINE_P
  | INPUT_P
  | INSENSITIVE
  | INSERT
  | INSTEAD
  | INVOKER
  | ISOLATION
  | KEEP
  | KEY
  | KEYS
  | LABEL
  | LANGUAGE
  | LARGE_P
  | LAST_P
  | LEAKPROOF
  | LEVEL
  | LISTEN
  | LOAD
  | LOCAL
  | LOCATION
  | LOCK_P
  | LOCKED
  | LOGGED
  | MAPPING
  | MATCH
  | MATCHED
  | MATERIALIZED
  | MAXVALUE
  | MERGE
  | METHOD
  | MINUTE_P
  | MINVALUE
  | MODE
  | MONTH_P
  | MOVE
  | NAME_P
  | NAMES
  | NESTED
  | NEW
  | NEXT
  | NFC
  | NFD
  | NFKC
  | NFKD
  | NO
  | NORMALIZED
  | NOTHING
  | NOTIFY
  | NOWAIT
  | NULLS_P
  | OBJECT_P
  | OF
  | OFF
  | OIDS
  | OLD
  | OMIT
  | OPERATOR
  | OPTION
  | OPTIONS
  | ORDINALITY
  | OTHERS
  | OVER
  | OVERRIDING
  | OWNED
  | OWNER
  | PARALLEL
  | PARAMETER
  | PARSER
  | PARTIAL
  | PARTITION
  | PASSING
  | PASSWORD
  | PATH
  | PERIOD
  | PLAN
  | PLANS
  | POLICY
  | PRECEDING
  | PREPARE
  | PREPARED
  | PRESERVE
  | PRIOR
  | PRIVILEGES
  | PROCEDURAL
  | PROCEDURE
  | PROCEDURES
  | PROGRAM
  | PUBLICATION
  | QUOTE
  | QUOTES
  | RANGE
  | READ
  | REASSIGN
  | RECURSIVE
  | REF_P
  | REFERENCING
  | REFRESH
  | REINDEX
  | RELATIVE_P
  | RELEASE
  | RENAME
  | REPEATABLE
  | REPLACE
  | REPLICA
  | RESET
  | RESTART
  | RESTRICT
  | RETURN
  | RETURNS
  | REVOKE
  | ROLE
  | ROLLBACK
  | ROLLUP
  | ROUTINE
  | ROUTINES
  | ROWS
  | RULE
  | SAVEPOINT
  | SCALAR
  | SCHEMA
  | SCHEMAS
  | SCROLL
  | SEARCH
  | SECOND_P
  | SECURITY
  | SEQUENCE
  | SEQUENCES
  | SERIALIZABLE
  | SERVER
  | SESSION
  | SET
  | SETS
  | SHARE
  | SHOW
  | SIMPLE
  | SKIP
  | SNAPSHOT
  | SOURCE
  | SQL_P
  | STABLE
  | STANDALONE_P
  | START
  | STATEMENT
  | STATISTICS
  | STDIN
  | STDOUT
  | STORAGE
  | STORED
  | STRICT_P
  | STRING_P
  | STRIP_P
  | SUBSCRIPTION
  | SUPPORT
  | SYSID
  | SYSTEM_P
  | TABLES
  | TABLESPACE
  | TARGET
  | TEMP
  | TEMPLATE
  | TEMPORARY
  | TEXT_P
  | TIES
  | TRANSACTION
  | TRANSFORM
  | TRIGGER
  | TRUNCATE
  | TRUSTED
  | TYPE_P
  | TYPES_P
  | UESCAPE
  | UNBOUNDED
  | UNCOMMITTED
  | UNCONDITIONAL
  | UNENCRYPTED
  | UNKNOWN
  | UNLISTEN
  | UNLOGGED
  | UNTIL
  | UPDATE
  | VACUUM
  | VALID
  | VALIDATE
  | VALIDATOR
  | VALUE_P
  | VARYING
  | VERSION_P
  | VIEW
  | VIEWS
  | VIRTUAL
  | VOLATILE
  | WHITESPACE_P
  | WITHIN
  | WITHOUT
  | WORK
  | WRAPPER
  | WRITE
  | XML_P
  | YEAR_P
  | YES_P
  | ZONE
;

col_name_keyword :
    BETWEEN
  | BIGINT
  | BIT
  | BOOLEAN_P
  | CHAR_P
  | CHARACTER
  | COALESCE
  | DEC
  | DECIMAL_P
  | EXISTS
  | EXTRACT
  | FLOAT_P
  | GREATEST
  | GROUPING
  | INOUT
  | INT_P
  | INTEGER
  | INTERVAL
  | JSON
  | JSON_ARRAY
  | JSON_ARRAYAGG
  | JSON_EXISTS
  | JSON_OBJECT
  | JSON_OBJECTAGG
  | JSON_QUERY
  | JSON_SCALAR
  | JSON_SERIALIZE
  | JSON_TABLE
  | JSON_VALUE
  | LEAST
  | MERGE_ACTION
  | NATIONAL
  | NCHAR
  | NONE
  | NORMALIZE
  | NULLIF
  | NUMERIC
  | OUT_P
  | OVERLAY
  | POSITION
  | PRECISION
  | REAL
  | ROW
  | SETOF
  | SMALLINT
  | SUBSTRING
  | TIME
  | TIMESTAMP
  | TREAT
  | TRIM
  | VALUES
  | VARCHAR
  | XMLATTRIBUTES
  | XMLCONCAT
  | XMLELEMENT
  | XMLEXISTS
  | XMLFOREST
  | XMLNAMESPACES
  | XMLPARSE
  | XMLPI
  | XMLROOT
  | XMLSERIALIZE
  | XMLTABLE
;

type_func_name_keyword :
    AUTHORIZATION
  | BINARY
  | COLLATION
  | CONCURRENTLY
  | CROSS
  | CURRENT_SCHEMA
  | FREEZE
  | FULL
  | ILIKE
  | INNER_P
  | IS
  | ISNULL
  | JOIN
  | LEFT
  | LIKE
  | NATURAL
  | NOTNULL
  | OUTER_P
  | OVERLAPS
  | RIGHT
  | SIMILAR
  | TABLESAMPLE
  | VERBOSE
;

reserved_keyword :
    ALL
  | ANALYSE
  | ANALYZE
  | AND
  | ANY
  | ARRAY
  | AS
  | ASC
  | ASYMMETRIC
  | BOTH
  | CASE
  | CAST
  | CHECK
  | COLLATE
  | COLUMN
  | CONSTRAINT
  | CREATE
  | CURRENT_CATALOG
  | CURRENT_DATE
  | CURRENT_ROLE
  | CURRENT_TIME
  | CURRENT_TIMESTAMP
  | CURRENT_USER
  | DEFAULT
  | DEFERRABLE
  | DESC
  | DISTINCT
  | DO
  | ELSE
  | END_P
  | EXCEPT
  | FALSE_P
  | FETCH
  | FOR
  | FOREIGN
  | FROM
  | GRANT
  | GROUP_P
  | HAVING
  | IN_P
  | INITIALLY
  | INTERSECT
  | INTO
  | LATERAL_P
  | LEADING
  | LIMIT
  | LOCALTIME
  | LOCALTIMESTAMP
  | NOT
  | NULL_P
  | OFFSET
  | ON
  | ONLY
  | OR
  | ORDER
  | PLACING
  | PRIMARY
  | REFERENCES
  | RETURNING
  | SELECT
  | SESSION_USER
  | SOME
  | SYMMETRIC
  | SYSTEM_USER
  | TABLE
  | THEN
  | TO
  | TRAILING
  | TRUE_P
  | UNION
  | UNIQUE
  | USER
  | USING
  | VARIADIC
  | WHEN
  | WHERE
  | WINDOW
  | WITH
;

bare_label_keyword :
    ABORT_P
  | ABSENT
  | ABSOLUTE_P
  | ACCESS
  | ACTION
  | ADD_P
  | ADMIN
  | AFTER
  | AGGREGATE
  | ALL
  | ALSO
  | ALTER
  | ALWAYS
  | ANALYSE
  | ANALYZE
  | AND
  | ANY
  | ASC
  | ASENSITIVE
  | ASSERTION
  | ASSIGNMENT
  | ASYMMETRIC
  | AT
  | ATOMIC
  | ATTACH
  | ATTRIBUTE
  | AUTHORIZATION
  | BACKWARD
  | BEFORE
  | BEGIN_P
  | BETWEEN
  | BIGINT
  | BINARY
  | BIT
  | BOOLEAN_P
  | BOTH
  | BREADTH
  | BY
  | CACHE
  | CALL
  | CALLED
  | CASCADE
  | CASCADED
  | CASE
  | CAST
  | CATALOG_P
  | CHAIN
  | CHARACTERISTICS
  | CHECK
  | CHECKPOINT
  | CLASS
  | CLOSE
  | CLUSTER
  | COALESCE
  | COLLATE
  | COLLATION
  | COLUMN
  | COLUMNS
  | COMMENT
  | COMMENTS
  | COMMIT
  | COMMITTED
  | COMPRESSION
  | CONCURRENTLY
  | CONDITIONAL
  | CONFIGURATION
  | CONFLICT
  | CONNECTION
  | CONSTRAINT
  | CONSTRAINTS
  | CONTENT_P
  | CONTINUE_P
  | CONVERSION_P
  | COPY
  | COST
  | CROSS
  | CSV
  | CUBE
  | CURRENT_P
  | CURRENT_CATALOG
  | CURRENT_DATE
  | CURRENT_ROLE
  | CURRENT_SCHEMA
  | CURRENT_TIME
  | CURRENT_TIMESTAMP
  | CURRENT_USER
  | CURSOR
  | CYCLE
  | DATA_P
  | DATABASE
  | DEALLOCATE
  | DEC
  | DECIMAL_P
  | DECLARE
  | DEFAULT
  | DEFAULTS
  | DEFERRABLE
  | DEFERRED
  | DEFINER
  | DELETE_P
  | DELIMITER
  | DELIMITERS
  | DEPENDS
  | DEPTH
  | DESC
  | DETACH
  | DICTIONARY
  | DISABLE_P
  | DISCARD
  | DISTINCT
  | DO
  | DOCUMENT_P
  | DOMAIN_P
  | DOUBLE_P
  | DROP
  | EACH
  | ELSE
  | EMPTY_P
  | ENABLE_P
  | ENCODING
  | ENCRYPTED
  | END_P
  | ENFORCED
  | ENUM_P
  | ERROR_P
  | ESCAPE
  | EVENT
  | EXCLUDE
  | EXCLUDING
  | EXCLUSIVE
  | EXECUTE
  | EXISTS
  | EXPLAIN
  | EXPRESSION
  | EXTENSION
  | EXTERNAL
  | EXTRACT
  | FALSE_P
  | FAMILY
  | FINALIZE
  | FIRST_P
  | FLOAT_P
  | FOLLOWING
  | FORCE
  | FOREIGN
  | FORMAT
  | FORWARD
  | FREEZE
  | FULL
  | FUNCTION
  | FUNCTIONS
  | GENERATED
  | GLOBAL
  | GRANTED
  | GREATEST
  | GROUPING
  | GROUPS
  | HANDLER
  | HEADER_P
  | HOLD
  | IDENTITY_P
  | IF_P
  | ILIKE
  | IMMEDIATE
  | IMMUTABLE
  | IMPLICIT_P
  | IMPORT_P
  | IN_P
  | INCLUDE
  | INCLUDING
  | INCREMENT
  | INDENT
  | INDEX
  | INDEXES
  | INHERIT
  | INHERITS
  | INITIALLY
  | INLINE_P
  | INNER_P
  | INOUT
  | INPUT_P
  | INSENSITIVE
  | INSERT
  | INSTEAD
  | INT_P
  | INTEGER
  | INTERVAL
  | INVOKER
  | IS
  | ISOLATION
  | JOIN
  | JSON
  | JSON_ARRAY
  | JSON_ARRAYAGG
  | JSON_EXISTS
  | JSON_OBJECT
  | JSON_OBJECTAGG
  | JSON_QUERY
  | JSON_SCALAR
  | JSON_SERIALIZE
  | JSON_TABLE
  | JSON_VALUE
  | KEEP
  | KEY
  | KEYS
  | LABEL
  | LANGUAGE
  | LARGE_P
  | LAST_P
  | LATERAL_P
  | LEADING
  | LEAKPROOF
  | LEAST
  | LEFT
  | LEVEL
  | LIKE
  | LISTEN
  | LOAD
  | LOCAL
  | LOCALTIME
  | LOCALTIMESTAMP
  | LOCATION
  | LOCK_P
  | LOCKED
  | LOGGED
  | MAPPING
  | MATCH
  | MATCHED
  | MATERIALIZED
  | MAXVALUE
  | MERGE
  | MERGE_ACTION
  | METHOD
  | MINVALUE
  | MODE
  | MOVE
  | NAME_P
  | NAMES
  | NATIONAL
  | NATURAL
  | NCHAR
  | NESTED
  | NEW
  | NEXT
  | NFC
  | NFD
  | NFKC
  | NFKD
  | NO
  | NONE
  | NORMALIZE
  | NORMALIZED
  | NOT
  | NOTHING
  | NOTIFY
  | NOWAIT
  | NULL_P
  | NULLIF
  | NULLS_P
  | NUMERIC
  | OBJECT_P
  | OF
  | OFF
  | OIDS
  | OLD
  | OMIT
  | ONLY
  | OPERATOR
  | OPTION
  | OPTIONS
  | OR
  | ORDINALITY
  | OTHERS
  | OUT_P
  | OUTER_P
  | OVERLAY
  | OVERRIDING
  | OWNED
  | OWNER
  | PARALLEL
  | PARAMETER
  | PARSER
  | PARTIAL
  | PARTITION
  | PASSING
  | PASSWORD
  | PATH
  | PERIOD
  | PLACING
  | PLAN
  | PLANS
  | POLICY
  | POSITION
  | PRECEDING
  | PREPARE
  | PREPARED
  | PRESERVE
  | PRIMARY
  | PRIOR
  | PRIVILEGES
  | PROCEDURAL
  | PROCEDURE
  | PROCEDURES
  | PROGRAM
  | PUBLICATION
  | QUOTE
  | QUOTES
  | RANGE
  | READ
  | REAL
  | REASSIGN
  | RECURSIVE
  | REF_P
  | REFERENCES
  | REFERENCING
  | REFRESH
  | REINDEX
  | RELATIVE_P
  | RELEASE
  | RENAME
  | REPEATABLE
  | REPLACE
  | REPLICA
  | RESET
  | RESTART
  | RESTRICT
  | RETURN
  | RETURNS
  | REVOKE
  | RIGHT
  | ROLE
  | ROLLBACK
  | ROLLUP
  | ROUTINE
  | ROUTINES
  | ROW
  | ROWS
  | RULE
  | SAVEPOINT
  | SCALAR
  | SCHEMA
  | SCHEMAS
  | SCROLL
  | SEARCH
  | SECURITY
  | SELECT
  | SEQUENCE
  | SEQUENCES
  | SERIALIZABLE
  | SERVER
  | SESSION
  | SESSION_USER
  | SET
  | SETOF
  | SETS
  | SHARE
  | SHOW
  | SIMILAR
  | SIMPLE
  | SKIP
  | SMALLINT
  | SNAPSHOT
  | SOME
  | SOURCE
  | SQL_P
  | STABLE
  | STANDALONE_P
  | START
  | STATEMENT
  | STATISTICS
  | STDIN
  | STDOUT
  | STORAGE
  | STORED
  | STRICT_P
  | STRING_P
  | STRIP_P
  | SUBSCRIPTION
  | SUBSTRING
  | SUPPORT
  | SYMMETRIC
  | SYSID
  | SYSTEM_P
  | SYSTEM_USER
  | TABLE
  | TABLES
  | TABLESAMPLE
  | TABLESPACE
  | TARGET
  | TEMP
  | TEMPLATE
  | TEMPORARY
  | TEXT_P
  | THEN
  | TIES
  | TIME
  | TIMESTAMP
  | TRAILING
  | TRANSACTION
  | TRANSFORM
  | TREAT
  | TRIGGER
  | TRIM
  | TRUE_P
  | TRUNCATE
  | TRUSTED
  | TYPE_P
  | TYPES_P
  | UESCAPE
  | UNBOUNDED
  | UNCOMMITTED
  | UNCONDITIONAL
  | UNENCRYPTED
  | UNIQUE
  | UNKNOWN
  | UNLISTEN
  | UNLOGGED
  | UNTIL
  | UPDATE
  | USER
  | USING
  | VACUUM
  | VALID
  | VALIDATE
  | VALIDATOR
  | VALUE_P
  | VALUES
  | VARCHAR
  | VARIADIC
  | VERBOSE
  | VERSION_P
  | VIEW
  | VIEWS
  | VIRTUAL
  | VOLATILE
  | WHEN
  | WHITESPACE_P
  | WORK
  | WRAPPER
  | WRITE
  | XML_P
  | XMLATTRIBUTES
  | XMLCONCAT
  | XMLELEMENT
  | XMLEXISTS
  | XMLFOREST
  | XMLNAMESPACES
  | XMLPARSE
  | XMLPI
  | XMLROOT
  | XMLSERIALIZE
  | XMLTABLE
  | YES_P
  | ZONE
;

__empty :
    __empty
;