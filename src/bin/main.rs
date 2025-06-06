fn error() -> Result<(), ()> {
    Err(())
}

fn main() {
    error().unwrap();
}
