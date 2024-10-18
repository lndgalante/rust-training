fn main() {
    let file = std::fs::read_to_string("lines").unwrap();

    file.lines().for_each(|line| println!("{}", line));
}

fn main() {
    let file = std::fs::read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}
