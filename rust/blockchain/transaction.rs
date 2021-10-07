#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::borrow::{ Borrow, BorrowMut };
use std::collections::HashMap;
use serde_with::serde_as;
use wtools as wt;

use super::system::*;
use super::digest::*;
use super::chain::*;

#[ serde_as ]
#[ repr( C ) ]
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TransactionGeneric< T : ?Sized >
{
  pub sender : Digest,
  #[ serde_as( as = "HashMap<serde_with::json::JsonString, _>" ) ]
  pub receiver : HashMap< Digest, f64 >,
  pub amount : f64,
  pub time : i64,
  pub body : T,
}

pub type TransactionHeader = TransactionGeneric< () >;

//

#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TransactionRestricts
{
  pub hash : Digest,
}

pub type Transaction = TransactionGeneric< TransactionRestricts >;

//

impl TransactionHeader
{
  pub fn form( &self ) -> Transaction
  {
    Transaction::new( self )
  }
}

//

impl Transaction
{
  pub fn new( body : &TransactionHeader ) -> Transaction
  {
    let hash = hash_single( &body );
    Transaction
    {
      sender : body.sender.clone(),
      receiver : body.receiver.clone(),
      amount : body.amount,
      time : body.time,
      body : TransactionRestricts{ hash },
    }
  }
}

//

impl Borrow< TransactionHeader > for Transaction
{
  fn borrow<'a>( &'a self ) -> &'a TransactionHeader
  {
    // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
    unsafe
    {
      std::mem::transmute::< &'a Transaction, &'a TransactionHeader >( self )
    }
  }
}

//

impl BorrowMut< TransactionHeader > for Transaction
{
  fn borrow_mut<'a>( &'a mut self ) -> &'a mut TransactionHeader
  {
    // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
    unsafe
    {
      std::mem::transmute::< &'a mut Transaction, &'a mut TransactionHeader >( self )
    }
  }
}

//

impl System
{

  //

  pub fn transaction_create( &mut self, sender : String, receiver : String, amount : f64 )
  {
    let sender_digest = Digest::from( Vec::from( sender.as_bytes() ) );
    let receiver_digest = Digest::from( Vec::from( receiver.as_bytes() ) );
    self.chain.transaction_create( sender_digest, receiver_digest, amount );
  }

  //

}

//

impl Chain
{

  //

  pub fn transaction_create( &mut self, sender : Digest, _receiver : Digest, amount : f64 )
  {  
    //1. Go to system and check sender amount -> print("Not Enough money")
    let availableBalance = self.balance_get(&sender);

    if availableBalance >= amount {

      //2. Transaction create && Send remainig money to sender
      let mut receiver: HashMap<Digest, f64> = HashMap::new();
      receiver.insert(_receiver, amount);
      receiver.insert(sender.clone(), availableBalance - &amount);
      let time = wt::time::s::now();

      let transaction = TransactionHeader {
        sender: sender.clone(),
        receiver,
        amount,
        time,
        body:()
      };
   
       //3. Add to pool
      self.transactions_pool.insert(sender,transaction.form());

    } else {
      println!("Not Enough money")
    }
  }

  //

  pub fn balance_get( &mut self, _public_key : &Digest ) -> f64
  {
    1.0
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/1
    complexity : difficult
    stage : late
    */
  }

  //

}
