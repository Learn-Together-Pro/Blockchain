
#![ allow( unused_comparisons ) ]
#![ allow( non_snake_case ) ]

use std::fs;
use lt_blockchain::blockchain::system;

//

#[ test ]
fn new()
{
  println!( "empty initial transactions" );
  let system = system::System::new();
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.blocks[ 0 ].body.transactions.len(), 1 );
  assert_eq!( system.store_path.clone(), system::System::StorePathDefault() );
  assert_eq!( system.wallets.len(), 1 );
  let wallet = system.wallets.get( &String::from( "root" ) );
  assert_eq!( wallet.is_some(), true );
  assert_eq!( system.chain.balance_get( &wallet.unwrap().public_key.clone() ), system::START_AMOUNT );
}

//

#[ test ]
fn Make()
{
  println!( "empty initial transactions" );
  let system = system::System::Make();
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.blocks[ 0 ].body.transactions.len(), 1 );
  assert_eq!( system.store_path.clone(), system::System::StorePathDefault() );
  assert_eq!( system.wallets.len(), 1 );
  let wallet = system.wallets.get( &String::from( "root" ) );
  assert_eq!( wallet.is_some(), true );
  assert_eq!( system.chain.balance_get( &wallet.unwrap().public_key.clone() ), system::START_AMOUNT );
  assert_eq!( fs::read_to_string( system.store_path ).is_err(), true );
}

//

#[ test ]
fn MakePersistant()
{
  println!( "empty initial transactions" );
  let system = system::System::MakePersistant();
  assert_eq!( system.chain.blocks.len(), 1 );
  assert_eq!( system.chain.blocks[ 0 ].body.transactions.len(), 1 );
  assert_eq!( system.store_path.clone(), system::System::StorePathDefault() );
  assert_eq!( system.wallets.len(), 1 );
  let wallet = system.wallets.get( &String::from( "root" ) );
  assert_eq!( wallet.is_some(), true );
  assert_eq!( system.chain.balance_get( &wallet.unwrap().public_key.clone() ), system::START_AMOUNT );
  assert_eq!( fs::read_to_string( system.store_path.clone() ).is_ok(), true );
  fs::remove_file( system.store_path.clone() ).unwrap();
}

