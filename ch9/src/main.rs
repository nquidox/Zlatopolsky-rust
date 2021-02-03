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
    println!("Tasks 9.13-17");
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


fn task_9_19(){
    println!("Task 9.19");
    let word: &str = "Keyboard";
    let letter2: &str = &word.chars().nth(2).unwrap().to_string();
    let letter4: &str = &word.chars().nth(4).unwrap().to_string();
    let letters24 = letter2.to_owned() + letter4;
    println!("Letter 2 + 4 from word {}: {}", word, letters24);
    println!();
}

//unfinished, needs fixing
fn task_9_20_21() {
    println!("Tasks 9.20-21");
    let the_word: &str = "repository";
    println!("The word for all tasks: {}", the_word);
    let letter3: &str = &the_word.chars().nth(2).unwrap().to_string();
    let letter_last: &str = &the_word.chars().last().unwrap().to_string();
    println!("Third + last letters: {res}", res = { letter3.to_owned() + letter_last });
    let length: usize = the_word.chars().count();
    let mut chars234: String = "".to_string();
    let mut chn: String;
    for i in 0..length {
        if i >= 1 && i <= 3 {
        chn = the_word.chars().nth(i).unwrap().to_string();
        chars234 = chars234.to_owned() + &chn;
        }
    };
    println!("{:?}", chars234);
    println!();
}


fn task_9_24(){
    println!("Task 9.24");
    let the_word: String = String::from("TopRoflBot");
    let mut word: String = the_word.clone();
    let bot: String = word.split_off(7);
    let rofl: String = word.split_off(3);
    println!("The word '{}' splitted: {}, {}, {}", the_word, word, rofl, bot);
    println!();
}


fn task_9_42(){
    println!("Task 9.42");
    let the_word: &str = "Rust";
    let backwards_word: String = the_word.chars().rev().collect();
    println!("The word {} written backwards: {}", the_word, backwards_word);
}

//keep main fn in the end
fn main() {
    println!("Zlatopolsky tasks book. Chapter 9.");
    println!();
    task_9_1();
    task_9_3();
    task_9_13_17();
    task_9_18();
    task_9_19();
    task_9_20_21();
    task_9_24();
    task_9_42();
    println!("End of tasks.");
}