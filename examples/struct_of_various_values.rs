use atruct::atruct;

fn main() {
    let anonymous = atruct!(
        // Type annotaion is needed for each non-literal value.
        // There are 2 options to annotate type:

        string1 @ String: String::from("string1"),  // @ pattern and
        string2(String): String::from("string2"),  // () pattern.
        // Their behaviors are completely the same. Use any one you like!

        box_option_vec @ Box<Option<Vec<u8>>>: Box::new(Some(Vec::new())),
        vec(Vec<u8>): vec![0, 1, 0, 1, 1],
        
        nest: {
            a: "you can define nested struct without prepare lots of named structs",
            b: 100usize,  // literals don't need type annotation
        },
    );

    println!("{}", anonymous.string1);  // string1
    println!("{}", anonymous.string2);  // string2
    println!("{:?}", anonymous.box_option_vec);  // Some([])
    println!("{:?}", anonymous.vec);  // [0, 1, 0, 1, 1]
    println!("{}", anonymous.nest.a)  // you can define nested struct without prepare lots of named structs
}