use atruct::atruct;
use std::collections::HashMap;

fn main() {
    let anonymous = atruct!(
        // Type annotaion is needed for each non-literal value.
        // There are 2 options to annotate type:
        string1 @ String: String::from("string1"),  // @ pattern and
        string2(String): String::from("string2"),  // () pattern.
        // Their behaviors are completely the same. Use any one you like!
        box_option_vec(Box<Option<Vec<u8>>>): Box::new(Some(Vec::new())),
        hash @ HashMap<u8, u8>: HashMap::from([]),
        vec(Vec<u8>): vec![0, 1, 0, 1, 1],
        nest: {
            a: "literals don't need type annotation",
            b: 100usize,  // unlike v0.1, type suffix is supported for integers!
        },
    );

    println!("{}", anonymous.string1);  // string1
    println!("{}", anonymous.string2);  // string2
    println!("{:?}", anonymous.box_option_vec);  // Some([])
    println!("{:?}", anonymous.hash);  // {}
    println!("{:?}", anonymous.vec);  // [0, 1, 0, 1, 1]
}