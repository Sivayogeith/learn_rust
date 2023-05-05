use human::human::Human;
pub mod human;

fn main() {
    let yogi: Human = Human {
        name: "Sivayogeith",
        last_name: "Umamaheswaran",
        full_name: "Sivayogeith Umamaheswaran",
        age: &11,
        family: &human::human::Family {
            sister: "Sivaswatha",
            mother: "Chitra",
            father: "Umamaheswaran",
        },
    };
    println!("Hi, My Name is {0}, I am {1} years old. My Father's name is {2}, My Mother's name is {3}, My Sister's name is {4}", yogi.full_name, yogi.age, yogi.family.father, yogi.family.mother, yogi.family.sister);
}
