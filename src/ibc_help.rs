use crate::*;
use ibc::events::IbcEvent;

pub fn event_from_ibc_event<T: Config>(value: IbcEvent) -> Event<T> {
	match value {
		IbcEvent::NewBlock(value) => Event::NewBlock { height: value.height.into() },
		IbcEvent::CreateClient(value) => {
			let height = value.0.height;
			let client_id = value.0.client_id;
			let client_type = value.0.client_type;
			let consensus_height = value.0.consensus_height;
			Event::CreateClient {
				height: height.into(),
				client_id: client_id.into(),
				client_type: client_type.into(),
				consensus_height: consensus_height.into(),
			}
		},
		IbcEvent::UpdateClient(value) => {
			let height = value.common.height;
			let client_id = value.common.client_id;
			let client_type = value.common.client_type;
			let consensus_height = value.common.consensus_height;
			Event::UpdateClient {
				height: height.into(),
				client_id: client_id.into(),
				client_type: client_type.into(),
				consensus_height: consensus_height.into(),
			}
		},
		// Upgrade client events are not currently being used
		IbcEvent::UpgradeClient(value) => {
			let height = value.0.height;
			let client_id = value.0.client_id;
			let client_type = value.0.client_type;
			let consensus_height = value.0.consensus_height;
			Event::UpgradeClient {
				height: height.into(),
				client_id: client_id.into(),
				client_type: client_type.into(),
				consensus_height: consensus_height.into(),
			}
		},
		IbcEvent::ClientMisbehaviour(value) => {
			let height = value.0.height;
			let client_id = value.0.client_id;
			let client_type = value.0.client_type;
			let consensus_height = value.0.consensus_height;
			Event::ClientMisbehaviour {
				height: height.into(),
				client_id: client_id.into(),
				client_type: client_type.into(),
				consensus_height: consensus_height.into(),
			}
		},
		IbcEvent::OpenInitConnection(value) => {
			let height = value.attributes().height;
			let connection_id: Option<ConnectionId> =
				value.attributes().connection_id.clone().map(|val| val.into());
			let client_id = value.attributes().client_id.clone();
			let counterparty_connection_id: Option<ConnectionId> =
				value.attributes().counterparty_connection_id.clone().map(|val| val.into());

			let counterparty_client_id = value.attributes().counterparty_client_id.clone();
			Event::OpenInitConnection {
				height: height.into(),
				connection_id,
				client_id: client_id.into(),
				counterparty_connection_id,
				counterparty_client_id: counterparty_client_id.into(),
			}
		},
		IbcEvent::OpenTryConnection(value) => {
			let height = value.attributes().height;
			let connection_id: Option<ConnectionId> =
				value.attributes().connection_id.clone().map(|val| val.into());
			let client_id = value.attributes().client_id.clone();
			let counterparty_connection_id: Option<ConnectionId> =
				value.attributes().counterparty_connection_id.clone().map(|val| val.into());

			let counterparty_client_id = value.attributes().counterparty_client_id.clone();
			Event::OpenTryConnection {
				height: height.into(),
				connection_id,
				client_id: client_id.into(),
				counterparty_connection_id,
				counterparty_client_id: counterparty_client_id.into(),
			}
		},
		IbcEvent::OpenAckConnection(value) => {
			let height = value.attributes().height;
			let connection_id: Option<ConnectionId> =
				value.attributes().connection_id.clone().map(|val| val.into());
			let client_id = value.attributes().client_id.clone();
			let counterparty_connection_id: Option<ConnectionId> =
				value.attributes().counterparty_connection_id.clone().map(|val| val.into());

			let counterparty_client_id = value.attributes().counterparty_client_id.clone();
			Event::OpenAckConnection {
				height: height.into(),
				connection_id,
				client_id: client_id.into(),
				counterparty_connection_id,
				counterparty_client_id: counterparty_client_id.into(),
			}
		},
		IbcEvent::OpenConfirmConnection(value) => {
			let height = value.attributes().height;
			let connection_id: Option<ConnectionId> =
				value.attributes().connection_id.clone().map(|val| val.into());
			let client_id = value.attributes().client_id.clone();
			let counterparty_connection_id: Option<ConnectionId> =
				value.attributes().counterparty_connection_id.clone().map(|val| val.into());

			let counterparty_client_id = value.attributes().counterparty_client_id.clone();
			Event::OpenConfirmConnection {
				height: height.into(),
				connection_id,
				client_id: client_id.into(),
				counterparty_connection_id,
				counterparty_client_id: counterparty_client_id.into(),
			}
		},
		IbcEvent::OpenInitChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id.clone();
			let counterparty_channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			Event::OpenInitChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::OpenTryChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id.clone();
			let counterparty_channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			Event::OpenTryChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::OpenAckChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id.clone();
			let counterparty_channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			Event::OpenAckChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::OpenConfirmChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id;
			let counterparty_channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			Event::OpenConfirmChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::CloseInitChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = Some(value.channel_id.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id;
			let counterparty_channel_id: Option<ChannelId> =
				value.counterparty_channel_id.map(|val| val.into());
			Event::CloseInitChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::CloseConfirmChannel(value) => {
			let height = value.height;
			let port_id = value.port_id.clone();
			let channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			let connection_id = value.connection_id.clone();
			let counterparty_port_id = value.counterparty_port_id.clone();
			let counterparty_channel_id: Option<ChannelId> = value.channel_id.map(|val| val.into());
			Event::CloseConfirmChannel {
				height: height.into(),
				port_id: port_id.into(),
				channel_id,
				connection_id: connection_id.into(),
				counterparty_port_id: counterparty_port_id.into(),
				counterparty_channel_id,
			}
		},
		IbcEvent::SendPacket(value) => {
			let height = value.height;
			let packet = value.packet;
			Event::SendPacket { height: height.into(), packet: packet.into() }
		},
		IbcEvent::ReceivePacket(value) => {
			let height = value.height;
			let packet = value.packet;
			Event::ReceivePacket { height: height.into(), packet: packet.into() }
		},
		IbcEvent::WriteAcknowledgement(value) => {
			let height = value.height;
			let packet = value.packet;
			let ack = value.ack;
			Event::WriteAcknowledgement { height: height.into(), packet: packet.into(), ack }
		},
		IbcEvent::AcknowledgePacket(value) => {
			let height = value.height;
			let packet = value.packet;
			Event::AcknowledgePacket { height: height.into(), packet: packet.into() }
		},
		IbcEvent::TimeoutPacket(value) => {
			let height = value.height;
			let packet = value.packet;
			Event::TimeoutPacket { height: height.into(), packet: packet.into() }
		},
		IbcEvent::TimeoutOnClosePacket(value) => {
			let height = value.height;
			let packet = value.packet;
			Event::TimeoutOnClosePacket { height: height.into(), packet: packet.into() }
		},
		IbcEvent::AppModule(_) => Event::AppModule,
		IbcEvent::Empty(value) => Event::Empty(value.as_bytes().to_vec()),
		IbcEvent::ChainError(value) => Event::ChainError(value.as_bytes().to_vec()),
	}
}
