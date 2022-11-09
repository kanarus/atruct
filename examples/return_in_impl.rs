use atruct::withReturn;

fn main() {
    let abc = T::get_abc();
    println!("abc: {{a: {}, b: {}, c: {:?}}}", abc.a, abc.b, abc.c);
    // abc: {a: 0, b: I am b, c: [1, 0, -1, 0]}
}

struct T;
#[withReturn]
impl T {
    #[Return(a: u8, b: String, c: Vec<isize>)]
    fn get_abc() {
        Return {
            a: 0,
            b: "I am b".into(),
            c: vec![1, 0, -1, 0],
        }
    }
}