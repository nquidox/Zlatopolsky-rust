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


fn task_4_16(){
    println!("Task 4.16");
    let circle_square: u16 = 100;
    let square_square: u16 = 100;
    const PI: f64 = std::f64::consts::PI;
    let outer_circle_radius: f64 = (2.0 * square_square as f64).sqrt() / 2.0;
    let inner_circle_radius: f64 = (square_square as f64).sqrt() / 2.0;
    let circle_radius: f64 = (circle_square as f64 / PI).sqrt();
    println!("Circle radius: {:.2}, outer circle radius: {:.2}, inner circle radius: {:.2}",
                circle_radius, outer_circle_radius, inner_circle_radius);
    if circle_radius >= outer_circle_radius {println!("Circle is around the square")}
    else if circle_radius <= inner_circle_radius {println!("Circle is inside the square")}
    else {println!("Circle cannot be inside or outside the square")};
    println!();
}


fn task_4_20(){
    println!("Task 4.20");
    let mut rand_num = rand::thread_rng();
    let m: u8 = rand_num.gen_range(1..10);
    let n: u8 = rand_num.gen_range(1..10);
    println!("M = {}, N = {}, mod = {res}", m ,n, res = {m % n});
    if m % n == 0 {println!("The numbers devide evenly.")}
    else {println!("The numbers don't devide evenly.")}
    println!();
}


fn task_4_22_23(){
    println!("Tasks 4.22 and 23");
    let mut rand_num = rand::thread_rng();
    let x: u8 = rand_num.gen_range(10..101);
    println!("X = {}", x);

    if x % 2 == 0 {println!("X = {} is an even number", x)}
    else {println!("X = {} is not an even number", x)};

    if x % 10 == 7 {println!("X = {} ends with a number 7", x)}
    else {println!("X = {} doesn't end with a number 7", x)}

    if (x % 10) < (x / 10) {println!("First number is bigger than second")}
    else if (x % 10) > (x / 10) {println!("First number is lower than second")}
    else {println!("Both numbers are equal")};
    println!();
}


fn task_4_24(){
    println!("Task 4.24");
    let x: u16 = 48;
    let y: u16 = 52;

    if x.pow(2) == (4 * ((x / 10).pow(3) + (x % 10).pow(3))) {println!("True")}
    else {println!("False")};

    if y.pow(2) == (4 * ((y / 10).pow(3) + (y % 10).pow(3))) {println!("True")}
    else {println!("False")};
    println!();
}


fn task_4_37(){
    println!("Task 4.37");
    let mut rand_num = rand::thread_rng();
    let x: i8 = rand_num.gen_range(-10..10);
    if x >= -5 && x <= 3 {println!("X = {} belongs to interval -5..3", x)}
    else {println!("X = {} not belongs to interval -5..3", x)}
    println!();
}


fn task_4_42(){
    println!("Task 4.42");
    let mut rand_num = rand::thread_rng();
    let a: i8 = rand_num.gen_range(-10..10);
    let b: i8 = rand_num.gen_range(-10..10);
    let c: i8 = rand_num.gen_range(-10..10);

    if a < b && b < c {println!("{} < {} < {} is true", a, b, c)}
    else {println!("{} < {} < {}  is false", a, b, c)}

    if b > a && a > c {println!("{} > {} > {} is true", a, b, c)}
    else {println!("{} > {} > {} is false", a, b, c)}
    println!();
}


fn task_4_45(){
    println!("Task 4.45");
    let mut rand_num = rand::thread_rng();
    let a: i8 = rand_num.gen_range(-10..10);
    let b: i8 = rand_num.gen_range(-10..10);
    let c: i8 = rand_num.gen_range(-10..10);
    println!("A = {}, B = {}, C = {}", a, b, c);
    if a==b || b==c || a==c {println!("At least 2 numbers are equal")}
    else {println!("No equal numbers")};
    println!();
}


fn task_4_52(){
    println!("Task 4.52");
    let mut rand_num = rand::thread_rng();
    let x: u8 = rand_num.gen_range(1..30);
    let y: u8 = rand_num.gen_range(1..20);
    let r: u8 = rand_num.gen_range(1..10);
    let head_size: u8 = r * 2 + 2; //includes 1 cm on both sides
    println!("Width = {}, height = {}, head size = {}", x, y, head_size);
    if head_size <= x && head_size <= y {println!("Vasya can stick his head through fortochka")}
    else {println!("Vasya can not stick his head through fortochka")};
    println!();
}


fn task_4_60_63(){
    println!("Task 4.60-63");
    let mut rand_num = rand::thread_rng();
    let x: u16 = rand_num.gen_range(1000..10000); //last num is not included
    //let x: u16 = 3331; //hardcore a certain number here to check conditions
    let thousands: u16 = x / 1000;
    let hundreds: u16 = x % 1000 / 100;
    let tens: u16 = x % 100 / 10;
    let ones: u16 = x % 10;
    println!("Number = {}", x);
    //check for numbers
    if thousands == 2 || thousands == 7 || hundreds == 2 || hundreds == 7
        || tens == 2 || tens == 7 || ones == 2 || ones == 7
        {println!("The number has 2 or 7 in it")}
    else {println!("The number has no 2 or 7 in it")};

    //check if palindrome
    if ((thousands * 10) + hundreds) == ((ones * 10) + tens) {println!("The number is a palindrome")}
    else {println!("The number is not a palindrome")};

    //check for only 3 same numbers
    if thousands == hundreds && thousands == tens && hundreds == tens && tens != ones
    {println!("The number has 3 same numbers")}

    else if thousands == hundreds && thousands == tens && hundreds != tens && tens == ones
    {println!("The number has 3 same numbers")}

    else if thousands == hundreds && thousands != tens && hundreds == tens && tens == ones
    {println!("The number has 3 same numbers")}

    else if thousands != hundreds && thousands == tens && hundreds == tens && tens == ones
    {println!("The number has 3 same numbers")}

    else {println!("Number doesn't have 3 same numbers")}

    //check if all numbers are different
    if thousands != hundreds && thousands != tens && thousands != ones
        && hundreds != tens && hundreds != ones && tens != ones
        {println!("All numbers are different")}
    else {println!("Some number are equal")};
    println!();
}


fn task_4_64(){
    println!("Task 4.64");
    let mut rand_num = rand::thread_rng();
    let number: u32 = rand_num.gen_range(100_000..1_000_000);
    let n1: u32 = number / 100_000;
    let n2: u32 = number % 100_000 / 10_000;
    let n3: u32 = number % 10_000 / 1000;
    let n4: u32 = number % 1000 / 100;
    let n5: u32 = number % 100 / 10;
    let n6: u32 = number % 10;
    let lucky: bool = if n1 + n2 + n3 == n4 + n5 + n6 {true} else {false};
    println!("Is {} a lucky number? Answer is {}", number, lucky);
    println!();
}


fn task_4_65(){
    println!("Task 4.65");
    let mut rand_num = rand::thread_rng();
    let year: u16 = rand_num.gen_range(1000..9000);
    //let year: u16 = 2020; //hardcode a year to check condition
    if year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0))
    {println!("Year {} is leap.", year)}
    else {println!("Year {} is not leap.", year)};
    println!();
}


fn task_4_67(){
    println!("Task 4.67");
    let mut rand_num = rand::thread_rng();
    let k: u16 = rand_num.gen_range(1..366);
    println!("The day: {}", k);
    match k % 7 {
        1 => println!("Monday, work day."),
        2 => println!("Tuesday, work day."),
        3 => println!("Wednesday, work day."),
        4 => println!("Thursday, work day."),
        5 => println!("Friday, work day."),
        6 => println!("Saturday, day off."),
        0 => println!("Sunday, day off."),
        _ => println!("Shouldn't be.")
    };
    println!();
}


fn task_4_78_80() {
    println!("Task 4.78-80");
    let mut rand_num = rand::thread_rng();
    let a: i8 = rand_num.gen_range(-10..11);
    let b: i8 = rand_num.gen_range(-10..11);
    let c: i8 = rand_num.gen_range(-10..11);
    println!("Three numbers: {}, {}, {}", a, b, c);

    println!("Even numbers : {a}, {b}, {c}, 0 for odds.",
             a = {if a % 2 == 0 {a} else {0}},
             b = {if b % 2 == 0 {b} else {0}},
             c = {if c % 2 == 0 {c} else {0}});

    println!("Positive numbers squared: {}, {}, {}",
             a = {if a > 0 {a.pow(2)} else {a}},
             b = {if b > 0 {b.pow(2)} else {b}},
             c = {if c > 0 {c.pow(2)} else {c}});

    println!("Numbers in range 1,6 - 3.8 : {a}, {b}, {c}, | 0 for out of range",
             a = {if a as f64 >= 1.6 && a as f64 <= 3.8 {a} else {0}},
             b = {if b as f64 >= 1.6 && b as f64 <= 3.8 {b} else {0}},
             c = {if c as f64 >= 1.6 && c as f64 <= 3.8 {c} else {0}});

    println!("Numbers in range 0,7 - 5.1 : {a}, {b}, {c}, | 0 for out of range",
             a = {if a as f64 >= 0.7 && a as f64 <= 5.1 {a} else {0}},
             b = {if b as f64 >= 0.7 && b as f64 <= 5.1 {b} else {0}},
             c = {if c as f64 >= 0.7 && c as f64 <= 5.1 {c} else {0}});
    println!();
}


fn task_4_82() {
    println!("Task 4.82");
    let mut rand_num = rand::thread_rng();
    let a: u8 = rand_num.gen_range(1..11);
    let b: u8 = rand_num.gen_range(1..11);
    let c: u8 = rand_num.gen_range(1..11);
    let d: u8 = rand_num.gen_range(1..11);
    let mut counter: u8 = 0;
    println!("4 numbers given: {}, {}, {}, {}.", a, b, c, d);
    if a % 2 == 0 {counter += 1};
    if b % 2 == 0 {counter += 1};
    if c % 2 == 0 {counter += 1};
    if d % 2 == 0 {counter += 1};
    println!("Quantity of even numbers: {}", counter);
    println!();
}


fn task_4_83() {
    println!("Task 4.83");
    let mut rand_num = rand::thread_rng();
    let a: u8 = rand_num.gen_range(1..11);
    let b: u8 = rand_num.gen_range(1..11);
    let c: u8 = rand_num.gen_range(1..11);
    let d: u8 = rand_num.gen_range(1..11);
    let mut counter: u8 = 0;
    println!("4 numbers given: {}, {}, {}, {}.", a, b, c, d);
    if a > 5 {counter += a};
    if b > 5 {counter += b};
    if c > 5 {counter += c};
    if d > 5 {counter += d};
    println!("Sum of numbers that are higher than 5: {}", counter);
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
    task_4_16();
    task_4_20();
    task_4_22_23();
    task_4_24();
    task_4_37();
    task_4_42();
    task_4_45();
    task_4_52();
    task_4_60_63();
    task_4_64();
    task_4_65();
    task_4_67();
    task_4_78_80();
    task_4_82();
    task_4_83();
    println!("End of tasks.");
}
