use lalrpop_util::lalrpop_mod;

use crate::ast::{Variable::{*, self}, Sentence::*, Operator::*};

lalrpop_mod!(pub parser);

#[test]
fn test_parser() {
    let parser = parser::SentenceParser::new();

    vec![
        ("LABEL label1 :", Label(String::from("label1"))),
        ("FUNCTION mod :", Func(String::from("mod"))),
        ("vcnt := #0", Assign { l: Id(String::from("vcnt")), r: Number(0) }),
        ("*t181 := vi", Assign { 
            l: Deref(String::from("t181")), 
            r: new_id("vi")
        }),
        ("t107 := vt1 * vt2", Arth { 
            l: new_id("vt1"), 
            r: new_id("vt2"), 
            opt: Mul, 
            target: new_id("t107") }),
        ("t165 := &varray + t162", Arth { 
            l: Pointer(String::from("varray")), 
            r: new_id("t162"), 
            opt: Plus, 
            target: new_id("t165") 
        }),
        ("t157 := vsum + #1", Arth { 
            l: new_id("vsum"), 
            r: Number(1), 
            opt: Plus, 
            target: new_id("t157") 
        }),
        ("GOTO label1", Goto(String::from("label1"))),
        ("IF vcnt < vk GOTO label2", IfGoto { 
            l: new_id("vcnt"), 
            r: new_id("vk"), 
            opt: Less, 
            target: String::from("label2") 
        }),
        ("RETURN #0", Return(Number(0))),
        ("DEC varray 40", Dec{ var: new_id("varray"), size: 40}),
        ("t161 := CALL mod", Call { var: new_id("t161"), func: String::from("mod") })

    ].iter().for_each(|case|{
        let parser_result = match parser.parse(case.0) {
            Ok(r) => r,
            Err(e) => {
                panic!("case: \"{}\"\n{}", case.0, e)
            }
        };
        assert_eq!(parser_result, case.1)
    });
}

fn new_id(id: &str) -> Variable {
    Id(String::from(id))
}
