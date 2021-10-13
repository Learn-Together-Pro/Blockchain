// #![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;

use serde_with::serde_as;

use super::wt;
use super::transaction::*;
use super::digest::*;
use super::block::*;
use super::system::*;

#[ serde_as ]
#[ derive( Clone, Serialize, Deserialize, Debug, PartialEq ) ]
pub struct Chain
{
  pub blocks : Vec< Block >,
  #[ serde_as( as = "HashMap<serde_with::json::JsonString, _>" ) ]
  pub transactions_pool : HashMap< Digest, Transaction >,
  pub difficulty : u32,
  pub miner_addr : Digest,
  pub reward : f64
}

//

impl Chain
{

  //

  pub fn new( _transactions : Vec<Transaction> ) -> Chain
  {

    Chain
    {
      blocks : Vec::new(),
      transactions_pool : HashMap::new(),
      difficulty : 2,
      miner_addr : Digest::from( Vec::from( "Miner1".as_bytes() ) ),
      reward : 100.0
    }

    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/19
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L52
    complexity : mid
    stage : early
    */

  }

  //

  pub fn hash_last( &self ) -> Digest
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/9
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/chain_test.rs#L98
    complexity : easy
    stage : early
    */

    let last_block = &self.blocks.last();
    let last_block_hash: Digest = match last_block {
      Some(last_block) => last_block.to_owned().body.hash.clone(),
      None => {
        let zero : Vec< u8 > = [ 0 ; 32 ].into();
        Digest::from( zero.clone() )
      }
    };
    last_block_hash
  }

  //

  pub fn block_mine( &self ) -> Option< Block >
  {
    if self.transactions_pool.len() > 0
    {
      let time = wt::time::s::now();
      let block_header = BlockHeader
      {
        time,
        nonce : 0,
        pre_hash : self.hash_last(),
        merkle_hash : Digest::new(),
        difficulty : self.difficulty,
        body : (),
      };
      let mut block = block_header.form();

      let root_digest = &self.blocks[ 0 ].body.transactions[ 0 ].sender;
      let sender = root_digest.clone();
      let mut receiver_map = HashMap::new();
      let balance = self.balance_get( &sender );
      if balance >= self.reward
      {
        receiver_map.insert( self.miner_addr.clone(), self.reward );
        receiver_map.insert( sender.clone(), balance - self.reward );
      }
      else
      {
        eprintln!( "System has no enough coins to pay reward." );
        return None;
      }

      let reward_trans_body = TransactionHeader
      {
        sender,
        receiver : receiver_map,
        amount : self.reward,
        time,
        body : ()
      };
      let reward_trans = reward_trans_body.form();
      block.body.transactions.push( reward_trans );
      block.body.transactions.extend( self.transactions_pool.values().cloned() );

      block.merkle_hash = merkle_calc( &block.body.transactions );
      block.body.hash = block.header_mut().proof_of_work();

      println!( "ntransactions : {}", block.body.transactions.len() );
      println!( "hash : {:?}", block.body.hash );

      return Some( block );
    }
    None
  }

  //

  pub fn block_add( &mut self, block : Block )
  {
    println!( "Adding block : {:#?}", &block );

    if !block.valid_is()
    {
      eprintln!( "Block : {:#?} \nBlock is not valid!", &block );
      return;
    }

    let root_digest = self.blocks[ 0 ].body.transactions[ 0 ].sender.clone();
    for t in &block.body.transactions
    {
      if !self.transactions_pool.contains_key( &t.body.hash )
      {
        if t.sender != root_digest
        {
          eprintln!( "Unknown transaction : {:#?} \nBlock is not valid!", t );
          return;
        }
      }
    }

    for t in &block.body.transactions
    {
      let hash = &t.body.hash;
      self.transactions_pool.remove( hash );
    }

    self.blocks.push( block );
  }

  //

  pub fn valid_is( &self ) -> bool
  {
    if !self.first_block_valid_is()
    {
      return false;
    }

    let mut pre_hash = &self.blocks[ 0 ].body.hash;
    for i in 1..self.blocks.len()
    {
      if &self.blocks[ i ].pre_hash != pre_hash
      {
        return false;
      }
      if !&self.blocks[ i ].valid_is()
      {
        return false;
      }
      pre_hash = &self.blocks[ i ].body.hash;
    }
    true
  }

  //

  pub fn first_block_valid_is( &self ) -> bool
  {
    let block = &self.blocks[ 0 ];
    if block.pre_hash != Digest::from( Vec::from([ 0u8 ; 32]) )
    {
      return false;
    }
    if block.body.transactions.len() != 1
    {
      return false;
    }

    let transaction = &block.body.transactions[ 0 ];

    if transaction.amount != START_AMOUNT
    {
      return false;
    }

    let sender = transaction.sender.clone();
    let receiver = transaction.receiver.clone();
    let amount_opt = receiver.get( &sender );
    if receiver.len() != 1 || amount_opt.is_none() || *amount_opt.unwrap() != START_AMOUNT
    {
      return false;
    }

    if !block.valid_is()
    {
      return false;
    }
    true
  }
}

