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
    println!("{}", code.depth());
    let tree = table::Tree::new(code, SetVars::new(&['A', 'B']));
    println!("{tree}");
    println!("\n\n\n\n");
}
