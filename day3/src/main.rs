mod part1;
fn main() {
    // part1::part1();
    let a = "..3.";
    let b = a.split('.').collect::<Vec<&str>>();
    println!("{:?}", b);
}
