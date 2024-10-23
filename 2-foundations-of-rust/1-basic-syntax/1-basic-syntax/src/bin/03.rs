fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    // let mut a = 0;
    // let mut b = 9;

    // for i in input {
    //     if i > a {
    //         a = i;
    //     }
    //     if i < b {
    //         b = i;
    //     }
    // }
    let a = input.iter().max().unwrap();
    let b = input.iter().min().unwrap();
    println!("{} is largest and {} is smallest", a, b);
}
