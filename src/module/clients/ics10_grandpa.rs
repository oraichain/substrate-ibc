use crate::module::core::ics24_host::Height;
use alloc::string::String;
use codec::{Decode, Encode};
use core::str::FromStr;
use flex_error::{define_error, DisplayOnly};
use ibc::{
	clients::ics10_grandpa::{
		client_state::ClientState as IbcClientState,
		help::{
			Commitment, MmrRoot as IbcMmrRoot, SignedCommitment, ValidatorMerkleProof, ValidatorSet,
		},
	},
	core::ics24_host::{error::ValidationError, identifier::ChainId},
};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

define_error! {
	#[derive(Debug, PartialEq, Eq)]
	Error {
		InvalidFromUtf8
			[DisplayOnly<alloc::string::FromUtf8Error>]
			| _ | { "invalid from utf8 error" },
		InvalidDecode
			[DisplayOnly<codec::Error>]
			| _ | { "invalid decode error" },
		ParseTimestampFailed
			[DisplayOnly<ibc::timestamp::ParseTimestampError>]
			| _ | { "invalid parse timestamp error" },
		ValidationFailed
			[DisplayOnly<ValidationError>]
			| _ | { "invalid validation error"},
		InvalidChainId
			[DisplayOnly<core::convert::Infallible>]
			|_| { "invalid chain id error" },
	}
}

/// A structure representing the MMR root. Doc: https://paritytech.github.io/substrate/master/pallet_mmr/index.html
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct MmrRoot {
	pub signed_commitment: Vec<u8>,
	pub validator_merkle_proofs: Vec<Vec<u8>>,
	pub mmr_leaf: Vec<u8>,
	pub mmr_leaf_proof: Vec<u8>,
}

impl From<IbcMmrRoot> for MmrRoot {
	fn from(val: IbcMmrRoot) -> Self {
		let encode_validator_proofs = val
			.validator_merkle_proofs
			.into_iter()
			.map(|validator_proof| ValidatorMerkleProof::encode(&validator_proof))
			.collect();

		Self {
			signed_commitment: SignedCommitment::encode(&val.signed_commitment),
			validator_merkle_proofs: encode_validator_proofs,
			mmr_leaf: val.mmr_leaf,
			mmr_leaf_proof: val.mmr_leaf_proof,
		}
	}
}

impl TryFrom<MmrRoot> for IbcMmrRoot {
	type Error = Error;
	fn try_from(value: MmrRoot) -> Result<Self, Self::Error> {
		let decode_validator_proofs: Vec<ValidatorMerkleProof> = value
			.validator_merkle_proofs
			.into_iter()
			.map(|validator_proof| {
				ValidatorMerkleProof::decode(&mut &validator_proof[..]).unwrap() // TODO
			})
			.collect();
		Ok(IbcMmrRoot {
			signed_commitment: SignedCommitment::decode(&mut &value.signed_commitment[..])
				.map_err(Error::invalid_decode)?,
			validator_merkle_proofs: decode_validator_proofs,
			mmr_leaf: value.mmr_leaf,
			mmr_leaf_proof: value.mmr_leaf_proof,
		})
	}
}

/// A structure representing the client state under BEEFY protocol
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ClientState {
	pub chain_id: Vec<u8>,
	/// block_number is height?
	pub latest_height: u64,
	/// Block height when the client was frozen due to a misbehaviour
	pub frozen_height: Option<Height>,
	pub latest_commitment: Vec<u8>,
	pub validator_set: Vec<u8>,
}
impl From<IbcClientState> for ClientState {
	fn from(val: IbcClientState) -> Self {
		Self {
			chain_id: val.chain_id.as_str().as_bytes().to_vec(),
			latest_height: val.latest_height,
			frozen_height: val.frozen_height.map(|val| val.into()),
			latest_commitment: Commitment::encode(&val.latest_commitment),
			validator_set: ValidatorSet::encode(&val.validator_set),
		}
	}
}

impl TryFrom<ClientState> for IbcClientState {
	type Error = Error;
	fn try_from(value: ClientState) -> Result<Self, Self::Error> {
		let chain_id_str = String::from_utf8(value.chain_id).map_err(Error::invalid_from_utf8)?;
		Ok(IbcClientState {
			chain_id: ChainId::from_str(&chain_id_str).map_err(Error::invalid_chain_id)?,
			latest_height: value.latest_height,
			frozen_height: value.frozen_height.map(|value| value.into()),
			latest_commitment: Commitment::decode(&mut &value.latest_commitment[..])
				.map_err(Error::invalid_decode)?,
			validator_set: ValidatorSet::decode(&mut &value.validator_set[..])
				.map_err(Error::invalid_decode)?,
		})
	}
}
