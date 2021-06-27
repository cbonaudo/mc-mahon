use anyhow::{anyhow, Result as AnyResult};
use bastion::{
    context::BastionContext, distributor::Distributor, message::MessageHandler, run, Bastion,
};
use serde_json::json;

use crate::{
    config::{self, CONFIG},
    domain::dto::SendMessage,
};

pub struct API;

impl API {
    pub fn init() -> AnyResult<()> {
        Bastion::supervisor(|supervisor| {
            supervisor.children(|children| {
                children
                    .with_redundancy(5)
                    .with_distributor(Distributor::named("chat_adapter"))
                    .with_exec(handle_chat)
            })
        })
        .map_err(|_| anyhow!("adapter_secondary chat::API: could not be initialized"))?;

        Ok(())
    }
}

async fn handle_chat(ctx: BastionContext) -> Result<(), ()> {
    loop {
        MessageHandler::new(ctx.recv().await?).on_tell(|send_message: SendMessage, _| {
            tracing::info!("asked to send a message: {}", send_message.message);

            let target_channel_id = &CONFIG.target_channel_id;

            let request_url = format!(
                "https://discord.com/api/channels/{}/messages",
                target_channel_id
            );

            run!(async {
                surf::post(request_url)
                    .body(json!({
                        "content": send_message.message,
                    }))
                    .header("Authorization", config::get_header())
                    .send()
                    .await
                    .unwrap();
            });
        });
    }
}
