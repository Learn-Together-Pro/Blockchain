
#![ allow( unused_comparisons ) ]
#![ allow( non_snake_case ) ]

use lt_blockchain::blockchain::
{
  chain,
  digest,
  system,
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

fn chain_init() -> chain::Chain
{
  let root_digest = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  let mut receiver_map = HashMap::new();
  receiver_map.insert( root_digest.clone(), system::START_AMOUNT );
  let init_transaction_header = transaction::TransactionHeader
  {
    sender : root_digest,
    receiver : receiver_map,
    amount : system::START_AMOUNT,
    time : 100,
    body : ()
  };
  let init_transaction = init_transaction_header.form();
  chain::Chain::new( vec![ init_transaction ] )
}

//

#[ test ]
#[ ignore ]
fn new()
{
  println!( "empty initial transactions" );
  let chain = chain::Chain::new( vec![] );
  assert_eq!( chain.blocks.len(), 1 );
  let block =  chain.blocks[ 0 ].clone();
  let block_header =  block.header().clone();
  assert_eq!( block.body.hash, digest::hash_single( &block_header ) );
  assert_eq!( block.body.transactions.len(), 0 );
  assert!( block.nonce >= 0 );

  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.difficulty, 2 );
  assert_eq!( chain.miner_addr, digest::Digest::from( Vec::from( "Miner1".as_bytes() ) ) );
  assert_eq!( chain.reward, 100.0 );

  println!( "initial transaction exists" );
  let transaction = transaction_empty_create();
  let chain = chain::Chain::new( vec![ transaction.clone() ] );
  assert_eq!( chain.blocks.len(), 1 );
  let block =  chain.blocks[ 0 ].clone();
  let block_header =  block.header().clone();
  assert_eq!( block.body.hash, digest::hash_single( &block_header ) );
  assert_eq!( block.body.transactions.len(), 1 );
  assert_eq!( block.body.transactions[ 0 ], transaction.clone() );
  assert!( block.nonce >= 0 );

  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.difficulty, 2 );
  assert_eq!( chain.miner_addr, digest::Digest::from( Vec::from( "Miner1".as_bytes() ) ) );
  assert_eq!( chain.reward, 100.0 );
}

//

#[ test ]
#[ ignore ]
fn hash_last()
{
  let chain_original = chain_init();

  /* */

  println!( "initialized chain, only genesis block" );
  let chain = chain_original.clone();
  assert_eq!( chain.blocks[ 0 ].pre_hash.clone(), digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ) );
  let hash = chain.hash_last();
  assert_ne!( hash, digest::Digest::from( Vec::from([ 0u8 ; 32 ]) ) );
  let difficulty = chain.blocks[ 0 ].difficulty as usize;
  let mut exp : Vec<u8> = vec![];
  for _i in 0..difficulty { exp.push( 0 ) }
  assert_eq!( hash[ 0..difficulty ], exp );

  println!( "genesis block and mined block" );
  let mut chain = chain_original.clone();
  let transaction = transaction_empty_create();
  chain.transactions_pool.insert( transaction.body.hash.clone(), transaction );
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
#[ ignore ]
fn block_mine()
{
  let chain_original = chain_init();

  /* */

  println!( "no transactions in transactions_pool" );
  let chain = chain_original.clone();
  let block = chain.block_mine();
  assert!( block.is_none() );

  println!( "genesis block and mined block" );
  let mut chain = chain_original.clone();
  let transaction = transaction_empty_create();
  chain.transactions_pool.insert( transaction.body.hash.clone(), transaction );
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
#[ ignore ]
fn block_add()
{
  let chain_original = chain_init();

  /* */

  println!( "mined block from all transactions mined block" );
  let mut chain = chain_original.clone();
  let mut transaction = transaction_empty_create();
  transaction.body.hash = digest::hash_single( transaction.header() );
  chain.transactions_pool.insert( transaction.body.hash.clone(), transaction );
  let block = chain.block_mine().unwrap();
  assert_eq!( chain.transactions_pool.len(), 1 );
  assert_eq!( chain.blocks.len(), 1 );
  chain.block_add( block.clone() );
  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.blocks.len(), 2 );
  assert_eq!( chain.blocks[ 1 ], block.clone() );

  println!( "transaction is added before block is added to chain" );
  let mut chain = chain_original.clone();
  let transaction = transaction_empty_create();
  chain.transactions_pool.insert( transaction.body.hash.clone(), transaction );
  let block = chain.block_mine().unwrap();
  assert_eq!( chain.transactions_pool.len(), 1 );
  assert_eq!( chain.blocks.len(), 1 );
  let mut transaction = transaction_empty_create();
  transaction.time = 10000;
  transaction.body.hash = digest::hash_single( transaction.header() );
  chain.transactions_pool.insert( transaction.body.hash.clone(), transaction.clone() );
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

//

#[ test ]
#[ ignore ]
fn first_block_valid_is()
{
  let chain_original = chain_init();
  let init_transaction = chain_original.blocks[ 0 ].body.transactions[ 0 ].clone();

  /* */

  println!( "with valid first transaction" );
  let chain = chain_original.clone();
  let got = chain.first_block_valid_is();
  assert_eq!( got, true );

  println!( "no transactions in first block" );
  let chain = chain::Chain::new( vec![] );
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "first transaction has invalid amount" );
  let mut transaction = init_transaction.clone();
  transaction.amount = 100.0;
  let chain = chain::Chain::new( vec![ transaction ] );
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "first transaction has invalid amount send to receiver" );
  let mut transaction = init_transaction.clone();
  let root_digest = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  transaction.receiver.insert( root_digest, 100.0 );
  let chain = chain::Chain::new( vec![ transaction ] );
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "first transaction has different sender and receiver" );
  let mut transaction = init_transaction.clone();
  transaction.sender = digest::Digest::from( Vec::from( "another".as_bytes() ) );
  let chain = chain::Chain::new( vec![ transaction ] );
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "first transaction has modified data" );
  let mut chain = chain_original.clone();
  let mut transaction = &mut chain.blocks[ 0 ].body.transactions[ 0 ];
  transaction.time = 10000;
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "first transaction has modified body hash" );
  let mut chain = chain_original.clone();
  let mut transaction = &mut chain.blocks[ 0 ].body.transactions[ 0 ];
  transaction.body.hash = digest::Digest::from( Vec::from([ 0u8; 32 ]) );
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "block has valid transaction but not valid data" );
  let mut chain = chain_original.clone();
  chain.blocks[ 0 ].time = 10000;
  let got = chain.first_block_valid_is();
  assert_eq!( got, false );

  println!( "block not not valid transaction and modified data, updated" );
  let mut chain = chain_original.clone();
  let mut transaction = &mut chain.blocks[ 0 ].body.transactions[ 0 ];
  transaction.time = 10000;
  transaction.body.hash = digest::hash_single( transaction.header() );
  chain.blocks[ 0 ].time = 10000;
  chain.blocks[ 0 ].merkle_hash = digest::merkle_calc( &chain.blocks[ 0 ].body.transactions );
  chain.blocks[ 0 ].body.hash = chain.blocks[ 0 ].header_mut().proof_of_work();
  let got = chain.first_block_valid_is();
  assert_eq!( got, true );

  println!( "block has valid transaction and modified data, updated" );
  let mut chain = chain_original.clone();
  chain.blocks[ 0 ].time = 10000;
  chain.blocks[ 0 ].body.hash = chain.blocks[ 0 ].header_mut().proof_of_work();
  let got = chain.first_block_valid_is();
  assert_eq!( got, true );
}

//

#[ test ]
#[ ignore ]
fn valid_is()
{
  let root_digest = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  let mut chain_original = chain_init();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain_original.transaction_create( root_digest.clone(), receiver.clone(), 10.0 );
  let block = chain_original.block_mine();
  chain_original.block_add( block.unwrap() );
  chain_original.transaction_create( root_digest.clone(), receiver.clone(), 10.0 );
  let block = chain_original.block_mine();
  chain_original.block_add( block.unwrap() );

  /* */

  println!( "valid chain" );
  let chain = chain_original.clone();
  let got = chain.valid_is();
  assert_eq!( got, true );

  println!( "invalid first block" );
  let mut chain = chain_original.clone();
  chain.blocks[ 0 ].time = 10000;
  let got = chain.valid_is();
  assert_eq!( got, false );

  println!( "invalid second block" );
  let mut chain = chain_original.clone();
  chain.blocks[ 1 ].time = 10000;
  let got = chain.valid_is();
  assert_eq!( got, false );

  println!( "invalid third block" );
  let mut chain = chain_original.clone();
  chain.blocks[ 2 ].time = 10000;
  let got = chain.valid_is();
  assert_eq!( got, false );

  /* */

  println!( "first block modified and made valid" );
  let mut chain = chain_original.clone();
  chain.blocks[ 0 ].time = 10000;
  chain.blocks[ 0 ].body.hash = chain.blocks[ 0 ].header_mut().proof_of_work();
  let got = chain.valid_is();
  assert_eq!( got, false );

  println!( "second block modified and made valid" );
  let mut chain = chain_original.clone();
  chain.blocks[ 1 ].time = 10000;
  chain.blocks[ 1 ].body.hash = chain.blocks[ 1 ].header_mut().proof_of_work();
  let got = chain.valid_is();
  assert_eq!( got, false );

  println!( "third block modified and made valid" );
  let mut chain = chain_original.clone();
  chain.blocks[ 2 ].time = 10000;
  chain.blocks[ 2 ].body.hash = chain.blocks[ 2 ].header_mut().proof_of_work();
  let got = chain.valid_is();
  assert_eq!( got, true );
}

