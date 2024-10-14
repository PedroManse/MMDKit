use mmdkit::*;

fn main() {
    let mvar = |n| Var{var_name: n};
    use Op::*;
    use Value::*;
    let code = 
        Or(
            Xor(
                mvar('A'),
                mvar('B'),
            ).expr(),
            And(
                Not(mvar('B')).expr(),
                Not(mvar('A')).expr(),
            ).expr(),
        ).expr();
    let table = table::Table::new(code, VarPlex::new(&['A', 'B']));
    println!("{table}");
    println!("\n\n\n\n");
}
