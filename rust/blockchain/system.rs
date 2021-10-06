// #![allow(dead_code)]
#![allow(non_snake_case)]

use std::path::PathBuf;
use std::collections::HashMap;

use super::chain::*;
use super::wallet::*;

#[ derive( Clone, Serialize, Deserialize, Debug ) ]
pub struct System
{
  pub wallets : HashMap< String, Wallet >,
  pub store_path : PathBuf,
  pub chain : Chain,
}

impl System
{
  pub fn new() -> System
  {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/33
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
      store_path,
    }
  }
}
