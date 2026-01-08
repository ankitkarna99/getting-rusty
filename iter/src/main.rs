fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }

    elements
        .iter()
        // .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    return elements.iter().map(|el| el.to_uppercase()).collect();
}

fn move_elements(a: Vec<String>, b: &mut Vec<String>) {
    a.into_iter().for_each(|el| b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    return elements.iter().map(|el| el.chars().map(|c| c.to_string()).collect()).collect();
}

fn find_or(elements: &[String], search: &str, fallback: &str) -> String {
    return elements.iter().find(|el| el.contains(search)).map_or(String::from(fallback), |el| el.to_string());
}

fn main() {
    let mut colors = vec![
        String::from("red"), 
        String::from("green"),
        String::from("blue")
    ];

    let search_result = find_or(&colors, "grex", "Orange");

    println!("{}", search_result);

    // let exploded_colors = explode(&colors);
    
    // println!("{:#?}", exploded_colors);
    
    // print_elements(&colors);
    
    // shorten_strings(&mut colors);
    
    // print_elements(&colors);

    // let uppercase_colors = to_uppercase(&colors);

    // print_elements(&uppercase_colors);

    // let mut new_uppercase_colors = vec![];
    
    // move_elements(uppercase_colors, &mut new_uppercase_colors);
    
    // print_elements(&new_uppercase_colors);
}
