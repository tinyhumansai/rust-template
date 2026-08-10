use rust_template::greet;

fn main() -> rust_template::Result<()> {
    println!("{}", greet("Rust")?);
    Ok(())
}
