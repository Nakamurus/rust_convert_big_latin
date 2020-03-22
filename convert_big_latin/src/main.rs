fn main() {
    let mut txt = String::from("first");
    converter(txt);
}

fn converter(mut s: String) {
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    let mut hay = String::from("-hay");
    let head = &s.chars().nth(0).unwrap();

    if vowels.contains(&head) == false {
        hay = format!("-{}ay", head);
    }

    s.push_str(&hay);
    s.remove(0);
    println!("{}", s);
}
