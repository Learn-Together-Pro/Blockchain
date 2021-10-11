
use lt_blockchain::blockchain::{ miner, system };

//

#[ test ]
fn new()
{
  let system = system::System::new();

  /* */

  println!( "check constructor" );
  let wallet = system.wallets.get( &String::from( "root" ) ).unwrap();
  let got = miner::Miner::new( system.chain.clone(), wallet.clone() );
  assert_eq!( got.name, "root" );
  assert_eq!( got.chain, system.chain );
  assert_eq!( &got.wallet, wallet );
}

//

#[ test ]
fn chain_sync()
{
  let system_original = system::System::new();

  /* */

  println!( "check synchronized chains" );
  let mut system = system_original.clone();
  let root_name = String::from( "root" );
  let user_name = String::from( "user" );
  system.wallet_create( &user_name ).unwrap();
  let wallet = system.wallets.get( &root_name ).unwrap();
  let mut got = miner::Miner::new( system.chain.clone(), wallet.clone() );
  assert_eq!( got.chain, system.chain );
  system.transaction_create( root_name.clone(), user_name.clone(), 100.0 );
  assert_ne!( got.chain, system.chain );
  got.chain_sync( &system.chain );
  assert_eq!( got.chain, system.chain );
}

//

#[ test ]
fn miner_create()
{
  let system_original = system::System::new();

  /* */

  println!( "add miner, wallet exists" );
  let mut system = system_original.clone();
  let root_name = String::from( "root" );
  assert_eq!( system.miners.len(), 0 );
  let got = system.miner_create( &root_name );
  assert!( got.is_some() );
  let got = got.unwrap();
  assert_eq!( system.miners.len(), 1 );
  assert_eq!( got.name, "root" );
  assert_eq!( got.chain.blocks, system.chain.blocks );
  assert_eq!( got.chain.transactions_pool, system.chain.transactions_pool );
  assert_ne!( got.chain.miner_addr, system.chain.miner_addr );
  assert_eq!( &got.wallet, system.wallets.get( &root_name ).unwrap() );
  assert_eq!( system.miners.get( &root_name ).unwrap(), &got );

  /* */

  println!( "add miner, wallet does not exist" );
  let mut system = system_original.clone();
  let user_name = String::from( "user" );
  assert_eq!( system.miners.len(), 0 );
  let got = system.miner_create( &user_name );
  assert!( got.is_none() );
}

