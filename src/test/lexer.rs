use lalrpop_util::lalrpop_mod;

use crate::ast::Variable;

lalrpop_mod!(pub parser);



#[test]
fn test_label() {
    let parser = parser::LabelParser::new();

    assert_eq!(parser.parse("va").unwrap(), String::from("va"));
}

#[test]
#[should_panic]
fn test_label_error() {
    let parser = parser::LabelParser::new();
    parser.parse("123").unwrap();
}

#[test]
fn test_number() {
    let parser = parser::VariableParser::new();
    
    vec![
        ("#1", Variable::Number(1)),
        ("#321", Variable::Number(321)),
        ("#89", Variable::Number(89)),
        ("&a", new_pointer("a")),
        ("&zdcs",new_pointer("zdcs")),
        ("&sa", new_pointer("sa")),
        ("*a", new_deref("a")),
        ("*zdcs",new_deref("zdcs")),
        ("*sa", new_deref("sa")),
    ].iter().enumerate().for_each(|(_i, case)|{
        assert_eq!(parser.parse(case.0).unwrap(), case.1)
    });
}

fn new_pointer(pointer: &str) -> Variable {
    Variable::Pointer(String::from(pointer))
}

fn new_deref(pointer: &str) -> Variable {
    Variable::Deref(String::from(pointer))
}
