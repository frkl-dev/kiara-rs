use std::env;
use viva::{VivaGlobals};
use kiara::{KiaraContext};

fn handle_result<T>(result: Result<T, anyhow::Error>) -> T {
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        for cause in e.chain().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
        std::process::exit(1);
    } else {
        result.unwrap()
    }
}


#[tokio::main]
async fn main() {
    let globals = VivaGlobals::create("dev", "frkl", "kiara");

    let cmd: Vec<String> = env::args().skip(1).collect();

    let context_name = "default";
    let plugins: Option<Vec<String>> = None;


    let kiara_context = KiaraContext::create(context_name, plugins, None, &globals);

    handle_result(kiara_context.run_kiara_command(&cmd).await);

}
