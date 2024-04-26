use oxc_ast::{ast::TSEnumMemberName, AstKind};
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::Error,
};
use oxc_macros::declare_oxc_lint;
use oxc_span::{CompactStr, Span};

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error("typescript-eslint(prefer-enum-initializers):The value of the member {0:?} should be explicitly defined.")]
#[diagnostic(severity(warning), help("Can be fixed to {0:?} = {1:?}."))]
struct PreferEnumInitializersDiagnostic(CompactStr, usize, #[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct PreferEnumInitializers;

declare_oxc_lint!(
    /// ### What it does
    /// Require each enum member value to be explicitly initialized.
    ///
    /// ### Why is this bad?
    /// In projects where the value of `enum` members are important, allowing implicit values for enums can cause bugs if enums are modified over time.
    ///
    /// ### Example
    /// ```typescript
    /// // wrong, the value of `Close` is not constant
    /// enum Status {
    ///  Open = 1,
    ///  Close,
    /// }
    /// ```
    PreferEnumInitializers,
    pedantic
);

impl Rule for PreferEnumInitializers {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::TSEnumDeclaration(decl) = node.kind() else { return };

        for (index, member) in decl.members.iter().enumerate() {
            if member.initializer.is_none() {
                if let TSEnumMemberName::Identifier(i) = &member.id {
                    ctx.diagnostic(PreferEnumInitializersDiagnostic(
                        i.name.to_compact_str(),
                        index + 1,
                        member.span,
                    ));
                }
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "
			enum Direction {}
			    ",
        "
			enum Direction {
			  Up = 1,
			}
			    ",
        "
			enum Direction {
			  Up = 1,
			  Down = 2,
			}
			    ",
        "
			enum Direction {
			  Up = 'Up',
			  Down = 'Down',
			}
			    ",
    ];

    let fail = vec![
        "
			enum Direction {
			  Up,
			}
			      ",
        "
			enum Direction {
			  Up,
			  Down,
			}
			      ",
        "
			enum Direction {
			  Up = 'Up',
			  Down,
			}
			      ",
        "
			enum Direction {
			  Up,
			  Down = 'Down',
			}
			      ",
    ];

    Tester::new(PreferEnumInitializers::NAME, pass, fail).test_and_snapshot();
}