//Zlatopolsky tasks book. Chapter 1.
use rand::Rng;


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
	println!("Full E value is: {}", e_num);
	println!("Short E value is: {name:.*}", 1, name = e_num);
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

	println!("i) sin(a)cos(b)+cos(a)sin(b), alpha = {ialpha:.2}, beta = {ibeta:.2}, result (in radians) = {rez:.2}",
			 ialpha = alpha, ibeta = beta, rez = {(alpha.sin() * beta.cos()) + (alpha.cos() * beta.sin())});

	println!("j) a*sqrt(2b), a = {ia:.2}, b = {ib:.2}, result = {rez:.2}",
			 ia = a, ib = b, rez = {a * (2.0 * b).sqrt()});

	println!("k) 3sin(2alpha)*cos(3beta), alpha = {ia:.2}, beta = {ib:.2}, result (in radians) = {rez:.2}",
			 ia = alpha, ib = beta, rez = {3.0 * (2.0 * alpha).sin() * (3.0 * beta).cos()});

	println!("l) -5*sqrt(x+sqrt(y)), x = {ia:.2}, y = {ib:.2}, result = {rez:.2}",
			 ia = x, ib = y, rez = {-5.0 * (x + y.sqrt()).sqrt()});

	println!();
}


//15-17 skipped


//18
fn task_1_18(){
	println!("Task 1.18");
	
	let mut s: u8 = 5;
	s = 57;					//simple change of value of the variable
	println!("a) s = 5, s = 57, result = {rez}", rez = s);
	
	let mut s: f64 = 6.0;	//s is shadowed for a new value and has a new type
	s = -5.2 * s;
	s = 0.0; 
	println!("b) s = 6, s = -5.2 * s, s = 0, result = {rez:.2}", rez = s);

	let mut s: f64 = -7.5; 	//you still need to set the type of a variable after shadowing it
	s = 2.0 * s; //there are no increments like s *= 2 or s++ in rust lang, it helps to avoid bugs
	println!("c) s = -7.5, s = 2 * s, result = {rez:.2}", rez = s);

	let mut rand_num = rand::thread_rng();
	let k: i8 = rand_num.gen_range(-50..50);
	let mut s: i8 = 45;
	s = -25;
	s = s + k;
	println!("d) s = 45, s = -25, s = s + k, k = {k} result = {rez}", k = k, rez = s);
	println!();
}


//19-21 skipped


//21
fn task_1_21(){
	println!("Task 1.21");
	let mut a: f64 = 5.8;
	let mut b: f64 = -7.9;
	println!("a) a = {}, b = {}", a, b);
	b = a;
	a = b;
	println!("Result: a = {}, b = {}", a,b);
	println!();

	let mut a: f64 = 0.0;
	let mut b: f64 = -9.99;
	println!("b) a = {}, b = {}", a, b);
	b = a;
	a = b;
	println!("Result: a = {}, b = {}", a,b);
	println!()
}


//22
fn task_1_22(){
	println!("Task 1.22");
	let mut rand_num = rand::thread_rng();
	let mut y: i32 = 0;
	let mut b: i32 = 0;
	let x: i32 = rand_num.gen_range(-5..5);
	let a: i32 = rand_num.gen_range(-7..7);
	y = 7 * x.pow(2) - 3 * x + 6;
	b = 12 * a.pow(2) + 7 * a - 16;
	println!("a) y = 7x^2 - 3x + 6, x = {}, result = {}", x, y);
	println!("b) b = 12a^2 + 7a - 16, a = {}, result = {}", a, b);
	println!()
}


//23
fn task_1_23(){
	println!("Task 1.23");
	let mut rand_num = rand::thread_rng();
	let a: f64 = rand_num.gen_range(-10.0..10.0);
	let mut y: f64 = 0.0;
	y = (a.powi(2) + 10.0) / ((a.powi(2) + 1.0).sqrt());
	println!("y = a^2 / sqrt(a^2+1), a = {:.2}, result = {:.2}", a, y);
	println!();
}


//25-29
fn task_1_25_29(){
	println!("Tasks 1.25 - 1.29");
	let mut rand_num = rand::thread_rng();
	let sq_side: u8 = rand_num.gen_range(1..50);
	println!("25) Square side len = {}, perimeter = {p}", sq_side, p = {sq_side * 4});

	let radius: u8 = rand_num.gen_range(1..15);
	println!("26) Radius = {}, diameter = {d}", radius, d = {radius * 2});

	//D = [(R+h)^2 - R^2]^1/2
	const R: f64 = 6350.0;
	let mut horizon: f64 = 0.0;
	let height: f64 = 0.0017; //in km 
	horizon = ((R + height).powi(2) - R.powi(2)).sqrt();
	println!("27) Distance to horizon is {:.2} km from height {} km.", horizon, height);

	let cube_side: u32 = rand_num.gen_range(1..10);
	let mut volume: u32 = 0;
	let mut side_square: u32 = 0;
	volume = cube_side.pow(3);
	side_square = cube_side.pow(2);
	println!("28) Cube side = {}, volume = {}, side square = {}", cube_side, volume, side_square);

	let pi_num = std::f64::consts::PI;
	let mut circle_len: f64 = 0.0;
	let mut circle_sqr: f64 = 0.0;
	let rad = radius as f64;
	circle_len = 2.0 * pi_num * rad;
	circle_sqr = pi_num * rad.powi(2);
	println!("Radius = {:.2}, circle length = {:.2}, circle square = {:.2} ",
			 rad, circle_len, circle_sqr);
	println!();
}


//30
fn task_1_30(){
	println!("Task 1.30");
	let mut rand_num = rand::thread_rng();
	let x: f64 = rand_num.gen_range(1.0..10.0);
	let y: f64 = rand_num.gen_range(1.0..10.0);
	let mut z: f64 = 0.0;
	z = x.powi(3) - (2.5 * x * y) + (1.78 * x.powi(2)) - (2.5 * y) + 1.0;
	println!("a) X = {:.2}, Y = {:.2},  Result of equation: {:.2}", x, y, z);
	println!();
}


//31
fn task_1_31(){
	println!("Task 1.31");
	let mut rand_num = rand::thread_rng();
	let x: f64 = rand_num.gen_range(1.0..10.0);
	let y: f64 = rand_num.gen_range(1.0..10.0);
	let _arif: f64 = 0.0;
	let _geom: f64 = 0.0;
	println!("X = {:.2}, Y = {:.2}, arif = {:.2}, geom = {:.2}",
			 x, y, arif = {(x + y) / 2.0}, geom = {(x * y).sqrt()});
	println!();
}


//35+37
fn task_1_35_37(){
	println!("Task 1.35 + 37");
	let mut rand_num = rand::thread_rng();
	let kat_x: f64 = rand_num.gen_range(1.0..10.0);
	let kat_y: f64 = rand_num.gen_range(1.0..10.0);
	let mut hypot: f64 = 0.0;
	let mut perim: f64 = 0.0;
	hypot = (kat_x.powi(2) + kat_y.powi(2)).sqrt();
	perim = kat_x + kat_y + hypot;
	println!("Katet 1 = {:.2}, Katet 2 = {:.2}, Hypotenuse = {:.2}, Perimeter = {:.2}",
			 kat_x, kat_y, hypot, perim);
	println!();
}


//51
fn task_1_51(){
	println!("Task 1.51");

	//check fig51 in src
	let ax: u8 = 1;
	let ay: u8 = 2;
	let bx: u8 = 3;
	let by: u8 = 4;
	let cx: u8 = 5;
	let cy: u8 = 3;
	let dx: u8 = 2;
	let dy: u8 = 1;

	//find katets
	let ab: f64 = ((ax as f64 - bx as f64).abs().powi(2) +
					  (ay as f64 - by as f64).abs().powi(2)).sqrt();

	let bc: f64 = ((bx as f64 - cx as f64).abs().powi(2) +
					  (by as f64 - cy as f64).abs().powi(2)).sqrt();

	let ad: f64 = ((ax as f64 - dx as f64).abs().powi(2) +
					  (ay as f64 - dy as f64).abs().powi(2)).sqrt();

	let cd: f64 = ((cx as f64 - dx as f64).abs().powi(2) +
					  (cy as f64 - dy as f64).abs().powi(2)).sqrt();

	//find hypotenuse
	let ac: f64 = (ab.powi(2) + bc.powi(2)).sqrt();

	//find half perimeters of triangles
	let hp_abc: f64 = (ab + bc + ac) / 2.0; //4.34
	let hp_adc: f64 = (ad + cd + ac) / 2.0; //4.315

	//find squares of triangles
	let sq_abc: f64 = (hp_abc * (hp_abc - ab) * (hp_abc - bc) * (hp_abc - ac)).sqrt();
	let sq_adc: f64 = (hp_adc * (hp_adc - ad) * (hp_adc - cd) * (hp_adc - ac)).sqrt();

	println!("Sides. AB = {:.2}, BC = {:.2}, AC = {:.2}, AD = {:.2}, CD = {:.2}", ab, bc, ac, ad, cd);
	println!("Triangles squares are: {:.2} and {:.2}. Quadrangle square is {:.2}.",
			 sq_abc, sq_adc, q = {sq_abc + sq_adc});
	println!();
}


//57
fn task_1_57(){
	println!("Task 1.57");
	let mut rand_num = rand::thread_rng();
	let temp_celcius: u8 = rand_num.gen_range(1..100);
	let temp_kelvin: f64 = temp_celcius as f64 + 273.15;
	let temp_fahren: f64 = temp_celcius as f64 * 1.8 + 32.0;
	println!("Temperature in celcius: {:.2}, kelvin: {:.2}, fahrenheit: {:.2}",
			 temp_celcius, temp_kelvin, temp_fahren);
	println!();
}


//59
fn task_1_59(){
	println!("Task 1.59");
	let mut rand_num = rand::thread_rng();
	let mut x: u8 = rand_num.gen_range(1..10);
	let mut y: u8 = rand_num.gen_range(1..10);
	println!("Variables before change: {}, {}", x, y);
	x = x + y;
	y = x - y;
	x = x - y;
	println!("Variables after change: {}, {}", x, y);
	println!();
}


//60
fn task_1_60(){
	println!("Task 1.60");
	let mut rand_num = rand::thread_rng();
	let mut a: u8 = rand_num.gen_range(1..10);
	let mut b: u8 = rand_num.gen_range(1..10);
	let mut c: u8 = rand_num.gen_range(1..10);
	println!("Variables before change: {}, {}, {}", a, b, c);
	b = c;
	a = b;
	c = a;
	println!("Variables after change: {}, {}, {}", a, b, c);
	println!();

}


//61
fn task_1_61(){
	println!("Task 1.61");
	let a: u16 = 2;
	let mut a4: u16 = 0;
	a4 = a * a;
	a4 = a4 * a4;
	println!("a^4 in 2 operations: {}", a4);

	let mut a9: u16 = 0;
	a9 = a * a;
	a9 = a9 * a9;
	a9 = a9 * a9;
	a9 = a9 * a;
	println!("a^9 in 4 operations: {}", a9);

	let mut a15: u16 = 0;
	let mut a15t: u16 = 0;
	a15 = a * a;
	a15t = a15 * a;
	a15 = a15t * a15t;
	a15 = a15 * a15;
	a15 = a15 * a15t;
	println!("a^15 in 5 operations: {}", a15);
	println!();
}

//keep main fn in the end
fn main() {
	println!("Zlatopolsky tasks book. Chapter 1.");
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
	task_1_18();
	task_1_21();
	task_1_22();
	task_1_23();
	task_1_25_29();
	task_1_30();
	task_1_31();
	task_1_35_37();
	task_1_51();
	task_1_57();
	task_1_59();
	task_1_60();
	task_1_61();
	println!("End of tasks.");
} 
