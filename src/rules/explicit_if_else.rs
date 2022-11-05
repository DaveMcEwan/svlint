use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitIfElse {
    under_always_construct: bool,
}

impl Rule for ExplicitIfElse {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::AlwaysConstruct(_) => {
                        self.under_always_construct = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::AlwaysConstruct(_) => {
                        self.under_always_construct = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };
        match (self.under_always_construct, node) {
            (true, RefNode::ConditionalStatement(y)) => {
                let (_, ref b, _, _, _, ref f) = &y.nodes;
                let loc = unwrap_locate!(b).unwrap();
                if f.is_none() {
                    RuleResult::FailLocate(*loc)
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_if_else")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`if` must have `else` in `always*`")
    }

    fn reason(&self) -> String {
        String::from("explicit `else` makes design intent clearer")
    }
}
