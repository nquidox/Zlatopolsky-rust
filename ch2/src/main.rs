//Zlatopolsky tasks book. Chapter 2.
use rand::Rng;

fn task_2_1(){
    println!("Task 2.1");
    let mut rand_num = rand::thread_rng();
    let centimeters: u16 = rand_num.gen_range(100..2000);
    let meters: u16 = centimeters / 100;
    println!("Centimeters: {}, full meters: {}", centimeters, meters);
    println!();
}


fn task_2_6(){
    println!("Task 2.2");
    let mut rand_num = rand::thread_rng();
    let seconds: u16 = rand_num.gen_range(4_000..12_000);
    let hours: u16 = seconds / 3600;
    let minutes: u16 = seconds % 3600 / 60;
    let sec: u16 = seconds % 3600 % 60;
    println!("Seconds: {}", seconds);
    println!("Full hours: {}, minutes since hour start: {}, seconds since minute start: {}",
             hours, minutes, sec);
    println!();
}


fn task_2_12() {
    println!("Task 2.12");
    let mut rand_num = rand::thread_rng();
    let number: u16 = rand_num.gen_range(100..900);
    let ones: u16 = number % 100 % 10;
    let tens: u16 = number % 100 / 10;
    let hundreds: u16 = number / 100;
    println!("Number of ones in {}: {}", number, ones);
    println!("Number of tens in {}: {}", number, tens);
    println!("Number of hundreds in {}: {}", number, hundreds);
    println!("Sum of all numbers in {}: {sum}", number, sum = {hundreds + tens + ones});
    println!("Multiply all numbers in {}: {sum}", number, sum = {hundreds * tens * ones});
    println!();
}


fn task_2_13(){
    println!("Task 2.13");
    let mut rand_num = rand::thread_rng();
    let number: u16 = rand_num.gen_range(100..900);
    let ones: u16 = number % 100 % 10;
    let tens: u16 = number % 100 / 10;
    let hundreds: u16 = number / 100;
    //numbers like 100 will be written as 1 without zeroes at the begging
    let backwards_number: u16 = (ones * 100) + (tens * 10) + (hundreds);
    println!("Random number: {}, written backwards: {}", number, backwards_number);
    println!();

}


fn task_2_18(){
    println!("Task 2.18");
    let number: u16 = 123;

    let hundreds: u16 = number / 100;
    let tens: u16 = number % 100 / 10;
    let ones: u16 = number % 100 % 10;

    let n2: u16 = (hundreds * 100) + (ones * 10) + (tens);
    let n3: u16 = (tens * 100) + (hundreds * 10) + (ones);
    let n4: u16 = (tens * 100) + (ones * 10) + (hundreds);
    let n5: u16 = (ones * 100) + (hundreds * 10) + (tens);
    let n6: u16 = (ones * 100) + (tens * 10) + (hundreds);
    println!("All variants with numbers 1, 2, 3: {}, {}, {}, {}, {}, {}",
             number, n2, n3, n4, n5, n6);
    println!();

}


fn task_2_24(){
    println!("Task 2.24");
    let x: u16 = 237;
    let y: u16 = (x % 100) * 10 + (x / 100);
    println!("Number : {}", y);
    println!()

}


fn task_2_43(){
    println!("Task 2.43");
    let mut rand_num = rand::thread_rng();
    let a: u8 = rand_num.gen_range(1..11);
    let b: u8 = rand_num.gen_range(1..11);
    let ab: u8 = a / b;
    let ba: u8 = b / a;
    let check:u8 = (ab * ba) + 1;
    println!("Numbers a = {} anf b = {}, if this = 1: {}, then numbers divide.", a, b, check);
    println!();

}


//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 2.");
    println!();
    task_2_1();
    task_2_6();
    task_2_12();
    task_2_13();
    task_2_18();
    task_2_24();
    task_2_43();
    println!("End of tasks.");
}
