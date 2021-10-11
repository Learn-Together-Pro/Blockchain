use std::fmt;

use super::chain::Chain;
use super::system::*;
use super::wallet::Wallet;

//

#[ derive( Clone, Serialize, Deserialize, PartialEq ) ]
pub struct Miner
{
  pub name : String,
  pub wallet : Wallet,
  pub chain : Chain
}

impl Miner
{
  pub fn new( chain: Chain, wallet: Wallet ) -> Miner
  {
    let miner = Miner
    {
      name : wallet.name.clone(),
      wallet,
      chain,
    };
    miner
  }

  //

  pub fn chain_sync( &mut self, chain : &Chain )
  {
    self.chain = chain.clone();
  }
}

//

impl fmt::Debug for Miner
{
  fn fmt( &self, formatter : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    formatter.debug_struct( "Miner" )
    .field( "name", &self.name )
    .field( "wallet", &self.wallet)
    .field( "chain", &self.chain )
    .finish()
  }
}

//

impl System
{
  pub fn miner_create( &mut self, name : &String ) -> Option<Miner>
  {
    let mut chain = self.chain.clone();
    let wallet = self.wallets.get( name );
    if wallet.is_none()
    {
      return None;
    }
    let wallet = wallet.unwrap();

    chain.miner_addr = wallet.public_key.clone();

    let miner = Miner::new( chain, wallet.clone() );
    self.miners.insert( name.clone(), miner.clone() );
    Some( miner )
  }
}

