
#![ allow( non_snake_case ) ]

use std::fs;
use serial_test::*;

use lt_blockchain::blockchain::{ digest, system };

//

#[ test ]
#[ ignore ]
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
#[ ignore ]
fn valid_is()
{
  let system_original = system::System::new();

  /* */

  println!( "valid system" );
  let system = system_original.clone();
  let got = system.valid_is();
  assert_eq!( got, true );

  println!( "system has no root wallet" );
  let mut system = system_original.clone();
  system.wallets.remove( &String::from( "root" ) ).unwrap();
  let got = system.valid_is();
  assert_eq!( got, false );

  println!( "system has invalid public key of root wallet" );
  let mut system = system_original.clone();
  let key = digest::Digest::from( Vec::from([ 0u8 ; 270 ]) );
  let mut root_wallet = system.wallets.get_mut( &String::from( "root" ) ).unwrap();
  root_wallet.public_key = key;
  let got = system.valid_is();
  assert_eq!( got, false );
}

//

#[ test ]
#[ ignore ]
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
  assert_eq!( fs::read_to_string( &system.store_path ).is_err(), true );
}

//

#[ test ]
#[ ignore ]
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
  assert_eq!( fs::read_to_string( &system.store_path ).is_ok(), true );
  fs::remove_file( &system.store_path ).unwrap();
}

//

#[ test ]
#[ ignore ]
fn StorePathDefault()
{
  fs::remove_file( system::System::StorePathDefault() ).unwrap_or_default();

  /* */

  println!( "no system with persistent" );
  let path = system::System::StorePathDefault();
  assert_eq!( path.exists(), false );
  assert_eq!( path.is_file(), false );
  assert_eq!( path.is_absolute(), true );
  assert_eq!( path.file_name().unwrap().to_str().unwrap(), "chain.json" );

  println!( "system with persistent" );
  let system = system::System::MakePersistant();
  let path = system::System::StorePathDefault();
  assert_eq!( path.exists(), true );
  assert_eq!( path.is_file(), true );
  assert_eq!( path.is_absolute(), true );
  assert_eq!( path.file_name().unwrap().to_str().unwrap(), "chain.json" );
  fs::remove_file( &system.store_path ).unwrap();
}

//

#[ test ]
#[ ignore ]
#[ serial ]
fn Load()
{
  fs::remove_file( system::System::StorePathDefault() ).unwrap_or_default();

  /* */

  println!( "no file" );
  let got = system::System::Load();
  assert_eq!( got.is_err(), true );

  println!( "wrong schema of json file" );
  let path = system::System::StorePathDefault();
  fs::write( &path, "{}" ).unwrap();
  let got = system::System::Load();
  assert_eq!( got.is_err(), true );
  fs::remove_file( &path ).unwrap();

  println!( "load from file" );
  let system = system::System::MakePersistant();
  let got = system::System::Load();
  assert_eq!( got.is_ok(), true );
  assert_eq!( got.unwrap(), system );
  fs::remove_file( &system.store_path ).unwrap();
}

//

#[ test ]
#[ ignore ]
#[ serial ]
fn LoadFromFile()
{
  fs::remove_file( system::System::StorePathDefault() ).unwrap_or_default();

  /* */

  println!( "no file" );
  let path = system::System::StorePathDefault();
  let got = system::System::LoadFromFile( &path );
  assert_eq!( got.is_err(), true );

  println!( "wrong schema of json file" );
  let path = system::System::StorePathDefault();
  fs::write( &path, "{}" ).unwrap();
  let got = system::System::LoadFromFile( &path );
  assert_eq!( got.is_err(), true );
  fs::remove_file( &path ).unwrap();

  println!( "load from file" );
  let system = system::System::MakePersistant();
  let path = system::System::StorePathDefault();
  let got = system::System::LoadFromFile( &path );
  assert_eq!( got.is_ok(), true );
  assert_eq!( got.unwrap(), system );
  fs::remove_file( &system.store_path ).unwrap();
}

//

#[ test ]
#[ ignore ]
#[ serial ]
fn store()
{
  let store_path = system::System::StorePathDefault();
  fs::remove_file( &store_path ).unwrap_or_default();

  /* */

  println!( "store system to user path" );
  let system = system::System::new();
  assert_eq!( system.store_path.exists(), false );
  assert_eq!( system.store_path.is_file(), false );
  system.store();
  assert_eq!( system.store_path.exists(), true );
  assert_eq!( system.store_path.is_file(), true );
  let got = system::System::LoadFromFile( &system.store_path );
  assert_eq!( got.is_ok(), true );
  assert_eq!( got.unwrap(), system );
  fs::remove_file( &store_path ).unwrap();
}

//

#[ test ]
#[ ignore ]
fn store_to()
{
  let store_path = system::System::StorePathDefault().with_file_name( "store.json" );
  fs::remove_file( &store_path ).unwrap_or_default();

  /* */

  println!( "store system to user path" );
  let path = store_path.clone();
  let system = system::System::new();
  system.store_to( &path );
  assert_eq!( path.exists(), true );
  assert_eq!( path.is_file(), true );
  let got = system::System::LoadFromFile( &path );
  assert_eq!( got.is_ok(), true );
  assert_eq!( got.unwrap(), system );
  fs::remove_file( &path ).unwrap();
}

