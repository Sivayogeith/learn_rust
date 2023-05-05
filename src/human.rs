pub mod human {
    pub struct Family<'a> {
        pub sister: &'a str,
        pub mother: &'a str,
        pub father: &'a str,
    }
    pub struct Human<'a> {
        pub name: &'a str,
        pub last_name: &'a str,
        pub full_name: &'a str,
        pub age: &'a i64,
        pub family: &'a Family<'a>,
    }
}
