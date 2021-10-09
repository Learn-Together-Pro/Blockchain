// #![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;

use serde_with::serde_as;

use super::wt;
use super::transaction::*;
use super::digest::*;
use super::block::*;

#[ serde_as ]
#[ derive( Clone, Serialize, Deserialize, Debug ) ]
pub struct Chain
{
  pub blocks : Vec< Block >,
  #[ serde_as( as = "HashMap<serde_with::json::JsonString, _>" ) ]
  pub transactions_pool : HashMap< Digest, Transaction >,
  pub difficulty : u32,
  pub miner_addr : String,
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
      blocks : vec![Block
                    {
                      time: wt::time::s::now(),
                      nonce: 0,
                      pre_hash: Digest::new(),
                      merkle_hash: Digest::new(),
                      difficulty: 2,
                      body: BlockBody{transactions: _transactions, hash: Digest::new()}
                    }],
      transactions_pool : HashMap::new(),
      difficulty : 2,
      miner_addr : "Miner1".to_string(),
      reward : 100.0
    }

  }

  //

  pub fn hash_last( &self ) -> Digest
  {
    let last_block = &self.blocks.last();
    let last_block_hash: Digest = match last_block {
      Some(last_block) => last_block.to_owned().body.hash.clone(),
      None => Digest::new()
    };

    last_block_hash
  }

  //

  pub fn block_mine( &self ) -> Option< Block >
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

    let sender = Digest::from( Vec::from( "root".as_bytes() ) );
    let mut receiver_map = HashMap::new();
    receiver_map.insert( Digest::from( Vec::from( self.miner_addr.as_bytes() ) ), self.reward );

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
    block.body.hash = block.header().proof_of_work();

    println!( "ntransactions : {}", block.body.transactions.len() );
    println!( "hash : {:?}", block.body.hash );

    Some( block )
  }

  //

  pub fn block_add( &mut self, block : Block )
  {
    println!( "Adding block : {:#?}", &block );

    for t in &block.body.transactions
    {
      let hash = &t.body.hash;
      self.transactions_pool.remove( hash );
    }

    self.blocks.push( block );
  }

}

