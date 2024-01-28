use anyhow::Result;

use serenity::builder::{CreateEmbed, ExecuteWebhook};
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serenity::model::Color;

pub enum Stage {
    PushDetected,
    Reloading{_try: u16},
    Finish{_try: u16},
    Fail
}

pub struct Notifier {
    http: Http,
    webhook: Webhook
}

impl Notifier {
    pub async fn new(webhook_url: &str) -> Result<Self> {
        
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, webhook_url)
            .await?;

        Ok(Self{
            http,
            webhook
        })
    }
}

impl Notifier {
    pub async fn send(&self, stage: Stage, content: &str) -> Result<()> {
        let (title, color) = match stage {
            Stage::PushDetected => ("Received push.".to_string(), Color::from_rgb(255, 255, 255)),
            Stage::Reloading { _try } => (format!("Attempting to reload (try {_try})."), Color::TEAL),
            Stage::Finish { _try } => (format!("Finished reloading (try {_try})."), Color::GOLD),
            Stage::Fail => (format!("Failed to reload."), Color::RED)
        };

        let builder = ExecuteWebhook::new().username("Webhook Notifier")
        .embed(
            CreateEmbed::new().title(title)
            .color(color)
            .field("http://niooi.studio", content, true)
        );
        self.webhook.execute(&self.http, false, builder).await?;

        Ok(())
    }
}