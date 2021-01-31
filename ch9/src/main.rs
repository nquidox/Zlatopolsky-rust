//Zlatopolsky tasks book. Chapter 9.


fn task_9_1(){
    println!("Task 9.1");
    let name: &str = "Rust";
    println!("{}", name);
    println!("Hello from {}!", name);
    println!();

}


fn task_9_3(){
    println!("Task 9.3");
    let name: &str = "Ivan";
    let surname: &str = "Ivanov";
    println!("{} {}", name, surname);
    println!("{}", name.to_owned() + " " + surname); //to_owned() is needed for concatenation
    println!();
}


fn task_9_13_17(){
    println!("Task 9.13-17");
    let word: &str = "Programming"; //should have at least 4 letters
    println!("{}", word);

    let ch3: char = word.chars().nth(2).unwrap();
    println!("Third char is: {:?}", ch3);

    let last: char = word.chars().last().unwrap();
    println!("Last char is: {:?}", last);

    let k: usize = 5;
    let kch: char = word.chars().nth(k).unwrap();
    println!("{} char is: {:?}", k, kch);

    let second_char: char = word.chars().nth(2).unwrap();
    let forth_char: char = word.chars().nth(4).unwrap();
    let eq: bool = second_char == forth_char;
    println!("Check if 2 and 4 are the same: {}", eq);

    let first_char: char = word.chars().next().unwrap();
    let last_char: char = word.chars().last().unwrap();
    let eq2: bool = first_char == last_char;
    println!("Check if first and last chars are the same: {}", eq2);
    println!();
}


fn task_9_18(){
    println!("Task 9.18");
    let word1: &str = "Tea";
    let word2: &str = "Party";
    let eq: bool = word1.chars().next().unwrap() == word2.chars().last().unwrap();
    println!("Is 1st letter of '{}' equals to last letter of '{}'? - {}", word1, word2, eq);
    //don't forget that T != t
    println!();
}


//not works, needs fixing
/*
fn task_9_19(){
    println!("Task 9.19");
    let word: &str = "Keyboard";
    let letters24: &str = !concat!(word.chars().nth(2).unwrap(), word.chars().nth(4).unwrap());
    println!("Letter 2 + 4 from word {}: {}", word, letters24);
    println!();
}
*/

//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 9.");
    println!();
    task_9_1();
    task_9_3();
    task_9_13_17();
    task_9_18();
    //task_9_19();
    println!("End of tasks.");
}