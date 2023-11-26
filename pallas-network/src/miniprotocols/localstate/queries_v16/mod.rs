// TODO: this should move to pallas::ledger crate at some point

use pallas_addresses::Address;
use pallas_crypto::hash::Hash;
use std::hash::Hash as StdHash;
// required for derive attrs to work
use pallas_codec::minicbor::{self};

use pallas_codec::utils::{AnyUInt, Bytes, KeyValuePairs};
use pallas_codec::{
    minicbor::{Decode, Encode},
    utils::AnyCbor,
};
use pallas_primitives::conway::Metadatum;

use crate::miniprotocols::Point;

use super::{Client, ClientError};

mod codec;

// https://github.com/input-output-hk/ouroboros-consensus/blob/main/ouroboros-consensus-cardano/src/shelley/Ouroboros/Consensus/Shelley/Ledger/Query.hs
#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum BlockQuery {
    GetLedgerTip,
    GetEpochNo,
    GetNonMyopicMemberRewards(AnyCbor),
    GetCurrentPParams,
    GetProposedPParamsUpdates,
    GetStakeDistribution,
    GetUTxOByAddress(Addrs),
    GetUTxOWhole,
    DebugEpochState,
    GetCBOR(AnyCbor),
    GetFilteredDelegationsAndRewardAccounts(AnyCbor),
    GetGenesisConfig,
    DebugNewEpochState,
    DebugChainDepState,
    GetRewardProvenance,
    GetUTxOByTxIn(AnyCbor),
    GetStakePools,
    GetStakePoolParams(AnyCbor),
    GetRewardInfoPools,
    GetPoolState(AnyCbor),
    GetStakeSnapshots(AnyCbor),
    GetPoolDistr(AnyCbor),
    GetStakeDelegDeposits(AnyCbor),
    GetConstitutionHash,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum HardForkQuery {
    GetInterpreter,
    GetCurrentEra,
}

pub type Proto = u16;
pub type Era = u16;

#[derive(Debug, Clone, PartialEq)]
pub enum LedgerQuery {
    BlockQuery(Era, BlockQuery),
    HardForkQuery(HardForkQuery),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    LedgerQuery(LedgerQuery),
    GetSystemStart,
    GetChainBlockNo,
    GetChainPoint,
}

#[derive(Debug, Encode, Decode, PartialEq)]
pub struct SystemStart {
    #[n(0)]
    pub year: u32,

    #[n(1)]
    pub day_of_year: u32,

    #[n(2)]
    pub picoseconds_of_day: u64,
}

#[derive(Debug, Encode, Decode, PartialEq)]
pub struct StakeDistribution {
    #[n(0)]
    pub pools: KeyValuePairs<Bytes, Pool>,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct Pool {
    #[n(0)]
    pub stakes: Fraction,

    #[n(1)]
    pub hashes: Bytes,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct Fraction {
    #[n(0)]
    pub num: u64,

    #[n(1)]
    pub dem: u64,
}

pub type Addr = Bytes;

pub type Addrs = Vec<Addr>;

pub type Coin = AnyUInt;

pub type Multiasset<A> = KeyValuePairs<A, Metadatum>;

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct UTxOByAddress {
    #[n(0)]
    pub utxo: KeyValuePairs<UTxO, Multiasset<AnyUInt>>,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone, StdHash, Eq)]
pub struct UTxO {
    #[n(0)]
    pub txid: Hash<32>,

    #[n(1)]
    pub idx: AnyUInt,
}

pub async fn get_chain_point(client: &mut Client) -> Result<Point, ClientError> {
    let query = Request::GetChainPoint;
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_current_era(client: &mut Client) -> Result<Era, ClientError> {
    let query = HardForkQuery::GetCurrentEra;
    let query = LedgerQuery::HardForkQuery(query);
    let query = Request::LedgerQuery(query);
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_system_start(client: &mut Client) -> Result<SystemStart, ClientError> {
    let query = Request::GetSystemStart;
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_block_epoch_number(client: &mut Client, era: u16) -> Result<u32, ClientError> {
    let query = BlockQuery::GetEpochNo;
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    let (result,): (_,) = client.query(query).await?;

    Ok(result)
}

pub async fn get_stake_distribution(
    client: &mut Client,
    era: u16,
) -> Result<StakeDistribution, ClientError> {
    let query = BlockQuery::GetStakeDistribution;
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    println!("query stake distribution: {:?}", query);
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_utxo_by_address(
    client: &mut Client,
    era: u16,
    address: String,
) -> Result<UTxOByAddress, ClientError> {
    println!("get_utxo_by_address");
    let addr: Address = Address::from_bech32(&address).unwrap();
    let addr: Addr = addr.to_vec().into();
    let addrs: Addrs = Vec::from([addr]);
    let query = BlockQuery::GetUTxOByAddress(addrs);
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    println!("query get_utxo_by_address: {:?}", query);
    let result = client.query(query).await?;

    Ok(result)
}
