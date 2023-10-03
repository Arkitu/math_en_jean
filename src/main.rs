mod exo1;
mod exo2;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--exo1" => exo1::main().await,
            "--exo2" => exo2::main(&args[i+1..]).await,
            _ => {}
        }
    }
}
