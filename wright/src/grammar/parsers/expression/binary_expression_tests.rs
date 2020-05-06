use crate::grammar::parsers::testing::{test_should_succeed};
use crate::grammar::ast::{BinaryExpression, Expression};
use crate::grammar::model::Fragment;
use nom::IResult;

fn p(f: Fragment) -> IResult<Fragment, Expression> {
    BinaryExpression::parse(f)
}

#[test]
fn test_binary_expr_simple<'a>() {
    test_should_succeed::<Expression<'a>, _>(
        p,
        "2 + 2",
        "",
        |expr| {
            true
        }
    )
}
