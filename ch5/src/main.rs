//Zlatopolsky tasks book. Chapter 4.
use rand::Rng;

fn task_5_1(){
    println!("Task 5.1");
    for _x in 1..10 {print!("20 ")};
    println!();
    println!();
}


fn task_5_3(){
    println!("Task 5.3");
    for x in 20..26 {println!("{}", x)};
    println!();
}


fn task_5_4(){
    println!("Task 5.4");
    let mut a: u8 = 10;
    let mut b: f64 = 10.4;
    loop {
        println!("{}  {}", a, b);
        a += 1;
        b += 1.0;
        if a == 26 || b == 26.4 {break};
    }
    println!();
}


fn task_5_5(){
    println!("Task 5.5");
    let mut a: u8 = 21;
    let mut b: f64 = 19.2;
    while a != 9 && b != 7.2 {
        println!("{}  {}", a, b);
        a -= 1;
        b -= 1.0;
    }
    println!();
}


fn task_5_17(){
    println!("Task 5.17");
    for x in 4..28 {
        let t: f64 = x as f64 + 2.0;
        println!("For x = {}, y = {res:.2} ", x, res = {2.0 * t.sqrt() + 5.5 * t - 2.0});
    }
    println!();
}


fn task_5_27(){
    println!("Task 5.27");
    let mut x: u32 = 100;
    for counter in 100..501 {x = x + counter};
    println!("Sum of numbers in range 100-500 is: {}", x);
    println!();
}


fn task_5_31(){
    println!("Task 5.31");
    let mut rand_num = rand::thread_rng();
    let n: u8 = rand_num.gen_range(3..8);
    let mut counter: u8 = 0;
    let mut sum: u8 = 0;
    loop {
        sum += (n + counter).pow(2);
        counter += 1;
        if sum > 2 * n.pow(2) {break};
    }
    println!("Random nuber = {}, sum = {}", n, sum);
    println!();
}


//need to finish this
fn task_5_67(){
    println!("Task 5.67");
    let mut rand_num = rand::thread_rng();
    let n: u8 = rand_num.gen_range(3..8);
    println!("N = {}, fib number = ", n);
}


//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 5.");
    println!();
    task_5_1();
    task_5_3();
    task_5_4();
    task_5_5();
    task_5_17();
    task_5_27();
    task_5_31();
    task_5_67();
    println!("End of tasks.");
}
