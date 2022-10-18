pub fn type_of_num<Num: std::ops::Add>(num: Num) -> &'static str {
    std::any::type_name::<Num>()
}