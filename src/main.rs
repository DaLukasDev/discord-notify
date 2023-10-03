use serde::Deserialize;
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::utils::Color;

#[derive(Deserialize, Debug)]
struct Config {
    webhook_id: u64,
    webhook_token: String,
    is_success: bool,
    short_commit_hash: String,
    commit_hash: String,
    commit_message: String,
    environment: String,
    project_url: String,
    project_name: String,
    public_url: String,
}

#[tokio::main]
async fn main() {
    let config = envy::from_env::<Config>().unwrap();
    let http = Http::new(&config.webhook_token);
    let color_to_display = match config.is_success {
        true => Color::DARK_GREEN,
        false => Color::RED,
    };

    let title = match config.is_success {
        true => format!(
            "Deploy {} succeeded ({})",
            config.project_name, config.environment
        ),
        false => format!(
            "Deploy {} failed ({})",
            config.project_name, config.environment
        ),
    };

    let timestamp = chrono::Utc::now();
    let webhook = http
        .get_webhook_with_token(config.webhook_id, &config.webhook_token)
        .await
        .unwrap();

    let embed = Embed::fake(|e| {
        e.title(title);
        e.field(":hash: Commit hash:", &config.short_commit_hash, true);
        e.field(
            ":link: Public URL:",
            format!("https://{}", config.public_url),
            true,
        );
        e.field(":dart: Environment:", &config.environment, true);
        e.field(
            ":page_facing_up: Commit message:",
            &config.commit_message,
            false,
        );
        e.color(color_to_display);
        e.url(format!(
            "{}/-/commit/{}",
            config.project_url, config.commit_hash
        ));
        e.timestamp(timestamp.to_rfc3339());
        e
    });

    let _ = webhook
        .execute(&http, false, |w| {
            w.embeds(vec![embed]);
            w
        })
        .await;
}
