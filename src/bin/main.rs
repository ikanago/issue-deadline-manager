use issue_deadline_manager::runner::{Config, Runner};

#[tokio::main]
async fn main() {
    // Don't use clap or structopt here; arguments presence and order is stable.
    // They are guranteed by metadata of GitHub Actions.
    let mut args = std::env::args();
    let _bin_name = args.next().unwrap();
    let token = args.next().unwrap();
    let repository = args.next().unwrap();
    let config = Config::new(repository);

    let runner = Runner::new(token, config);
    if let Err(err) = runner.update_labels().await {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
