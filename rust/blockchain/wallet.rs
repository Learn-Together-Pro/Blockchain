#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::str;

use super::digest::*;
use super::system::*;

//

#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct Wallet
{
  pub name : String,
  pub public_key : Digest,
  pub private_key : Digest,
}

//

impl Wallet
{

  //

  pub fn new< 'a, 'b >( _wallets : &'a mut HashMap< String, Wallet >, _name : &'b String ) -> Option< &'a Wallet >
  {
    None
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/4
    complexity : mid
    stage : late
    */
  }

  //

  fn keys_pair_generate() -> ( Digest, Digest )
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/3
    complexity : mid
    stage : mid
    */
    let zero : Vec< u8 > = [ 0 ; 64 ].into();
    ( Digest::from( zero.clone() ), Digest::from( zero.clone() ) )
  }

  //

}

//

impl System
{

  //

  pub fn wallet_create( &mut self, name : &String ) -> Option< &Wallet >
  {
    Wallet::new( &mut self.wallets, name )
  }

  //

}
