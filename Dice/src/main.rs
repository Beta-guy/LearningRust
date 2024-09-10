use rand::{thread_rng, Rng};
use read_stdin;

fn main() {
    let Ok(num_o_num) = read_stdin::prompt::<i32>("Enter a number: ") else {
        println!("failed to parse number!");
        return;
    };
    let Ok(ref mut num_o_dice) = read_stdin::prompt::<i32>("How many dice? ") else {
        println!("failed to parse number!");
        return;
    };
    while *num_o_dice != 0{
        let num = thread_rng().gen_range(1..=num_o_num);
        println!("{}", num);
        *num_o_dice -= 1;
    }
}
