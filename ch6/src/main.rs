//Zlatopolsky tasks book. Chapter 6.
use rand::Rng;


fn task_6_1(){
    println!("Task 6.1");
    let list: [u8; 6] = [1, 2, 3, 4, 5, 0];
    let mut sum: u8 = 0;
    for i in 0..list.len(){
        sum += list[i];
    }
    println!("List of numbers: {:?}", list);
    println!("Sum of all numbers in array = {}", sum);
    println!("Quantity of numbers in array = {}", list.len());
    println!();
}


fn task_6_2(){
    println!("Task 6.2");
    let list: [i8; 6] = [1, 2, 3, 4, 5, -1];
    let mut sum: i8 = 0;
    let mut counter: u8 = 0;
    for i in 0..list.len(){
        if list[i] > 0 {sum += list[i]; counter += 1;}
    };
    println!("List of numbers: {:?}", list);
    println!("Average of not negative numbers = {avg:.2}", avg = {sum as f64/ counter as f64});
    println!();
}


fn task_6_8_9(){
    println!("Task 6.8-9");
    let mut n: u32 = 100;

    println!("All numbers n^2 that lower than {}:", n);
    for i in 1..n {
        if i * i < n {print!("{}, ", i * i)}
    }
    println!("\n");

    println!("First number that bigger than {}:", n);
    for i in 1..n {
        if i * i > n {print!("{}", i * i); break}
    }
    println!("\n");

}



fn task_6_22(){
    println!("Task 6.22");
    let mut rand_num= rand::thread_rng();
    let n: u16 = rand_num.gen_range(1000..10_000);
    let thousands: u16 = n / 1000;
    let hundreds: u16 = n % 1000 / 100;
    let tens: u16 = n % 100 / 10;
    let ones: u16 = n % 10;
    let list:[u16; 4] = [thousands, hundreds, tens, ones];

    let mut q3: u8 = 0;
    for i in 0..list.len() {if list[i] == 3 {q3 += 1};}
    println!("a) Quantity of 3 in number {} = {}", n, q3);

    let mut qlast: u8 = 0;
    for i in 0..list.len(){if list[i] == list[3] {qlast += 1;}};
    println!("b) Quantity of last number {} in number {} = {}", list[3], n, qlast);
    println!();

}


//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 6.");
    println!();
    task_6_1();
    task_6_2();
    task_6_8_9();
    task_6_22();
    println!("End of tasks.");
}
