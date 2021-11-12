use issue_deadline_manager::{
    label::DeadlineLabel,
    runner::{Config, Runner},
};

#[tokio::main]
async fn main() {
    let (config, token) = match Config::initialize() {
        Ok((config, token)) => (config, token),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let runner = Runner::new(token, config);
    if let Err(err) = runner.update_labels().await {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
