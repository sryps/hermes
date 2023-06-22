use alloc::collections::BTreeSet;

use async_trait::async_trait;

use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::{HasCounterpartyMessageHeight, HasIbcChainTypes};
use crate::logger::traits::level::HasBaseLogLevels;
use crate::relay::traits::ibc_message_sender::IbcMessageSender;
use crate::relay::traits::logs::logger::CanLogRelayTarget;
use crate::relay::traits::messages::update_client::CanBuildUpdateClientMessage;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

pub struct SendIbcMessagesWithUpdateClient<Sender>(pub Sender);

#[async_trait]
impl<Sender, Relay, Target, TargetChain, CounterpartyChain> IbcMessageSender<Relay, Target>
    for SendIbcMessagesWithUpdateClient<Sender>
where
    Relay: CanLogRelayTarget<Target>,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    Sender: IbcMessageSender<Relay, Target>,
    TargetChain: HasIbcChainTypes<CounterpartyChain>,
    TargetChain: HasCounterpartyMessageHeight<CounterpartyChain>,
    CounterpartyChain: HasIbcChainTypes<TargetChain> + CanIncrementHeight,
    Relay: CanBuildUpdateClientMessage<Target>,
{
    async fn send_messages(
        relay: &Relay,
        messages: Vec<TargetChain::Message>,
    ) -> Result<Vec<Vec<TargetChain::Event>>, Relay::Error> {
        let source_heights: BTreeSet<CounterpartyChain::Height> = messages
            .iter()
            .flat_map(|message| TargetChain::counterparty_message_height(message).into_iter())
            .collect();

        let mut in_messages = Vec::new();

        for height in source_heights {
            // IBC requires the update client height to be at least one greater than the proof height
            let update_height = CounterpartyChain::increment_height(&height)
                .map_err(Target::counterparty_chain_error)?;

            let messages = relay
                .build_update_client_messages(Target::default(), &update_height)
                .await?;

            let messages_count = messages.len();

            relay.log_relay_target(
                Relay::Logger::LEVEL_TRACE,
                "built update client messages for sending message at height",
                |log| {
                    log.display("height", &height)
                        .display("messages_count", &messages_count);
                },
            );

            in_messages.extend(messages);
        }

        let update_messages_count = in_messages.len();

        in_messages.extend(messages);

        let in_events = Sender::send_messages(relay, in_messages).await?;

        let events = in_events.into_iter().skip(update_messages_count).collect();

        Ok(events)
    }
}