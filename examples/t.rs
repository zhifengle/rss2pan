fn main() {
    let mut names = Vec::new();
    names.push("Mittens");
    names.push("Jingle Paws");
    names.push("Sir Fluffy");
    let placeholders = names.iter().map(|_| "(?)").collect::<Vec<_>>().join(", ");
    let q = format!("INSERT INTO cats (name) VALUES {}", placeholders);
    println!("{}", q);
    println!("{}", 12312313.to_string())
}
