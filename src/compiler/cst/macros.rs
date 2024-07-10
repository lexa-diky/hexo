#[macro_export] macro_rules! match_ast {
    ($node:expr => $node_expected:ident, $($option:ident => $holder:ident | $transform:expr)+) => {
        guard_node_type($node, AstNodeType::$node_expected)?;

        $(
        let mut $holder = Option::None;
        )+


        for child in $node.children() {
           match child.node_type() {
               $(
                AstNodeType::$option => {
                     guard_empty($holder)?;
                     let parse_result = parse_value_of(child)?;
                     let transformed = $transform(parse_result)?;
                     $holder = Some(transformed);
                }
                )+
               _ => {
                   return Err(Error::UnexpectedNode {
                       actual: child.node_type(),
                       expected: vec![
                            $(
                                 AstNodeType::$option,
                            )+
                       ],
                   })
               }
           }
        }

        $(
        let $holder = $holder.ok_or(Error::MissingContent {
            node_type: AstNodeType::$option,
        })?;
        )+
    };
}
