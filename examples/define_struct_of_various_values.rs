use atruct::atruct;
use std::collections::HashMap;

fn main() {
    let anonymous = atruct!(
        string1 @ String: String::from("string1"),
        string2(String): String::from("string2"),
        box_option_vec(Box<Option<Vec<u8>>>): Box::new(Some(Vec::new())),
        hash @ HashMap<u8, u8>: HashMap::from([]),
        vec(Vec<u8>): vec![0, 1, 0, 1, 1],
    );

    println!("{}", anonymous.string1);  // string1
    println!("{}", anonymous.string2);  // string2
    println!("{:?}", anonymous.box_option_vec);  // Some([])
    println!("{:?}", anonymous.hash);  // {}
    println!("{:?}", anonymous.vec);  // [0, 1, 0, 1, 1]
}