
#![ allow( non_snake_case ) ]

use lt_blockchain::blockchain::{ block, digest };

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
  let got = block.header().proof_of_work();
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
  let hash2 = block.header().proof_of_work();
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
  let hash2 = block.header().proof_of_work();
  assert_ne!( hash1, hash2 );
}

