use anyhow::{anyhow, Result as AnyResult};
use bastion::{
    context::BastionContext, distributor::Distributor, message::MessageHandler, Bastion,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::domain::dto::SendMessage;

pub struct Test {
    pub calls: Mutex<Vec<String>>,
}

pub static TEST_INSTANCE: Lazy<Test> = Lazy::new(|| Test {
    calls: Default::default(),
});

impl Test {
    pub fn init() -> AnyResult<()> {
        Bastion::supervisor(|supervisor| {
            supervisor.children(|children| {
                children
                    .with_redundancy(5)
                    .with_distributor(Distributor::named("test_chat_adapter"))
                    .with_exec(handle_chat)
            })
        })
        .map_err(|_| anyhow!("adapter_secondary chat::test: could not be initialized"))?;

        Ok(())
    }
}

async fn handle_chat(ctx: BastionContext) -> Result<(), ()> {
    loop {
        MessageHandler::new(ctx.recv().await?).on_tell(|send_message: SendMessage, _| {
            tracing::info!("asked to send a message: {}", send_message.message);

            let mut calls = TEST_INSTANCE
                .calls
                .lock()
                .expect("lock was poisoned: chat_test send_message");

            calls.push(send_message.message);

            tracing::info!("calling : {:?}", calls);
        });
    }
}
