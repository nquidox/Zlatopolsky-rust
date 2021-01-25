//Zlatopolsky tasks book. Chapter 3.


fn task_3_1_5(){
    println!("Task 3.1 - 3.5");
    let a: bool = true;
    let b: bool = false;
    let c: bool = false;
    println!("a = {}, b = {}, c = {}.", a, b, c);
    println!();
    println!("3.1 Results for: a OR b: {}, a AND b: {}, b OR c: {}.", a || b, a && b, b || c);
    println!();
    println!("3.3 Results for: NOT a AND b: {}, a OR NOT b: {}, a AND b OR c: {}.",
             !a && b, a || !b, a && b || c);
    println!();
    println!("3.5 Results: \na) a OR b AND NOT c: {},\n\
                             b) NOT a AND NOT b: {},\n\
                             c) NOT (a AND c) OR b: {}\n\
                             d) a AND NOT b OR c: {},\n\
                             e) a AND (NOT b OR c): {},\n\
                             f) a or (NOT(b AND c)): {}.",
                                a || b && !c,
                                !a && !b,
                                !(a && c) || b,
                                a && !b || c,
                                a && (!b || c),
                                a || (!(b && c)));
    println!();
}


fn task_3_11(){
    println!("Task 3.11");
    //values of x and y will be hardcoded
    let a: bool = 1_i8.pow(2) + (-1_i8.pow(2)) <= 4;
    let b: bool = (1 >= 0) || (2_i8.pow(2) != 4);
    let c: bool = (1 >= 0) && (2_i8.pow(2) != 4);
    let d: bool = (2 * 1 != 0) && (1 > 2);
    let e: bool = (2 * 1 != 0) || (1 > 2);
    let f: bool = (!(2 * 1 < 0)) && (1 > 2);
    let g: bool = (!(1 * 2 < 0)) || (2 > 1);
    println!("a) x^2 + y^2 <= 4, x = 1, y = -1: {}\n\
              b) (x >= 0) OR (y^2 != 4), x = 1, y = 2: {}\n\
              c) (x >= 0) AND (y^2 != 4), x = 1, y = 2: {}\n\
              d) (x * y != 0) AND (y > x), x = 2, y = 1: {}\n\
              e) (x * y != 0) OR (y < x), x = 2, y = 1: {}\n\
              f) (NOT (x * y <0)) AND (y > x), x = 2, y = 1: {}\n\
              g) (NOT (x * y <0)) OR (y > x), x = 1, y = 2: {}\n",
             a, b, c, d, e, f, g);
    println!();
}


fn task_3_27(){
    println!("Task 3.27");
    println!();
    let x: i8 = 3;
    let y: i8 = 4;
    println!("X = {}, Y = {}.", x, y);
    println!("a) x > 2 AND y > 3: {res}", res = {x > 2 && y > 3});
    println!("b) x > 2 AND y > 3: {res}", res = {x >= 0 && y < 5}); //B)
    println!("c) 0 < y <= 4 AND x < 5: {res}", res = {y > 0 && y <= 4 && x < 5}); //u)
    println!();
}


fn task_3_30(){
    println!("Task 3.30");
    let a: u8 = 6;
    println!("A = {} is multiple of 2 or 3: {res}", a, res = {a % 2 == 0 || a % 3 == 0});
    let a: u8 = 10;
    println!("A = {} is not multiple of 3 and ends with a 0: {res}",
             a, res = {a % 3 != 0 && a % 10 == 0});
    println!();
}


//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 3.");
    println!();
    task_3_1_5();
    task_3_11();
    task_3_27();
    task_3_30();
    println!("End of tasks.");
}
