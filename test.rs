fn main() {
    let users = vec!["alice", "bob", "carol"];
    let target = "bob";

    match users.iter().find(|&&u| u == target) {
        Some(user) => println!("Found user: {}", user),
        None => println!("User not found"),
    }

    let sum: i32 = (1..=10).sum();
    println!("Sum 1..10 = {}", sum);
}
