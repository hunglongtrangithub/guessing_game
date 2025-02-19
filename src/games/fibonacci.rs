use crate::utils;

pub fn launch() {
    utils::clear_screen();
    println!("Fibonacci Series");
    loop {
        println!("Enter the number of terms, or press Enter to exit:");
        let n = utils::read_input();
        if n.trim().is_empty() {
            break;
        }
        let n = match n.trim().parse::<u8>() {
            Ok(num) => {
                if num > 99 {
                    println!("Please enter a number less than 100");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let mut a: u128 = 0;
        let mut b: u128 = 1;
        let mut i: u8 = 0;
        while i < n {
            print!("{a} ");
            let c = a + b;
            a = b;
            b = c;
            i += 1;
        }
        println!();
    }
}
