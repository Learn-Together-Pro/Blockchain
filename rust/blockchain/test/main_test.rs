
use super::*;

//

#[ test ]
#[ ignore ]
fn command_system_wipe_test()
{
  println!( "wipe system" );
  let mut system = System::new();
  assert_eq!( system.wallets.len(), 1 );
  system.wallet_create( &String::from( "user" ) );
  assert_eq!( system.wallets.len(), 2 );
  command_system_wipe( &mut system );
  assert_eq!( system.wallets.len(), 1 );
}

//

#[ test ]
#[ ignore ]
fn command_block_mine_test()
{
  let system_original = System::new();

  /* */

  println!( "mine block without transactions" );
  let mut system = system_original.clone();
  let user_name = String::from( "user" );
  system.wallet_create( &user_name );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
  command_block_mine( &mut system );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );

  println!( "mine block with transactions" );
  let mut system = system_original.clone();
  let user_name = String::from( "user" );
  system.wallet_create( &user_name );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
  system.transaction_create( String::from( "root" ), user_name.clone(), 100.0 );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 1 );
  command_block_mine( &mut system );
  assert_eq!( system.chain.blocks.len(), 2 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
}

//

#[ test ]
#[ ignore ]
fn command_network_mine_test()
{
  let system_original = System::new();

  /* */

  println!( "mine block without transactions" );
  let mut system = system_original.clone();
  let root_name = String::from( "root" );
  let user_name = String::from( "user" );
  system.wallet_create( &user_name );
  system.miner_create( &root_name );
  system.miner_create( &user_name );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
  command_network_mine( &mut system );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );

  println!( "mine block with transactions" );
  let mut system = system_original.clone();
  let root_name = String::from( "root" );
  let user_name = String::from( "user" );
  system.wallet_create( &user_name );
  system.miner_create( &root_name );
  system.miner_create( &user_name );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
  system.transaction_create( String::from( "root" ), user_name.clone(), 100.0 );
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.transactions_pool.len(), 1 );
  command_block_mine( &mut system );
  assert_eq!( system.chain.blocks.len(), 2 );
  assert_eq!( system.chain.transactions_pool.len(), 0 );
}

