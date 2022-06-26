use async_trait::async_trait;

use crate::traits::message_sender::IbcMessageSender;
use crate::traits::messages::receive_packet::ReceivePacketMessageBuilder;
use crate::traits::packet_relayer::PacketRelayer;
use crate::traits::queries::status::{ChainStatus, ChainStatusQuerier};
use crate::traits::relay_types::{RelayContext, RelayTypes};
use crate::types::aliases::Packet;

pub struct ReceivePacketRelayer;

#[async_trait]
impl<Context, Relay> PacketRelayer<Context> for ReceivePacketRelayer
where
    Relay: RelayTypes,
    Context: RelayContext<RelayTypes = Relay>,
    Context: ReceivePacketMessageBuilder<Relay>,
    Context::SrcChainContext: ChainStatusQuerier<Relay::SrcChain>,
    Context::DstChainContext: IbcMessageSender<Relay::DstChain, Relay::SrcChain>,
{
    type Return = ();

    async fn relay_packet(
        &self,
        context: &Context,
        packet: Packet<Relay>,
    ) -> Result<(), Relay::Error> {
        let source_height = context
            .source_context()
            .query_chain_status()
            .await?
            .height();

        let message = context
            .build_receive_packet_message(&source_height, &packet)
            .await?;

        context.destination_context().send_message(message).await?;

        Ok(())
    }
}
