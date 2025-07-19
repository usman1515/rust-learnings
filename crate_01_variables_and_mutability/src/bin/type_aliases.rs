// type aliases
type Km = f32;

fn main() {
    println!("========== type_aliases ==========");

    let marathon: Km = 42.195;
    let half_marathon: Km = marathon / 2.0;

    println!("A marathon is {} km long.", marathon);
    println!("A half marathon is {} km long.", half_marathon);
    println!("A half marathon is {} km long.", marathon / 2.0);
}
