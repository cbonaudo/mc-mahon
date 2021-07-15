use anyhow::{anyhow, Result as AnyResult};
use bastion::{
    context::BastionContext, distributor::Distributor, message::MessageHandler, Bastion,
};
use rand::{prelude::SliceRandom, thread_rng, Rng};

use crate::domain::dto::{GetFinisher, GetUpperHand, UpperHand};

pub struct Random;

impl Random {
    pub fn init() -> AnyResult<()> {
        Bastion::supervisor(|supervisor| {
            supervisor.children(|children| {
                children
                    .with_redundancy(5)
                    .with_distributor(Distributor::named("fight_adapter"))
                    .with_exec(handle_users)
            })
        })
        .map_err(|_| anyhow!("adapter_secondary chat::API: could not be initialized"))?;

        Ok(())
    }
}

async fn handle_users(ctx: BastionContext) -> Result<(), ()> {
    loop {
        MessageHandler::new(ctx.recv().await?)
            .on_question(|get_upper_hand: GetUpperHand, sender| {
                tracing::info!("asked to get the upper hand: {} vs {}", get_upper_hand.fighter1, get_upper_hand.fighter2);

                let upper_hand = if get_upper_hand.fighters().find(|fighter| *fighter == get_upper_hand.champion_name).is_some() {
                    let vanquished = get_upper_hand.fighters().find(|fighter| *fighter != get_upper_hand.champion_name).unwrap_or_else(|| &get_upper_hand.champion_name).to_owned();

                    UpperHand {
                        victor: get_upper_hand.champion_name,
                        vanquished,
                    }
                } else {
                    let mut randomized = vec![get_upper_hand.fighter1, get_upper_hand.fighter2];
                    randomized.shuffle(&mut thread_rng());

                     UpperHand {
                        victor: randomized[0].clone(),
                        vanquished: randomized[1].clone(),
                    }
                };

                sender.reply(upper_hand).unwrap();
            })
            .on_question(|get_finisher: GetFinisher, sender| {
                tracing::info!("asked to get the finisher: {} vs {}", get_finisher.victor, get_finisher.vanquished);

                let random: i32 = thread_rng().gen_range(0..5);

                let finisher = match random {
                    0 => format!("Oh my god! {} smashed {} to the troat and FIRED them from life!", get_finisher.victor, get_finisher.vanquished),
                    1 => format!("{} shot an INSANE! high kick at {} and sent them flying!", get_finisher.victor, get_finisher.vanquished),
                    2 => format!("INCREDIBLE! {} threw {} to the ground and is now kicking them inconscious!", get_finisher.victor, get_finisher.vanquished),
                    3 => format!("A spectator gave a chair to {} who then SLAMMED it right on {}'s skull! That's illegal, but not forbidden by the rules.", get_finisher.victor, get_finisher.vanquished),
                    _ => format!("That's a THIRD CORD JUMP from {}, right on {} that was already lying on the ground!", get_finisher.victor, get_finisher.vanquished),
                };

                sender.reply(finisher).unwrap();
            });
    }
}

#[cfg(test)]
mod tests {
    use bastion::{distributor::Distributor, Bastion as BASTION};

    use super::Random;
    use crate::domain::dto::{GetFinisher, GetUpperHand, UpperHand};

    #[tokio::test]
    // #[ignore]
    async fn fight_random_adapter() {
        // // Tracing: uncomment if needed
        // let subscriber = tracing_subscriber::fmt()
        //     .with_max_level(Level::INFO)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber).unwrap();

        BASTION::init();
        Random::init().unwrap();
        BASTION::start();

        let fight_adapter = Distributor::named("fight_adapter");

        let fighter1 = "John dinner".to_string();
        let fighter2 = "The gravetaker".to_string();

        let get_upper_hand = GetUpperHand {
            fighter1: fighter1.clone(),
            fighter2: fighter2.clone(),
        };

        let upper_hand: UpperHand = fight_adapter
            .request(get_upper_hand)
            .await
            .expect("couldn't receive reply: get_upper_hand")
            .expect("could not get the upper hand:");

        assert!(upper_hand.victor == fighter1 || upper_hand.victor == fighter2);
        assert!((upper_hand.vanquished == fighter1 || upper_hand.vanquished == fighter2));
        assert!(upper_hand.victor != upper_hand.vanquished);

        let finisher: String = fight_adapter
            .request(GetFinisher::from(upper_hand))
            .await
            .expect("couldn't receive reply: get_finisher")
            .expect("could not get the finisher");

        assert!(finisher.contains(&fighter1) && finisher.contains(&fighter2));
    }
}
