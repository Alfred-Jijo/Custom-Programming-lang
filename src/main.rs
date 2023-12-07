mod basic;
use basic::run;

fn main() {
    let fn_name = "example";
    let text = "3.14 + 2 * (6 - 4.5)";
    let (tokens, error) = run(fn_name, text);

    println!("Tokens: {:?}", tokens);
    if let Some(error) = error {
        println!("{}", error.as_string());
    }
}
