// without this file 'build.rs', lalrpop will raise error

fn main() {
    lalrpop::process_root().unwrap();
}