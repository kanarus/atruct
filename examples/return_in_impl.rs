use atruct::withReturn;

fn main() {
    let abc = T::get_abc();
    println!("abc: {{a: {}, b: {}, c: {:?}}}", abc.a, abc.b, abc.c);
    // abc: {a: 0, b: string, c: [1, 0, -1, 0]}
}

struct T;
#[withReturn]
impl T {
    #[Return(a: u8, b: String, c: Vec<isize>)]
    fn get_abc() {
        Return {
            a: 0,
            b: "string".into(),
            c: vec![1, 0, -1, 0],
        }
    }
}