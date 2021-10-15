
#![allow(unused_imports)]

use std::path::PathBuf;
use std::collections::HashMap;

use super::wt;
use super::chain::*;
use super::miner::*;
use super::transaction::*;
use super::wallet::*;

pub const START_AMOUNT : f64 = 1_000_000.0;

#[ derive( Clone, Serialize, Deserialize, Debug, PartialEq ) ]
pub struct System
{
  pub wallets : HashMap< String, Wallet >,
  pub miners : HashMap< String, Miner >,
  pub store_path : PathBuf,
  pub chain : Chain,
}

impl System
{
  pub fn new() -> System
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/33
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L13
    complexity : difficult
    stage : mid
    */

    let store_path = Self::StorePathDefault();
    let wallets = HashMap::new();
    let chain = Chain::new( vec![] );

    /* */

    System
    {
      chain,
      wallets,
      miners : HashMap::new(),
      store_path,
    }
  }

  //

  pub fn valid_is( &self ) -> bool
  {
    let root_wallet = self.wallets.get( &String::from( "root" ) );
    if root_wallet.is_none()
    {
      return false;
    }
    if root_wallet.unwrap().public_key != self.chain.blocks[ 0 ].body.transactions[ 0 ].sender
    {
      return false;
    }
    self.chain.valid_is()
  }
}
