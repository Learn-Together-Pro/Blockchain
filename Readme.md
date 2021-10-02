# Blockchain  [![status](https://github.com/Learn-Together-Pro/Blockchain/actions/workflows/Main.yml/badge.svg)](https://github.com/Learn-Together-Pro/Blockchain/actions/workflows/Main.yml) [![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

Blockchain written with educational purpose

### Prerequisites

Before running the build please make sure standart development environment is configured.

- [Git](https://git-scm.com/)
- Stable version of [Rust](https://www.rust-lang.org/tools/install)

**Linux**

You may need to install `build-essential`:

```
sudo apt update
sudo apt install build-essential
```

**MacOS**

Make sure you've installed `Xcode` and its developer tools:

```
xcode-select --install
```

**Windows**

You may need to install the [ Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### How to start

```
git clone https://github.com/Learn-Together-Pro/Blockchain.git
cd Blockchain
cargo run
```

### Commands

To rebuild:
```
cargo build
```

To run the probject:
```
cargo run
```

To test your changes:
```
cargo test
```

### Usage

To see commands list execute:
```
cargo run
```

Output:

```
.quit => Quit
.system.wipe => Clear data from blockchain
.system.log => Show system state
.system.difficulty => Difficulty set
.system.reward => Set reward
.transaction.create => Create transaction
.block.mine => Mine a block
.blocks.log => Show sys state
.wallet.create => Create waller
.wallet.log => Print information about a wallet
.wallets.log => Print information about all wallets

Please select :
```

To run the command type its name starting with **dot** char and press `Enter` key, for example:
```
Please select : .system.log
```

Some commands require additional information that will be asked:

```
Please select : .transaction.create
Sender :
```

### Tickets

- Implement routine `balance_get()`
-- should return for adresses which do not exist