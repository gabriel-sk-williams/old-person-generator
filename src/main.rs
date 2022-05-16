mod macros;
mod op;
use op::OldPerson;

fn main() {
    let _names = macros::intro(); // name prompt
    let gender = macros::input_gender(); // gender prompt

    for _num in 0..10 {
        let op: OldPerson = OldPerson::create(&gender);
        op.describe();
    }
}