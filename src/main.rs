#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub mod ast;

fn main() {
    println!("Hello, world!");
}

#[macro_export]
macro_rules! test_check {
    ($lhs:expr, $rhs:expr, $err:ident) => {
        let expr = grammar::ExprsParser::new().parse(&mut $err, $lhs).unwrap();
        assert_eq!(&format!("{:?}", expr), $rhs);
    };
}

#[test]
fn test() {
    let mut errors = Vec::new();

    test_check!("", "[]", errors);
    test_check!("22", "[22]", errors);
    test_check!("(22)", "[22]", errors);
    test_check!("(((22)", "[error]", errors);
    test_check!("22))", "[error]", errors);

    test_check!("22 * 44 + 66", "[((22 * 44) + 66)]", errors);
    test_check!("22 + 44 * 66,", "[(22 + (44 * 66))]", errors);
    test_check!("(22+44) * 66, 13*3", "[((22 + 44) * 66), (13 * 3)]", errors);
    test_check!(
        "22 * 44, 66, 13 * 3, ,,",
        "[(22 * 44), 66, (13 * 3), error, error]",
        errors
    );
    test_check!("22 * + 3", "[((22 * error) + 3)]", errors);
    test_check!(
        "22 * 44 + 66, *3",
        "[((22 * 44) + 66), (error * 3)]",
        errors
    );
    test_check!(" % ", "[(error % error)]", errors);

    test_check!("1 & 2 + 3", "[(error + 3)]", errors);
    test_check!("1 & (2 + 3)", "[(1 & (2 + 3))]", errors);
    test_check!("2 << 4 / 6", "[(error / 6)]", errors);
    test_check!("(2 << 4) / 6", "[((2 << 4) / 6)]", errors);
    test_check!("2 << 4 / 6", "[(error / 6)]", errors);
    test_check!(
        "3 * 6 + 9 == 12 & 15",
        "[(((3 * 6) + 9) == (12 & 15))]",
        errors
    );
    test_check!("4 && (!8 || 12)", "[(4 && ((! 8) || 12))]", errors);
    test_check!("5 + 10 < !15 | 20", "[((5 + 10) < ((! 15) | 20))]", errors);

    test_check!("1 && 2 && 3", "[((1 && 2) && 3)]", errors);
    test_check!("1 || 2 || 3", "[((1 || 2) || 3)]", errors);
    test_check!("1 & 2 & 3", "[((1 & 2) & 3)]", errors);
    test_check!("1 | 2 | 3", "[((1 | 2) | 3)]", errors);
}
