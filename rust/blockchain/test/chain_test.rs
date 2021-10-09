
#![ allow( unused_comparisons ) ]
#![ allow( non_snake_case ) ]

use lt_blockchain::blockchain::
{
  chain,
  digest,
  transaction,
};
use std::collections::HashMap;

//

fn transaction_empty_create() -> transaction::Transaction
{
  let transaction_header = transaction::TransactionHeader
  {
    sender : digest::Digest::new(),
    receiver : HashMap::new(),
    amount : 1.0,
    time : 100,
    body : ()
  };
  transaction_header.form()
}

//

#[ test ]
fn new()
{
  println!( "empty initial transactions" );
  let chain = chain::Chain::new( vec![] );
  assert_eq!( chain.blocks.len(), 1 );
  let mut block =  chain.blocks[ 0 ].clone();
  let block_header =  block.header().clone();
  assert_eq!( block.body.hash, digest::hash_single( &block_header ) );
  assert_eq!( block.body.transactions.len(), 0 );
  assert!( block.nonce >= 0 );

  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.difficulty, 2 );
  assert_eq!( chain.miner_addr, "Miner1" );
  assert_eq!( chain.reward, 100.0 );

  println!( "initial transaction exists" );
  let transaction = transaction_empty_create();
  let chain = chain::Chain::new( vec![ transaction.clone() ] );
  assert_eq!( chain.blocks.len(), 1 );
  let mut block =  chain.blocks[ 0 ].clone();
  let block_header =  block.header().clone();
  assert_eq!( block.body.hash, digest::hash_single( &block_header ) );
  assert_eq!( block.body.transactions.len(), 1 );
  assert_eq!( block.body.transactions[ 0 ], transaction.clone() );
  assert!( block.nonce >= 0 );

  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.difficulty, 2 );
  assert_eq!( chain.miner_addr, "Miner1" );
  assert_eq!( chain.reward, 100.0 );
}

//

#[ test ]
fn transaction_create()
{
  let mut transaction = transaction_empty_create();
  transaction.sender = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  transaction.receiver.insert( transaction.sender.clone(), 100.0 );
  let original_chain = chain::Chain::new( vec![ transaction.clone() ] );

  /* */

  println!( "make transaction from user with coins" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver, 10.0 );
  assert_eq!( chain.transactions_pool.len(), 1 );
  let keys = chain.transactions_pool.keys();
  assert_eq!( keys.len(), 1 );
  for e in keys
  {
    assert_eq!( e.len(), 32 );
  }
  for v in chain.transactions_pool.values()
  {
    assert_eq!( v.sender, sender.clone() );
    let mut receiver_expected = HashMap::new();
    receiver_expected.insert( digest::Digest::from( Vec::from([ 1u8 ; 32 ]) ), 10.0 );
    receiver_expected.insert( transaction.sender.clone(), 90.0 );
    assert_eq!( v.receiver, receiver_expected );
    assert_eq!( v.amount, 100.0 );
  }

  println!( "make transaction from user that does not exists == user without coins, amount - 0.0" );
  let mut chain = original_chain.clone();
  let sender = digest::Digest::from( Vec::from( "not_existed".as_bytes() ) );
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver, 0.0 );
  assert_eq!( chain.transactions_pool.len(), 1 );
  let keys = chain.transactions_pool.keys();
  assert_eq!( keys.len(), 1 );
  for e in keys
  {
    assert_eq!( e.len(), 32 );
  }
  for v in chain.transactions_pool.values()
  {
    assert_eq!( v.sender, sender.clone() );
    let mut receiver_expected = HashMap::new();
    receiver_expected.insert( digest::Digest::from( Vec::from([ 1u8 ; 32 ]) ), 0.0 );
    assert_eq!( v.receiver, receiver_expected );
    assert_eq!( v.amount, 0.0 );
  }

  println!( "make transaction from user that does not exists == user without coins, amount - 1.0" );
  let mut chain = original_chain.clone();
  let sender = digest::Digest::from( Vec::from( "not_existed".as_bytes() ) );
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver, 1.0 );
  assert_eq!( chain.transactions_pool.len(), 0 );
}

//

#[ test ]
fn balance_get()
{
  let mut transaction = transaction_empty_create();
  transaction.sender = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  transaction.receiver.insert( transaction.sender.clone(), 100.0 );
  let original_chain = chain::Chain::new( vec![ transaction.clone() ] );

  /* */

  println!( "get balance of not existed user" );
  let chain = original_chain.clone();
  let sender = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 0.0 );

  println!( "get balance of existed user, user do not transactions" );
  let chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let got = chain.balance_get( &sender );
  assert_eq!( got, 100.0 );

  println!( "get balance of existed user, user do transactions" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver, 10.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 90.0 );

  println!( "get balance of existed user, user do two transactions to same user" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 80.0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 0.0 );

  println!( "get balance of existed user, user do two transactions to different users" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 0.0 );
  let receiver = digest::Digest::from( Vec::from([ 2u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 0.0 );
  assert_eq!( chain.transactions_pool.len(), 2 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 80.0 );

  //

  println!( "get balance of existed user, user do two transactions to different users, mined block" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver1 = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver1.clone(), 10.0 );
  let receiver2 = digest::Digest::from( Vec::from([ 2u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver2.clone(), 10.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  assert_eq!( chain.transactions_pool.len(), 0 );
  let got = chain.balance_get( &receiver1 );
  assert_eq!( got, 10.0 );
  let got = chain.balance_get( &receiver2 );
  assert_eq!( got, 10.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 80.0 );

  println!( "several transactions to user" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  chain.transaction_create( receiver.clone(), sender.clone(), 5.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  assert_eq!( chain.transactions_pool.len(), 0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 5.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 95.0 );

  println!( "several transactions to user" );
  let mut chain = original_chain.clone();
  let sender = transaction.sender.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  chain.transaction_create( receiver.clone(), sender.clone(), 5.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let block = chain.block_mine().unwrap();
  chain.block_add( block );
  assert_eq!( chain.transactions_pool.len(), 0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 15.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 85.0 );
}

//

#[ test ]
fn hash_last()
{
  println!( "initialized chain, only genesis block" );
  let chain = chain::Chain::new( vec![] );
  assert_eq!( chain.blocks[ 0 ].pre_hash.clone(), digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ) );
  let hash = chain.hash_last();
  assert_ne!( hash, digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ) );
  let difficulty = chain.blocks[ 0 ].difficulty as usize;
  let mut exp : Vec<u8> = vec![];
  for _i in 0..difficulty { exp.push( 0 ) }
  assert_eq!( hash[ 0..difficulty ], exp );

  println!( "genesis block and mined block" );
  let mut chain = chain::Chain::new( vec![] );
  let transaction = transaction_empty_create();
  chain.transactions_pool.insert( digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ), transaction );
  let block = chain.block_mine().unwrap();
  chain.block_add( block.clone() );
  let hash = chain.hash_last();
  assert_eq!( hash.len(), 32 );
  assert_eq!( hash, block.body.hash.clone() );
  let difficulty = chain.blocks[ 0 ].difficulty as usize;
  let mut exp : Vec<u8> = vec![];
  for _i in 0..difficulty { exp.push( 0 ) }
  assert_eq!( hash[ 0..difficulty ], exp );
}

//

#[ test ]
fn block_mine()
{
  println!( "no transactions in transactions_pool" );
  let chain = chain::Chain::new( vec![] );
  let block = chain.block_mine();
  assert!( block.is_none() );

  println!( "genesis block and mined block" );
  let mut chain = chain::Chain::new( vec![] );
  let transaction = transaction_empty_create();
  chain.transactions_pool.insert( digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ), transaction );
  let block = chain.block_mine();
  assert_eq!( chain.transactions_pool.len(), 1 );
  assert!( block.is_some() );
  let block = block.unwrap();
  assert_eq!( block.merkle_hash, digest::merkle_calc( &block.body.transactions ) );
  let difficulty = block.difficulty as usize;
  let mut exp : Vec<u8> = vec![];
  for _i in 0..difficulty { exp.push( 0 ) }
  assert_eq!( block.body.hash[ 0..difficulty ], exp );
  assert_eq!( block.pre_hash.len(), 32 );
  assert_eq!( block.pre_hash[ 0..difficulty ], exp );
}

//

#[ test ]
fn block_add()
{
  println!( "mined block from all transactions mined block" );
  let mut chain = chain::Chain::new( vec![] );
  let mut transaction = transaction_empty_create();
  transaction.body.hash = digest::Digest::from( Vec::from([ 0u8 ; 32 ]) );
  chain.transactions_pool.insert( digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ), transaction );
  let block = chain.block_mine().unwrap();
  assert_eq!( chain.transactions_pool.len(), 1 );
  assert_eq!( chain.blocks.len(), 1 );
  chain.block_add( block.clone() );
  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.blocks.len(), 2 );
  assert_eq!( chain.blocks[ 1 ], block.clone() );

  println!( "transaction is added before block is added to chain" );
  let mut chain = chain::Chain::new( vec![] );
  let mut transaction = transaction_empty_create();
  transaction.body.hash = digest::Digest::from( Vec::from([ 0u8 ; 32 ]) );
  chain.transactions_pool.insert( digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ), transaction );
  let block = chain.block_mine().unwrap();
  assert_eq!( chain.transactions_pool.len(), 1 );
  assert_eq!( chain.blocks.len(), 1 );
  let mut transaction = transaction_empty_create();
  transaction.body.hash = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transactions_pool.insert( digest::Digest::from( Vec::from([ 1u8 ; 32 ]) ), transaction.clone() );
  assert_eq!( chain.transactions_pool.len(), 2 );
  chain.block_add( block.clone() );
  assert_eq!( chain.transactions_pool.len(), 1 );
  for t in chain.transactions_pool.values()
  {
    assert_eq!( t.clone(), transaction.clone() );
  }
  assert_eq!( chain.blocks.len(), 2 );
  assert_eq!( chain.blocks[ 1 ], block.clone() );
}

