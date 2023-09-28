mod exo1;
mod exo2;

#[tokio::main]
async fn main() {
    let args = std::env::args();
    for arg in args {
        match arg.as_str() {
            "--exo1" => exo1::main().await,
            "--exo2" => exo2::main().await,
            _ => {}
        }
    }
}
