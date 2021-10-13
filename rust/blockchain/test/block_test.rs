
#![ allow( non_snake_case ) ]

use std::collections::HashMap;
use lt_blockchain::blockchain::{ block, digest, transaction };

//

#[ test ]
fn form()
{
  println!( "check constructor" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let got = block_header.form();
  assert_eq!( got.time, 100 );
  assert_eq!( got.nonce, 0 );
  assert_eq!( got.pre_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( got.merkle_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( got.difficulty, 2 );
  assert_eq!( got.body.hash.clone(), digest::Digest::new() );
  assert_eq!( got.body.transactions.clone(), vec![] );
}

//

#[ test ]
#[ ignore ]
fn proof_of_work()
{
  println!( "not formed block" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let mut block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let got = block_header.proof_of_work();
  assert_eq!( block_header.time, 100 );
  assert_eq!( block_header.pre_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( block_header.merkle_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( block_header.difficulty, 2 );
  assert_eq!( block_header.body.clone(), () );
  assert_eq!( got.len(), 32 );
  assert_eq!( got[ 0..2 ], vec![ 0, 0 ] );

  println!( "formed block" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let mut block = block_header.form();
  let got = block.header_mut().proof_of_work();
  assert_eq!( block.time, 100 );
  assert_eq!( block.pre_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( block.merkle_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( block.difficulty, 2 );
  assert_eq!( block.body.hash.clone(), digest::Digest::new() );
  assert_eq!( block.body.transactions.clone(), vec![] );
  assert_eq!( got.len(), 32 );
  assert_eq!( got[ 0..2 ], vec![ 0, 0 ] );

  println!( "same hash for only header and formed block, because data is not changed" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let mut block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let mut block = block_header.clone().form();
  let hash1 = block_header.proof_of_work();
  let hash2 = block.header_mut().proof_of_work();
  assert_eq!( hash1, hash2 );

  println!( "same different hashes for only header and formed block, because data is different" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let mut block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let mut block = block_header.clone().form();
  block.time = 101;
  let hash1 = block_header.proof_of_work();
  let hash2 = block.header_mut().proof_of_work();
  assert_ne!( hash1, hash2 );
}

//

#[ test ]
fn new()
{
  println!( "check constructor" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let got = block::Block::new( block_header );
  assert_eq!( got.time, 100 );
  assert_eq!( got.nonce, 0 );
  assert_eq!( got.pre_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( got.merkle_hash.clone(), digest::Digest::from( zero.clone() ) );
  assert_eq!( got.difficulty, 2 );
  assert_eq!( got.body.hash.clone(), digest::Digest::new() );
  assert_eq!( got.body.transactions.clone(), vec![] );
}

//

#[ test ]
fn header()
{
  println!( "check constructor" );
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let block = block::Block::new( block_header.clone() );
  let got = block.header();
  assert_eq!( got, &block_header );
}

//

#[ test ]
#[ ignore ]
fn valid_is()
{
  let sender = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  let mut transaction_header = transaction::TransactionHeader
  {
    sender,
    receiver : HashMap::new(),
    amount : 1.0,
    time : 100,
    body : ()
  };
  transaction_header.receiver.insert( transaction_header.sender.clone(), 100.0 );
  let transaction = transaction_header.form();
  let zero : Vec<u8> = Vec::from([ 0u8 ; 32]);
  let block_header = block::BlockHeader
  {
    time : 100,
    nonce : 0,
    pre_hash : digest::Digest::from( zero.clone() ),
    merkle_hash : digest::Digest::from( zero.clone() ),
    difficulty : 2,
    body : (),
  };
  let mut block_original = block_header.form();
  let transactions = vec![ transaction.clone(), transaction.clone() ];
  block_original.body.transactions = transactions;
  block_original.merkle_hash = digest::merkle_calc( &block_original.body.transactions );
  block_original.body.hash = block_original.header_mut().proof_of_work();

  /* */

  println!( "block is valid" );
  let block = block_original.clone();
  let got = block.valid_is();
  assert_eq!( got, true );

  println!( "block is not valid, changed transaction" );
  let mut block = block_original.clone();
  block.body.transactions[ 0 ].time = 101;
  let got = block.valid_is();
  assert_eq!( got, false );

  println!( "block is not valid, changed body hash" );
  let mut block = block_original.clone();
  block.body.hash = digest::Digest::from( zero.clone() );
  let got = block.valid_is();
  assert_eq!( got, false );

  println!( "block is not valid, changed data in header" );
  let mut block = block_original.clone();
  block.time = 10;
  let got = block.valid_is();
  assert_eq!( got, false );

  println!( "block is valid, changed data in transaction, recalculated merkle hash and block hash" );
  let mut block = block_original.clone();
  block.body.transactions[ 0 ].time = 10000;
  block.body.transactions[ 0 ].body.hash = digest::hash_single( block.body.transactions[ 0 ].header() );
  block.merkle_hash = digest::merkle_calc( &block.body.transactions );
  block.body.hash = block.header_mut().proof_of_work();
  let got = block.valid_is();
  assert_eq!( got, true );

  println!( "block is valid, changed data in header, recalculated block hash" );
  let mut block = block_original.clone();
  block.time = 10000;
  block.body.hash = block.header_mut().proof_of_work();
  let got = block.valid_is();
  assert_eq!( got, true );
}

