### atruct
provides following 2 macros:

- `atruct!`
- `#[Return]`

they are **independent** of each other.

<br/>
<br/>

# atruct!
inspired by [structx](https://github.com/oooutlk/structx) (that doesn't work now), enables to define **anonymous struct**s like

```rs
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
```
( examples/define_struct.rs )

<br/>

As you see, atruct supports nested structs.

**NOTICE**: Current atruct supports **only literal**s as values. Additional supports are in progress...

<br/>
<br/>

# #[Return]
We usually return more than 1 values from a function. In such situations, Rust supports only **tupple** as a way to bundle returned values. But it's sometimes a bit anoying: when we'd like to name freely to each field, not `0`, `1`, `2`, ...

`#[Return]` enables this naming. You can write functions like

```rs
use atruct::Return;

fn main() {
    let abc = get_abc();
    println!("{}", abc.a);  // 24
    println!("{}", abc.b);  // you can use any type for a field
    println!("{:?}", abc.c);  // [-1, 0, 0, -1, 1, 0, 1, -1]
}

#[Return(a: u8, b: String, c: Vec<isize>)]  // not supporting nest
fn get_abc() {
    Return {
        a: 24,
        b: "you can use any type in a field".into(),
        c: vec![-1,0,0,-1,1,0,1,-1],
    }
}
```
( examples/return_struct.rs )

<br/>

- Unlike `atruct!`, `#[Return]` doesn't support nested structs (for a technical reason). So returned value is just like **a tupple you can give any names to its fields**.
- `#[Return]` automatically generates a struct named as "FunctionName" ( if function is `get_abc`, for example, `GetAbc` ), But at the same time defines a type synonym `Return`. So you **DON't have to** memorize or write the generated struct's name.