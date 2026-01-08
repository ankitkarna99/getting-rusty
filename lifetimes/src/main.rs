fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    return languages.last().unwrap();
}

fn longest_language<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }

    return b;
}

fn main() {
    let languages = vec![
        String::from("Rust"),
        String::from("C++"),
        String::from("Python")
    ];

    let result = next_language(&languages, "C++");

    let longest = longest_language("Typescript", "Go");

    println!("{}", longest);

    println!("{}", result)
}
