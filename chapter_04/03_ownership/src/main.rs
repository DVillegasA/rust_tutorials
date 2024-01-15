fn _add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn return_a_string() -> String {
    let s = String::from("Hello, world!");
    s
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let greet = return_a_string();
    println!("{}", greet);

    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let full = stringify_name_with_title(&name);
    println!("{}", first);
    println!("{}", full);

    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );

    // let first = get_first(&name); 
    let first = &name.0;

    name.1.push_str(" Esq.");
    println!("{} {}", first, name.1);
}
