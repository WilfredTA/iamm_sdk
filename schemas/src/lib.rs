#![no_std]
extern crate no_std_compat as std;

use std::prelude::v1::*;
use trampoline_sdk::contract::schema::{TrampolineBaseSchema, SchemaPrimitiveType, BytesConversion, MolConversion};

use trampoline_sdk::{impl_entity_unpack, impl_pack_for_fixed_byte_array, impl_primitive_reader_unpack};
use trampoline_sdk::ckb_types::bytes::Bytes;
use trampoline_sdk::ckb_types::prelude::*;
mod nft;
use nft::*;

#[cfg(all(feature = "std", not(feature = "script")))]
use ckb_jsonrpc_types::JsonBytes;

#[cfg(all(feature = "std", not(feature = "script")))]
use trampoline_sdk::contract::schema::{TrampolineSchema};

#[cfg(all(feature = "std", not(feature = "script")))]
use trampoline_sdk::contract::schema::{JsonConversion, JsonByteConversion};

impl_pack_for_fixed_byte_array!([u8; 32], Byte32);
impl_primitive_reader_unpack!([u8; 32], Byte32Reader, 32, from);
impl_entity_unpack!([u8; 32], Byte32);

pub type GenesisId = SchemaPrimitiveType<[u8; 32], Byte32>;
pub type ContentId = SchemaPrimitiveType<[u8; 32], Byte32>;


#[derive(Debug, Clone, Default)]
pub struct IammNFT {
    pub genesis_id: GenesisId,
    pub cid: ContentId,
}

impl BytesConversion for IammNFT {
    fn from_bytes(bytes: Bytes) -> Self {
        let nft_mol = NFT::from_compatible_slice(&bytes.to_vec()).unwrap();
        Self {
            genesis_id: GenesisId::new(nft_mol.genesis_id().unpack()),
            cid: ContentId::new(nft_mol.content_id().unpack()),
        }
    }

    fn to_bytes(&self) -> Bytes {
        NFTBuilder::default()
            .content_id(self.cid.to_mol())
            .genesis_id(self.genesis_id.to_mol())
            .build()
            .as_bytes()
    }
}

impl From<IammNFT> for trampoline_sdk::bytes::Bytes {
    fn from(nft: IammNFT) -> Self {
        nft.to_bytes().into()
    }
}
#[cfg(all(feature = "std", not(feature = "script")))]
impl JsonByteConversion for IammNFT {
    fn to_json_bytes(&self) -> JsonBytes {
        todo!()
    }

    fn from_json_bytes(_bytes: JsonBytes) -> Self {
        todo!()
    }
}
#[cfg(all(feature = "std", not(feature = "script")))]
impl JsonConversion for IammNFT {
    type JsonType = JsonBytes;

    fn to_json(&self) -> Self::JsonType {
        self.to_json_bytes()
    }

    fn from_json(json: Self::JsonType) -> Self {
        Self::from_json_bytes(json)
    }
}

impl MolConversion for IammNFT {
    type MolType = NFT;

    fn to_mol(&self) -> Self::MolType {
        NFTBuilder::default()
            .content_id(self.cid.inner.pack())
            .genesis_id(self.genesis_id.inner.pack())
            .build()
    }

    fn from_mol(entity: Self::MolType) -> Self {
        Self {
            genesis_id: GenesisId::new(entity.genesis_id().unpack()),
            cid: ContentId::new(entity.content_id().unpack()),
        }
    }
}

impl TrampolineBaseSchema for IammNFT {}

#[cfg(all(feature = "std", not(feature = "script")))]
impl TrampolineSchema for IammNFT {}

pub type NftArgs = SchemaPrimitiveType<Bytes, trampoline_sdk::ckb_types::packed::Bytes>;


