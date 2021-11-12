use issue_deadline_manager::runner::{Config, Runner};

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
    //     if let Err(err) = runner.update_labels().await {
    if let Err(err) = runner
        .create_label(&issue_deadline_manager::label::DeadlineLabel::DaysBefore(3))
        .await
    {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
