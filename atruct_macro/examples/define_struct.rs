use atruct::atruct;

fn main() {
    let anonymous = atruct!(
        integer1: 0,
        integer2: -5,
        float: 3.14,
        nest: {
            string: "literal",
            boolean: true,
        }
    );

    println!("{}", anonymous.integer1);  // 0
    println!("{}", anonymous.float);  // 3.14
    println!("{}", anonymous.nest.string);  // literal
}