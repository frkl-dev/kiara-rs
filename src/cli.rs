use std::env;
use viva::{EnvCheckStrategy, global_multi_progress, IndicatifWriter, PkgInstallStrategy, VivaEnv, VivaGlobals};
use tracing_subscriber::{EnvFilter, filter::LevelFilter, util::SubscriberInitExt};
use kiara::{ALL_KIARA_PACKAGES, KiaraContext};

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

    let env_name = "kiara";
    let specs = vec!["kiara_plugin.tabular".to_string()];
    let channels = vec!["conda-forge".to_string(), "dharpa".to_string()];

    let check_strategy = EnvCheckStrategy::Auto;
    let pkg_install_strategy = PkgInstallStrategy::Append;

    // let cmd = vec!("kiara".to_string(), "data".to_string(), "list".to_string());
    let cmd: Vec<String> = env::args().skip(1).collect();

    let context_name = "default";
    let plugins: Option<Vec<String>> = None;
    let kiara_context = KiaraContext::create(context_name, plugins, &globals);

    handle_result(kiara_context.run_kiara_command(&cmd).await);



    // let viva_env = handle_result(VivaEnv::create(env_name, Some(specs), Some(channels), &globals));
    //
    // // // Determine the logging level based on the the verbose flag and the RUST_LOG environment
    // // // variable.
    // let default_filter = LevelFilter::INFO;
    //
    // let env_filter = EnvFilter::builder()
    //     .with_default_directive(default_filter.into())
    //     .from_env().expect("Failed to parse the RUST_LOG environment variable")
    //     // filter logs from apple codesign because they are very noisy
    //     .add_directive("apple_codesign=off".parse().expect("Failed to parse the RUST_LOG environment variable"));
    //
    // // Setup the tracing subscriber
    // tracing_subscriber::fmt()
    //     .with_env_filter(env_filter)
    //     .with_writer(IndicatifWriter::new(global_multi_progress()))
    //     .without_time()
    //     .finish()
    //     .try_init().expect("Failed to initialize the tracing subscriber");
    //
    // // viva_env.ensure(check_strategy, pkg_install_strategy).await),
    // handle_result(viva_env.run_command_in_env(cmd, check_strategy).await);


}
