use std::collections::HashMap;

use bool_expr_parser_nom::ast::Atom;
use bool_expr_parser_nom::{self, eval::eval, parse::parse};
use chrono::NaiveDate;

/// library tests for public functions to parse and then evaluate

#[test]
fn test_parsing() {}

#[test]
fn test_evaluation() {
    let rule = r###"accountRole in (Admin,admin,"Admin/Order Manager")
    and upper(account_country_code) in (LT , NL, DE, GB, US)
    and account_uuid in ("543987b0-e69f-41ec-9a68-cfc5cfb15afe", "6133b8d6-4078-4270-9a68-fa0ac78bf512")
    and accountType in ("Some Corporate & Managament Type", Corporate , Managament)
    and user_id <= 2032313"###;

    let context = HashMap::from([
        ("accountRole", Atom::String("Admin/Order Manager".into())),
        ("account_country_code", Atom::String("lt".into())),
        (
            "account_uuid",
            Atom::String("543987b0-e69f-41ec-9a68-cfc5cfb15afe".into()),
        ),
        (
            "accountType",
            Atom::String("Some Corporate & Managament Type".into()),
        ),
        ("user_id", Atom::Number(2032312)),
    ]);

    let (i, expr) = parse(&rule).unwrap();
    let val = eval(&expr, &context).unwrap();
    assert_eq!(val, true);
    assert_eq!(i, ""); // empty remainder of parsed string
}

#[test]
fn scoped_test_case() {
    let rule = r###"accountRole in (Admin, "Admin/Order Manager") and
    ((lower(account_country_code) == lt or account_uuid = 32434) and accountType="Some Corporate & Managament Type") and user_id == 2032312"###;

    let context = HashMap::from([
        ("accountRole", Atom::String("Admin/Order Manager".into())),
        ("account_country_code", Atom::String("LT".into())),
        (
            "account_uuid",
            Atom::String("543987b0-e69f-41ec-9a68-cfc5cfb15afe".into()),
        ),
        (
            "accountType",
            Atom::String("Some Corporate & Managament Type".into()),
        ),
        ("user_id", Atom::Number(2032312)),
    ]);

    let (i, expr) = parse(&rule).unwrap();
    let val = eval(&expr, &context).unwrap();
    assert_eq!(val, true);
    assert_eq!(i, ""); // empty remainder of parsed string
}
