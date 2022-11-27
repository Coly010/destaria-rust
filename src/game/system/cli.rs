use std::io;

pub fn get_cli_input_with_prompt(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input = String::from(input.trim());

    input
}
