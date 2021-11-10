use issue_deadline_manager::{
    label::DeadlineLabel,
    runner::{Runner, RunnerConfig},
};

#[tokio::main]
async fn main() {
    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let config = RunnerConfig {
        owner: "ikanago".to_string(),
        repository: "issue-deadline-manager".to_string(),
    };

    let runner = Runner::new(token, config);
    if let Err(err) = runner.register_labels(&[DeadlineLabel::Outdated]).await {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
