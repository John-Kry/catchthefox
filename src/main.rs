mod farm;
mod den;

fn main() {
    let mut farm = farm::Farm::new();
    let mut i = 0;
    loop {
        if i > 10_000 {
            break;
        }
        if farm.check(2) ||
        farm.check(1) ||
        farm.check(2) ||
        farm.check(3) ||
        farm.check(1) ||
        farm.check(2) ||
        farm.check(3) {
            println!("Found fox at {}", farm.fox_idx())
        }else{
            panic!("You failed!")
        }
        farm.mov();
        i += 1;
    }
    println!("you win!");
    println!("idx {}", farm.fox_idx())
}
