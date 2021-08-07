use crepe::crepe;

crepe! {
    @input
    struct User(i32);

    @input
    struct HasRole(i32, i32);

    @output
    struct Allowed(i32);

    Allowed(u) <- User(u), HasRole(u, 0);
}

fn main() {
    let mut runtime = Crepe::new();
    runtime.extend(&[User(1)]);
    runtime.extend(&[HasRole(1, 0)]);

    let (allowed,) = runtime.run();
    for Allowed(x) in allowed {
        println!("User {} is allowed", x);
    }
}
