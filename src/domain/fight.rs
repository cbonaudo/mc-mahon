use anyhow::{anyhow, Error as AnyError};
use bastion::{distributor::Distributor, run};

use crate::domain::dto::{GetFinisher, GetUpperHand, SendMessage, UpperHand};

use super::dto::Fighters;

#[derive(Clone)]
pub struct Context {
    fight_adapter: Distributor,
    chat_adapter: Distributor,
}

impl Context {
    pub fn new(fight_adapter_name: &str, chat_adapter_name: &str) -> Self {
        let fight_adapter = Distributor::named(fight_adapter_name);
        let chat_adapter = Distributor::named(chat_adapter_name);

        Self {
            fight_adapter,
            chat_adapter,
        }
    }

    pub async fn resolve_fight(&self, fighters: Fighters) -> Result<(), AnyError> {
        run!(async {
            let upper_hand: UpperHand = self
                .fight_adapter
                .request(GetUpperHand::from(fighters))
                .await
                .expect("couldn't receive reply: get_upper_hand")
                .map_err(|err| anyhow!("could not get the upper hand: {}", err))?;

            let finisher: String = self
                .fight_adapter
                .request(GetFinisher::from(upper_hand))
                .await
                .expect("couldn't receive reply: get_finisher")
                .map_err(|err| anyhow!("could not get the finisher: {}", err))?;

            self.chat_adapter
                .tell_one(SendMessage { message: finisher })
                .map_err(|err| anyhow!("could not get the finisher: {}", err))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use bastion::Bastion as BASTION;
    use once_cell::sync::Lazy;

    use super::Context;
    use crate::{
        adapters_secondary::{self, chat::test},
        domain::dto::Fighters,
    };

    static CONTEXT: Lazy<Context> = Lazy::new(|| {
        // // Tracing: uncomment if needed
        // let subscriber = tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::INFO)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber).unwrap();

        BASTION::init();

        adapters_secondary::fight::Random::init().unwrap();
        adapters_secondary::chat::test::Test::init().unwrap();

        BASTION::start();

        sleep(Duration::from_secs(1));

        Context::new("fight_adapter", "test_chat_adapter")
    });

    #[tokio::test]
    // #[ignore]
    async fn name_handling() {
        let fighter1 = "Robin Sterieux".to_owned();
        let fighter2 = "Triple Pontage".to_owned();

        CONTEXT
            .resolve_fight(Fighters {
                fighter1: fighter1.clone(),
                fighter2: fighter2.clone(),
            })
            .await
            .unwrap();

        sleep(Duration::from_secs(1));

        {
            let test_calls = test::TEST_INSTANCE.calls.lock().unwrap();

            let matching_call = test_calls
                .iter()
                .find(|c| c.contains(&fighter1) && c.contains(&fighter2));

            assert!(matching_call.is_some());
        }
    }
}
