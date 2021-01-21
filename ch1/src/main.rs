//Задачник Златопольского, Глава 1.
use rand::Rng;

//Начало
//Арифметические выражения.


//1
fn task_1_1() {
	let pi_num = std::f64::consts::PI;
	println!("Task 1.1");
	println!("Full Pi value is: {}", pi_num);
	println!("Short Pi value is: {name:.*}", 2, name = pi_num);
	println!(); // prints just a newline
}


//2
fn task_1_2(){
	let e_num = std::f64::consts::E;
	println!("Task 1.2");
	println!("Full Pi value is: {}", e_num);
	println!("Short Pi value is: {name:.*}", 1, name = e_num);
	println!();
}


//3 and 4
fn task_1_3_4(){
	println!("Task 1.3 and 1.4");
	let mut rand_num = rand::thread_rng();
	let input_number: u8 = rand_num.gen();
	println!("Randomly generated number is: {}", input_number);
	println!("{} is randomly generated number.", input_number);
	println!();
}


//5
fn task_1_5(){
	println!("Task 1.5");
	let a: u8 = 1;
	let b: u8 = 13;
	let c: u8 = 49;
	println!("Print {} {} {} with one space between numbers", a,b,c);
	println!();
}


//6
fn task_1_6(){
	println!("Task 1.6");
	let a: u8 = 7;
	let b: u8 = 15;
	let c: u8 = 100;
	println!("Print {}  {}  {} with two spaces between numbers", a,b,c);
	println!();
}


//7 and 8
fn task_1_7_8(){
	println!("Task 1.7 and 1.8");
	let mut rand_num = rand::thread_rng();
	let a: u8 = rand_num.gen();
	let b: u8 = rand_num.gen();
	let c: u8 = rand_num.gen();
	let d: u8 = rand_num.gen();
	println!("Print {}  {}  {} with two spaces between numbers", a,b,c);
	println!("Print {} {} {} {} with one space between numbers", a,b,c,d);
	println!();
}


//9 and 10
fn task_1_9_10(){
	println!("Task 1.9 and 1.10");
	println!("Print 50 and 10 in a row");
	println!("50\n10\n");
	println!("Print {a} {b} {c} in a row: \n{a}\n{b}\n{c}", a = 5, b = 10, c = 21);
	println!();
}


//11
fn task_1_11(){
	println!("Task 1.11");
	let mut rand_num = rand::thread_rng();
	let a: u8 = rand_num.gen();
	let b: u8 = rand_num.gen();
	let c: u8 = rand_num.gen();
	let d: u8 = rand_num.gen();
	println!("Print {a} {b} {c} {d} in a row: \n{a}\n{b}\n{c}\n{d}", a = a, b = b, c = c, d = d);
	println!();
}


//12
fn task_1_12(){
	println!("Task 1.12");
	let mut rand_num = rand::thread_rng();
	let t: u8 = rand_num.gen();
	let v: u8 = rand_num.gen();
	let x: u8 = rand_num.gen();
	let y: u8 = rand_num.gen();
	println!("Variant a: \n {a} {b}\n{c} {d}\n", a = 2, b = "kg", c = 13, d = 17);
	println!("Variant b: \n {a} {b}\n{c} {d}\n", a = 100, b = t, c = 1949, d = v);
	println!("Variant c: \n {a} {b}\n{c} {d}\n", a = x, b = 25, c = x, d = y);
	println!();
}


//13 same as 12


//14
fn task_1_14(){
	println!("Task 1.14\nMath expressions.\n");
	let mut rand_num = rand::thread_rng();
	let x: f64 = rand_num.gen_range(1.0..100.0);
	let y: f64 = rand_num.gen_range(1.0..100.0);
	let a: f64 = rand_num.gen_range(1.0..10.0);
	let b: f64 = rand_num.gen_range(1.0..10.0);
	let n: i32 = rand_num.gen_range(-1000..1);
	let alpha: f64 = rand_num.gen_range(1.0..10.0);
	let beta: f64 = rand_num.gen_range(1.0..10.0);
	//Памятный момент, тут я впервые словил panic от компилятора 21.01.2021
	//println!("a) 2x, x = {a}, result = {b}", a = x, b = 2*x);
	println!("a) 2x, x = {ia:.2}, result = {rez:.2}", ia = x, rez = {2.0*x});
	println!("b) sin(x), x = {ia:.2}, result (in radians) = {rez:.2}", ia = x, rez = {x.sin()});
	println!("c) a^2, a = {ia:.2}, result = {rez:.2}", ia = a, rez = {a.powi(2)});
	println!("d) sqrt(x), x = {ia:.2}, result = {rez:.2}", ia = x, rez = {x.sqrt()});
	println!("e) abs n, n = {ia:.2}, result = {rez:.2}", ia = n, rez = {n.abs()});
	println!("f) 5cos(y), y = {ia:.2}, result (in radians) = {rez:.2}", ia = y, rez = {5.0 * y.cos()});
	println!("g) -7.5a^2, a = {ia:.2}, result = {rez:.2}", ia = a, rez = {-7.5 * a.powi(2)});
	println!("h) 3sqrt(x), x = {ia:.2}, result = {rez:.2}", ia = x, rez = {3.0 * x.sqrt()});
	println!("i) sin(a)cos(b)+cos(a)sin(b), alpha = {ialpha:.2}, beta = {ibeta:.2}, result (in radians) = {rez:.2}", ialpha = alpha, ibeta = beta, rez = {(alpha.sin() * beta.cos()) + (alpha.cos() * beta.sin())});
	println!("j) a*sqrt(2b), a = {ia:.2}, b = {ib:.2}, result = {rez:.2}", ia = a, ib = b, rez = {a * (2.0 * b).sqrt()});
	println!("k) 3sin(2alpha)*cos(3beta), alpha = {ia:.2}, beta = {ib:.2}, result (in radians) = {rez:.2}", ia = alpha, ib = beta, rez = {3.0 * (2.0 * alpha).sin() * (3.0 * beta).cos()});
	println!("l) -5*sqrt(x+sqrt(y)), x = {ia:.2}, y = {ib:.2}, result = {rez:.2}", ia = x, ib = y, rez = {-5.0 * (x + y.sqrt()).sqrt()});
}



//Эта функция будет всегда в самом конце и в ней будут запускаться все остальные функции
fn main() {
	//let mut rng = rand::thread_rng();
	//let num: u8 = rng.gen();
	println!("Задачник Златопольского, Глава 1");
	println!();
	task_1_1();
	task_1_2();
	task_1_3_4();
	task_1_5();
	task_1_6();
	task_1_7_8();
	task_1_9_10();
	task_1_11();
	task_1_12();
	task_1_14();
} 

//
//fn task_1_(){
//	println!("Task 1.");
//	println!();
//}