//Zlatopolsky tasks book. Chapter 4.
use rand::Rng;

fn task_4_1(){
    println!("Task 4.1");
    let mut rand_num = rand::thread_rng();
    let x: f64 = rand_num.gen_range(-10.0..10.0);
    println!("X = {:.2}", x);
    let mut y: f64 = 0.0;
    if x > 0.0 {y = x.sin().powi(2); println!("X > 0.0, Y = {:.3}", y)}
    else {y = 1.0 - 2.0 * x.powi(2).sin(); println!("Else if X is not > 0.0, Y = {:.3}", y)};
    println!();
}


fn task_4_6(){
    println!("Task 4.6");
    let mut rand_num = rand::thread_rng();
    let x: f64 = rand_num.gen_range(1.0..11.0);
    let y: f64 = rand_num.gen_range(1.0..11.0);
    if x > y {println!("X = {:.3} is bigger than Y = {:.3}", x, y)}
    else if  x < y {println!("X = {:.3} is lower than Y = {:.3}", x, y)}
    else {println!("X = {:.3} and Y = {:.3} are equal", x, y)};
    println!();
}


fn task_4_8(){
    println!("Task 4.8");
    let mut rand_num = rand::thread_rng();
    let meters: f64 = rand_num.gen_range(1.0..100.0);
    let foots: f64 = rand_num.gen_range(50.0..200.0);
    println!("Length in meters: {:.3}, in foots: {:.3}, in foots converted to meters: {res:.3}",
             meters, foots, res = {foots * 0.305});
    if meters > foots * 0.305 {println!("Length in meters is bigger")}
    else if meters < foots * 0.305 {println!("Length in foots is bigger")}
    else {println!("Lengths are equal")};
    println!();

}


fn task_4_10(){
    println!("Task 4.10");
    let mut rand_num = rand::thread_rng();
    let circle_radius: f64 = rand_num.gen_range(1.0..10.0);
    let square_side: f64 = rand_num.gen_range(1.0..10.0);
    const PI: f64 = std::f64::consts::PI;
    let circle_square: f64 = PI * circle_radius.powi(2);
    let square_square: f64 = square_side.powi(2);
    println!("Squares of circle: {:.3} and square: {:.3}", circle_square, square_square);
    if circle_square > square_square {println!("Square of circle is bigger")}
    else if circle_square < square_square {println!("Square of square is bigger")}
    else {println!("Squares are equal")};
    println!();
}


fn task_4_13_14(){
    println!("Tasks 4.13 + 14");
    let mut rand_num = rand::thread_rng();
    let a: f64 = rand_num.gen_range(1.0..10.0); //a != 0
    let b: f64 = rand_num.gen_range(-10.0..10.0);
    let c: f64 = rand_num.gen_range(-10.0..10.0);

    let discr: f64 = b.powi(2) - 4.0 * a * c;

    if discr < 0.0 {println!("Discriminant {:.3} is below zero, no roots.", discr)}
    else if discr == 0.0 {
        let root: f64 = (-b + discr.sqrt()) / (2.0 * a);
        println!("Discriminant {:.3} equals zero, one root: {:.3}.", discr, root);
    }
    else if discr > 0.0 {
        let root1: f64 = (-b + discr.sqrt()) / (2.0 * a);
        let root2: f64 = (-b - discr.sqrt()) / (2.0 * a);
        println!("Discriminant {:.3} is bigger than zero, two roots: {:.3}, {:.3}.",
                 discr, root1, root2);
    }
    else {println!("This should not happen at any cases.")};
    println!();
}


//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 4.");
    println!();
    task_4_1();
    task_4_6();
    task_4_8();
    task_4_10();
    task_4_13_14();
    println!("End of tasks.");
}
