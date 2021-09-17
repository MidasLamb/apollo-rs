use crate::parser::grammar::{directive, operation};
use crate::{create_err, Parser, SyntaxKind, S, T};

/// See: https://spec.graphql.org/June2018/#SchemaDefinition
///
/// ```txt
/// SchemaDefinition
///    schema Directives { OperationTypeDefinition }
/// ```
pub(crate) fn schema_definition(p: &mut Parser) {
    let _guard = p.start_node(SyntaxKind::SCHEMA_DEFINITION);
    p.bump(SyntaxKind::schema_KW);

    if let Some(T![@]) = p.peek() {
        directive::directives(p);
    }

    if let Some(T!['{']) = p.peek() {
        p.bump(S!['{']);
        operation::operation_type_definition(p, false);
        if let Some(T!['}']) = p.peek() {
            p.bump(S!['}']);
        } else {
            p.push_err(create_err!(
                p.peek_data().unwrap(),
                "Expected Schema Definition to have a closing curly bracket, got {}",
                p.peek_data().unwrap()
            ));
        }
    } else {
        p.push_err(create_err!(
            p.peek_data().unwrap(),
            "Expected Schema Definition to define a root operation, got {}",
            p.peek_data().unwrap()
        ));
    }
}

/// See: https://spec.graphql.org/June2018/#SchemaExtension
///
/// ```txt
/// SchemaExtension
///     extend schema Directives[Const][opt] { OperationTypeDefinition[list] }
///     extend schema Directives[Const]
/// ```
pub(crate) fn schema_extension(p: &mut Parser) {
    let _guard = p.start_node(SyntaxKind::SCHEMA_EXTENSION);
    p.bump(SyntaxKind::extend_KW);
    p.bump(SyntaxKind::schema_KW);

    if let Some(T![@]) = p.peek() {
        directive::directives(p);
    }

    if let Some(T!['{']) = p.peek() {
        p.bump(S!['{']);
        operation::operation_type_definition(p, false);
        if let Some(T!['}']) = p.peek() {
            p.bump(S!['}']);
        } else {
            p.push_err(create_err!(
                p.peek_data().unwrap(),
                "Expected Schema Extension to have a closing }} following Operation Type Definitions, got {}",
                p.peek_data().unwrap()
            ));
        }
    }
}

// TODO @lrlna: inlined collapsed AST should live in a 'fixtures' dir for ease of testing
#[cfg(test)]
mod test {
    use crate::parser::utils;

    #[test]
    fn it_parses_schema_definition() {
        utils::check_ast(
            "schema {
                query: MyQueryRootType
                mutation: MyMutationRootType,
                subscription: MySubscriptionRootType
            }",
            r#"
            - DOCUMENT@0..92
                - SCHEMA_DEFINITION@0..92
                    - schema_KW@0..6 "schema"
                    - L_CURLY@6..7 "{"
                    - OPERATION_TYPE_DEFINITION@7..28
                        - OPERATION_TYPE@7..12
                            - query_KW@7..12 "query"
                        - COLON@12..13 ":"
                        - NAMED_TYPE@13..28
                            - NAME@13..28
                                - IDENT@13..28 "MyQueryRootType"
                    - OPERATION_TYPE_DEFINITION@28..55
                        - OPERATION_TYPE@28..36
                            - mutation_KW@28..36 "mutation"
                        - COLON@36..37 ":"
                        - NAMED_TYPE@37..55
                            - NAME@37..55
                                - IDENT@37..55 "MyMutationRootType"
                    - COMMA@55..56 ","
                    - OPERATION_TYPE_DEFINITION@56..91
                        - OPERATION_TYPE@56..68
                            - subscription_KW@56..68 "subscription"
                        - COLON@68..69 ":"
                        - NAMED_TYPE@69..91
                            - NAME@69..91
                                - IDENT@69..91 "MySubscriptionRootType"
                    - R_CURLY@91..92 "}"
            "#,
        );
    }

    #[test]
    fn it_parses_schema_extension() {
        utils::check_ast(
            "extend schema @skip @example {
                query: MyExtendedQueryType
            }",
            r#"
            - DOCUMENT@0..52
                - SCHEMA_EXTENSION@0..52
                    - extend_KW@0..6 "extend"
                    - schema_KW@6..12 "schema"
                    - DIRECTIVES@12..25
                        - DIRECTIVE@12..17
                            - AT@12..13 "@"
                            - NAME@13..17
                                - IDENT@13..17 "skip"
                        - DIRECTIVE@17..25
                            - AT@17..18 "@"
                            - NAME@18..25
                                - IDENT@18..25 "example"
                    - L_CURLY@25..26 "{"
                    - OPERATION_TYPE_DEFINITION@26..51
                        - OPERATION_TYPE@26..31
                            - query_KW@26..31 "query"
                        - COLON@31..32 ":"
                        - NAMED_TYPE@32..51
                            - NAME@32..51
                                - IDENT@32..51 "MyExtendedQueryType"
                    - R_CURLY@51..52 "}"
            "#,
        );
    }
}