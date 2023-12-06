use lalrpop_util::lalrpop_mod;

use crate::ast::{Variable::{*, self}, Sentence::*, Operator::*};

lalrpop_mod!(pub parser);

#[test]
fn test_parser() {
    let parser = parser::SentenceParser::new();

    vec![
        ("LABEL label1 :", Label("label1")),
        ("FUNCTION mod :", Func("mod")),
        ("vcnt := #0", Assign { target: Id("vcnt"), var: Number(0) }),
        ("*t181 := vi", Assign { 
            target: Deref("t181"), 
            var: new_id("vi")
        }),
        ("t107 := vt1 * vt2", Arith { 
            l: new_id("vt1"), 
            r: new_id("vt2"), 
            opt: Mul, 
            target: new_id("t107") }),
        ("t165 := &varray + t162", Arith { 
            l: Pointer("varray"), 
            r: new_id("t162"), 
            opt: Plus, 
            target: new_id("t165") 
        }),
        ("t157 := vsum + #1", Arith { 
            l: new_id("vsum"), 
            r: Number(1), 
            opt: Plus, 
            target: new_id("t157") 
        }),
        ("GOTO label1", Goto("label1")),
        ("IF vcnt < vk GOTO label2", IfGoto { 
            l: new_id("vcnt"), 
            r: new_id("vk"), 
            opt: Less, 
            label: ("label2") 
        }),
        ("RETURN #0", Return(Number(0))),
        ("DEC varray 40", Dec{ target: new_id("varray"), size: 40}),
        ("t161 := CALL mod", Call { target: new_id("t161"), func: ("mod") })

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
    Id(id)
}
