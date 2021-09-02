// Copyright 2021 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use crate::{
    account_address::AccountAddress, block_info::PivotBlockDecision,
    validator_config::ConsensusSignature,
};
use diem_crypto::HashValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedBlock {
    pub hash: HashValue,
    pub miner: AccountAddress,
    pub parent_hash: HashValue,
    pub epoch: u64,
    pub round: u64,
    pub pivot_decision: PivotBlockDecision,
    pub version: u64,
    pub timestamp: u64,
    pub signatures: BTreeMap<AccountAddress, ConsensusSignature>,
}
