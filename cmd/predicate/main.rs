use crepe::crepe;

crepe! {
    @input
    struct User(&'static str);

    @input
    struct HasRole(&'static str, &'static str);

    @output
    struct Allowed(&'static str);

    Allowed(u) <- User(u), HasRole(u, "admin");
}

fn main() {
    let mut runtime = Crepe::new();
    runtime.extend(&[User("Alice"), User("Bob")]);
    runtime.extend(&[HasRole("Alice", "admin"), HasRole("Bob", "users")]);

    let (allowed,) = runtime.run();
    println!("Found {} output relationships", allowed.len());
    println!("Found output relationships for Alice: {}", allowed.get(&Allowed("Alice")).is_some());
    println!("Found output relationships for Bob: {}", allowed.get(&Allowed("Bob")).is_some());
    for Allowed(x) in allowed {
        println!("User {} is in allowed set", x);
    }
}
