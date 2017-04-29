pub fn does_not_compile() {
    let jaunty_phrase =  "Everything is awesome".to_string();

    print_jaunty_phrase(&jaunty_phrase);

    println!("\"{}\" is my favorite phrase", jaunty_phrase);
}

fn print_jaunty_phrase(phrase: &String) {
    println!("{}", phrase);
}
