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

    impl Human<'_> {
        pub fn eat(&self) {
            println!("I, {0} is eating!", self.full_name)
        }

        pub fn poo(&self) {
            println!("I, {0} is pooing!", self.full_name)
        }

        pub fn sleep(&self) {
            println!("I, {0} is sleeping!", self.full_name)
        }

        pub fn pee(&self) {
            println!("I, {0} is peeing!", self.full_name)
        }
    }
}
