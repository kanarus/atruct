atruct provides following 2 macros about anonymous struct:

- `atruct!`
- `#[Return]`

they are independent of each other.

<br/>
<br/>

# atruct!
inspired by [structx](https://github.com/oooutlk/structx) (that doesn't work now), `atruct!` macro enables to define **anonymous struct**s like

```rs
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
```
( examples/struct_of_various_values.rs )

<br/>

atruct supports **nested structs**.

**NOTICE**: When atruct was v0.1 only literals are supported as values, BUT in v0.2 you can use (maybe) **all value**s!

<br/>
<br/>

# #[Return]
We usually return more than 1 values from a function. In such situations, Rust supports only **tupple** as a way to bundle returned values. But it's sometimes a bit anoying: when we'd like to name freely to each field, not `0`, `1`, `2`, ...

`#[Return]` attribute enables such naming. You can write functions like

```rs
use atruct::Return;

fn main() {
    let abc = get_abc();
    println!("{}", abc.a);  // 24
    println!("{}", abc.b);  // you can use any value in a field
    println!("{:?}", abc.c);  // [-1, 0, 0, -1, 1, 0, 1, -1]
}

#[Return(a: u8, b: String, c: Vec<isize>)]  // not supporting nest
fn get_abc() {
    Return {
        a: 24,
        b: "you can use any value in a field".into(),
        c: vec![-1,0,0,-1,1,0,1,-1],
    }
}
```
( examples/return_struct.rs )

<br/>

- Unlike `atruct!`, `#[Return]` doesn't support nested structs. So you can use returned value just like **a tupple you can give any names to its fields**.
- `#[Return]` automatically generates a struct named as "FunctionName" ( e.g. if function is `get_abc`, for example, `GetAbc` ), but at the same time defines a type synonym `Return`. So you **DON't need to** memorize the generated struct's name.