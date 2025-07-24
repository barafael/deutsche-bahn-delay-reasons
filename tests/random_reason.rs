use deutsche_bahn_delay_reasons::Grund;

#[test]
fn some_reason() {
    let grund = Grund::default();
    println!("Test passed because {grund}");
}
