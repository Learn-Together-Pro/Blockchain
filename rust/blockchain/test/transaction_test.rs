
use std::collections::HashMap;
use lt_blockchain::blockchain::
{
  chain,
  digest,
  system,
  transaction,
};

//

#[ test ]
#[ ignore ]
fn form()
{
  println!( "check constructor" );
  let sender = digest::Digest::new();
  let mut receiver = HashMap::new();
  receiver.insert( sender.clone(), 100.0 );
  let transaction_header = transaction::TransactionHeader
  {
    sender,
    receiver,
    amount : 100.0,
    time : 100,
    body : (),
  };
  let clone = transaction_header.clone();
  let got = transaction_header.form();
  assert_eq!( got.sender.clone(), digest::Digest::new() );
  assert_eq!( got.receiver.len(), 1 );
  assert_eq!( *got.receiver.keys().next().unwrap(), digest::Digest::new() );
  assert_eq!( *got.receiver.values().next().unwrap(), 100.0 );
  assert_eq!( got.time, 100 );
  assert_eq!( got.amount, 100.0 );
  assert_eq!( digest::bytes_to_string_hex( &got.body.hash ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
  assert_eq!( digest::bytes_to_string_hex( &digest::hash_single( &clone ) ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
  assert_eq!( digest::bytes_to_string_hex( &digest::hash_single( &got.header() ) ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
}

//

#[ test ]
#[ ignore ]
fn new()
{
  println!( "check constructor" );
  let sender = digest::Digest::new();
  let mut receiver = HashMap::new();
  receiver.insert( sender.clone(), 100.0 );
  let transaction_header = transaction::TransactionHeader
  {
    sender,
    receiver,
    amount : 100.0,
    time : 100,
    body : (),
  };
  let clone = transaction_header.clone();
  let got = transaction::Transaction::new( &transaction_header );
  assert_eq!( got.sender.clone(), digest::Digest::new() );
  assert_eq!( got.receiver.len(), 1 );
  assert_eq!( *got.receiver.keys().next().unwrap(), digest::Digest::new() );
  assert_eq!( *got.receiver.values().next().unwrap(), 100.0 );
  assert_eq!( got.time, 100 );
  assert_eq!( got.amount, 100.0 );
  assert_eq!( digest::bytes_to_string_hex( &got.body.hash ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
  assert_eq!( digest::bytes_to_string_hex( &digest::hash_single( &clone ) ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
  assert_eq!( digest::bytes_to_string_hex( &digest::hash_single( &got.header() ) ), "1dbc47072efaca47255cd4b5de3b3b5474d372ef530515c8f9581646017257a1" );
}

//

#[ test ]
#[ ignore ]
fn transaction_create_from_system()
{
  println!( "transaction from root wallet to another" );
  let mut system = system::System::new();
  let root_wallet = system.wallets.get( &String::from( "root" ) ).unwrap().clone();
  let wallet_name = String::from( "user" );
  let wallet = system.wallet_create( &wallet_name.clone() ).unwrap().clone();
  assert_eq!( system.chain.transactions_pool.len(), 0 );
  system.transaction_create( String::from( "root" ), wallet_name.clone(), 100.0 );
  assert_eq!( system.chain.transactions_pool.len(), 1 );
  for e in system.chain.transactions_pool.keys()
  {
    assert_eq!( e.len(), 32 );
  }
  for v in system.chain.transactions_pool.values()
  {
    assert_eq!( v.sender.clone(), root_wallet.public_key.clone() );
    let mut receiver_expected = HashMap::new();
    receiver_expected.insert( wallet.public_key.clone(), 100.0 );
    receiver_expected.insert( root_wallet.public_key.clone(), system::START_AMOUNT - 100.0 );
    assert_eq!( v.receiver, receiver_expected );
    assert_eq!( v.amount, 100.0 );
  }
}

//

#[ test ]
#[ ignore ]
fn transaction_create_from_chain()
{
  /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/7

    To run test enter :
    cargo test transaction_test::transaction_create_from_chain -- --ignored

    When test will pass, remove directive `#[ ignore ]`.
  */

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
    assert_eq!( v.amount, 10.0 );
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
#[ ignore ]
fn balance_get()
{
  /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/1

    To run test enter :
    cargo test transaction_test::balance_get -- --ignored

    When test will pass, remove directive `#[ ignore ]`.
  */

  let root_digest = digest::Digest::from( Vec::from( "user".as_bytes() ) );
  let mut receiver_map = HashMap::new();
  receiver_map.insert( root_digest.clone(), 1_000_000.0 );
  let init_transaction_header = transaction::TransactionHeader
  {
    sender : root_digest.clone(),
    receiver : receiver_map,
    amount : 1_000_000.0,
    time : 100,
    body : ()
  };
  let init_transaction = init_transaction_header.form();
  let mut chain_original = chain::Chain::new( vec![ init_transaction.clone() ] );
  let receiver_original = digest::Digest::from( Vec::from([ 5u8 ; 32 ]) );
  chain_original.transaction_create( root_digest.clone(), receiver_original.clone(), 100.0 );
  chain_original.block_add( chain_original.block_mine().unwrap() );

  /* */

  println!( "get balance of not existed user" );
  let chain = chain_original.clone();
  let sender = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 0.0 );

  println!( "get balance of existed user, user do not transactions" );
  let chain = chain_original.clone();
  let sender = receiver_original.clone();
  let got = chain.balance_get( &sender );
  assert_eq!( got, 100.0 );

  println!( "get balance of existed user, user do transactions" );
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver, 10.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 90.0 );

  println!( "get balance of existed user, user do two transactions to same user" );
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
  let receiver = digest::Digest::from( Vec::from([ 1u8 ; 32 ]) );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  chain.transaction_create( sender.clone(), receiver.clone(), 10.0 );
  let got = chain.balance_get( &sender );
  assert_eq!( got, 80.0 );
  let got = chain.balance_get( &receiver );
  assert_eq!( got, 0.0 );

  println!( "get balance of existed user, user do two transactions to different users" );
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
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
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
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
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
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
  let mut chain = chain_original.clone();
  let sender = receiver_original.clone();
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
  let transaction_original = transaction_header.form();

  /* */

  println!( "transaction is valid" );
  let transaction = transaction_original.clone();
  let got = transaction.valid_is();
  assert_eq!( got, true );

  println!( "transaction is not valid, changed data in header" );
  let mut transaction = transaction_original.clone();
  transaction.time = 101;
  let got = transaction.valid_is();
  assert_eq!( got, false );

  println!( "transaction is not valid, changed hash in body" );
  let mut transaction = transaction_original.clone();
  transaction.body.hash = digest::Digest::from( Vec::from([ 0u8; 32 ]) );
  let got = transaction.valid_is();
  assert_eq!( got, false );

  println!( "transaction is valid, changed data in header and recalculated hash" );
  let mut transaction = transaction_original.clone();
  transaction.time = 10000;
  transaction.body.hash = digest::hash_single( transaction.header() );
  let got = transaction.valid_is();
  assert_eq!( got, true );
}

