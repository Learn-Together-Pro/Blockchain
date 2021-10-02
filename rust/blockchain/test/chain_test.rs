#![ allow( non_snake_case ) ]

use lt_blockchain::blockchain::{chain, digest};

//

#[ test ]
fn new()
{
  let chain = chain::Chain::new();
  assert_eq!( chain.blocks.len(), 1 );
  let mut block =  chain.blocks[ 0 ].clone();
  let block_header =  block.header().clone();
  assert_eq!( block.body.hash, digest::hash_single( &block_header ) );
  assert_eq!( block.body.transactions.len(), 0 );
  assert_eq!( block.nonce, 0 );

  assert_eq!( chain.transactions_pool.len(), 0 );
  assert_eq!( chain.difficulty, 2 );
  assert_eq!( chain.miner_addr, "Miner1" );
  assert_eq!( chain.reward, 100.0 );
}