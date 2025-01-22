#[cfg(not(feature = "sequentia"))] // use regular Bitcoin data structures
pub use bitcoin::{
    address, blockdata::block::Header as BlockHeader, blockdata::script, consensus::deserialize,
    hash_types::TxMerkleNode, Address, Block, BlockHash, OutPoint, ScriptBuf as Script, Sequence,
    Transaction, TxIn, TxOut, Txid,
};

#[cfg(feature = "sequentia")]
pub use {
    crate::elements::asset,
    elements::{
        address, confidential, encode::deserialize, script, Address, AssetId, Block, BlockHash,
        BlockHeader, OutPoint, Script, Sequence, Transaction, TxIn, TxMerkleNode, TxOut, Txid,
    },
};

use bitcoin::blockdata::constants::genesis_block;
pub use bitcoin::network::Network as BNetwork;

#[cfg(not(feature = "sequentia"))]
pub type Value = u64;
#[cfg(feature = "sequentia")]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    #[cfg(not(feature = "sequentia"))]
    Bitcoin,
    #[cfg(not(feature = "sequentia"))]
    Testnet,
    #[cfg(not(feature = "sequentia"))]
    Regtest,
    #[cfg(not(feature = "sequentia"))]
    Signet,

    #[cfg(feature = "sequentia")]
    Sequentia,
    #[cfg(feature = "sequentia")]
    SequentiaTestnet,
    #[cfg(feature = "sequentia")]
    SequentiaRegtest,
}

impl Network {
    #[cfg(not(feature = "sequentia"))]
    pub fn magic(self) -> u32 {
        u32::from_le_bytes(BNetwork::from(self).magic().to_bytes())
    }

    #[cfg(feature = "sequentia")]
    pub fn magic(self) -> u32 {
        match self {
            Network::Sequentia => 0xDAB5_BFFA,
            Network::SequentiaTestnet => 0xE0BA_01EF,
            Network::SequentiaRegtest => 0x0EF2_1953,
        }
    }

    pub fn is_regtest(self) -> bool {
        match self {
            #[cfg(not(feature = "sequentia"))]
            Network::Regtest => true,
            #[cfg(feature = "sequentia")]
            Network::SequentiaRegtest => true,
            _ => false,
        }
    }

    #[cfg(feature = "sequentia")]
    pub fn address_params(self) -> &'static address::AddressParams {
        // Sequentia regtest uses elements's address params
        match self {
            Network::Sequentia => &address::AddressParams::SEQUENTIA,
            Network::SequentiaTestnet => &address::AddressParams::SEQUENTIA_TESTNET,
            Network::SequentiaRegtest => &address::AddressParams::ELEMENTS,
        }
    }

    #[cfg(feature = "sequentia")]
    pub fn native_asset(self) -> &'static AssetId {
        match self {
            Network::Sequentia => &*asset::NATIVE_ASSET_ID,
            Network::SequentiaTestnet => &*asset::NATIVE_ASSET_ID_TESTNET,
            Network::SequentiaRegtest => &*asset::NATIVE_ASSET_ID_REGTEST,
        }
    }

    #[cfg(feature = "sequentia")]
    pub fn pegged_asset(self) -> Option<&'static AssetId> {
        match self {
             Network::Sequentia | Network::SequentiaTestnet | Network::SequentiaRegtest => None,
        }
    }

    pub fn names() -> Vec<String> {
        #[cfg(not(feature = "sequentia"))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "signet".to_string(),
        ];

        #[cfg(feature = "sequentia")]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
        ];
    }
}

pub fn genesis_hash(network: Network) -> BlockHash {
    #[cfg(not(feature = "sequentia"))]
    return bitcoin_genesis_hash(network.into());
    #[cfg(feature = "sequentia")]
    return sequentia_genesis_hash(network);
}

pub fn bitcoin_genesis_hash(network: BNetwork) -> bitcoin::BlockHash {
    lazy_static! {
        static ref BITCOIN_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Bitcoin).block_hash();
        static ref TESTNET_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Testnet).block_hash();
        static ref REGTEST_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Regtest).block_hash();
        static ref SIGNET_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Signet).block_hash();
    }
    match network {
        BNetwork::Bitcoin => *BITCOIN_GENESIS,
        BNetwork::Testnet => *TESTNET_GENESIS,
        BNetwork::Regtest => *REGTEST_GENESIS,
        BNetwork::Signet => *SIGNET_GENESIS,
        _ => panic!("unknown network {:?}", network),
    }
}

#[cfg(feature = "sequentia")]
pub fn sequentia_genesis_hash(network: Network) -> elements::BlockHash {
    use crate::util::DEFAULT_BLOCKHASH;

    lazy_static! {
        static ref SEQUENTIA_GENESIS: BlockHash =
            "1466275836220db2944ca059a3a10ef6fd2ea684b0688d2c379296888a206003"
                .parse()
                .unwrap();
        static ref SEQUENTIA_TESTNET_GENESIS: BlockHash =
            "997d61a708543ee56de675c9afebb690007793429967d7a28c61358a033766cd"
                .parse()
                .unwrap();
    }

    match network {
        Network::Sequentia => *SEQUENTIA_GENESIS,
        Network::SequentiaTestnet => *SEQUENTIA_TESTNET_GENESIS,
        // The genesis block for sequentia regtest chains varies based on the chain configuration.
        // This instead uses an all zeroed-out hash, which doesn't matter in practice because its
        // only used for Electrum server discovery, which isn't active on regtest.
        _ => *DEFAULT_BLOCKHASH,
    }
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            #[cfg(not(feature = "sequentia"))]
            "mainnet" => Network::Bitcoin,
            #[cfg(not(feature = "sequentia"))]
            "testnet1" => Network::Testnet,
            #[cfg(not(feature = "sequentia"))]
            "regtest" => Network::Regtest,
            #[cfg(not(feature = "sequentia"))]
            "signet" => Network::Signet,

            #[cfg(feature = "sequentia")]
            "mainnet" => Network::Sequentia,
            #[cfg(feature = "sequentia")]
            "testnet" => Network::SequentiaTestnet,
            #[cfg(feature = "sequentia")]
            _ => Network::SequentiaRegtest,

            #[cfg(not(feature = "sequentia"))]
            _ => panic!("unsupported Bitcoin network: {:?}", network_name),
        }
    }
}

#[cfg(not(feature = "sequentia"))]
impl From<Network> for BNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BNetwork::Bitcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,
            Network::Signet => BNetwork::Signet,
        }
    }
}

#[cfg(not(feature = "sequentia"))]
impl From<BNetwork> for Network {
    fn from(network: BNetwork) -> Self {
        match network {
            BNetwork::Bitcoin => Network::Bitcoin,
            BNetwork::Testnet => Network::Testnet,
            BNetwork::Regtest => Network::Regtest,
            BNetwork::Signet => Network::Signet,
            _ => panic!("unknown network {:?}", network),
        }
    }
}
