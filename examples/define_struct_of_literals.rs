use atruct::atruct;

fn main() {
    let anonymous = atruct!(
        integer2: -5,
        float2: -3.14,
        integer1: 0usize,
        float1: 3.14,
        nest: {
            string: "literal",
            boolean: true,
        }
    );

    println!("{}", anonymous.integer1);  // 0
    println!("{}", anonymous.float1);  // 3.14
    println!("{}", anonymous.nest.string);  // literal
}