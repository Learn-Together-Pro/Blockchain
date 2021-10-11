#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::num::ParseIntError;
use std::borrow::{ Borrow, BorrowMut };
use std::collections::HashMap;
use std::collections::BTreeMap;
use serde::ser::*;
use serde::de::*;
use wtools as wt;

use super::wallet::*;
use super::system::*;
use super::digest::*;
use super::chain::*;

#[ repr( C ) ]
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct TransactionGeneric< T : ?Sized >
{
  pub sender : Digest,
  #[serde(serialize_with = "ordered_map_serialize", deserialize_with = "hash_map_deserialize")]
  pub receiver : HashMap< Digest, f64 >,
  pub amount : f64,
  pub time : i64,
  pub body : T,
}

fn ordered_map_serialize<S>( value: &HashMap<Digest, f64>, serializer: S ) -> Result<S::Ok, S::Error>
where S: Serializer,
{
  let mut ordered : BTreeMap<String, f64> = BTreeMap::new();
  for ( k, v ) in value
  {
    ordered.insert( bytes_to_string_hex( k ), *v );
  }
  ordered.serialize( serializer )
}

fn hash_map_deserialize<'de, D>( deserializer : D ) -> Result<HashMap<Digest, f64>, D::Error>
where D: Deserializer<'de>
{
  let buf : BTreeMap<String, f64> = BTreeMap::deserialize( deserializer )?;

  let mut hash_map : HashMap<Digest, f64> = HashMap::new();
  for ( k, v ) in buf
  {
    hash_map.insert
    (
      Digest::from
      (
        ( 0..k.len() )
        .step_by( 2 )
        .map( | i | u8::from_str_radix( &k[ i..i + 2 ], 16 ) )
        .collect::<Result<Vec<u8>, ParseIntError>>().unwrap()
      ),
      v
    );
  }
  Ok( hash_map )
}
pub type TransactionHeader = TransactionGeneric< () >;

//

#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
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

  //

  pub fn header( &self ) -> &TransactionHeader
  {
    self.borrow()
  }

  //

  pub fn header_mut( &mut self ) -> &mut TransactionHeader
  {
    self.borrow_mut()
  }

  //

  pub fn valid_is( &self ) -> bool
  {
    if hash_single( &self.header() ) != self.body.hash
    {
      return false;
    }
    true
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

  pub fn transaction_create( &mut self, _sender : Digest, _receiver : Digest, _amount : f64 )
  {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/7
    complexity : difficult
    stage : mid
    */
    unimplemented!( "not implemented" );
  }

  //

  pub fn balance_get( &self, _public_key : &Digest ) -> f64
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
