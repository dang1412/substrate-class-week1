# Debug, Mock, Test và Benchmark

## Debug

```rs
impl<T: Config> sp_std::fmt::Display for Kitty<T> {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "(dna: {:?}, price: {:?}, gender: {:?}, owner: {:?})", self.dna, self.price, self.gender, self.owner)
    }
}

// ...
log::info!("{}", kitty);
```

## Mock

Cài đặt mock cho pallet kitties

`lib.rs`

```rs
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
```

Copy các thành phần sử dụng trong `runtime/Cargo.toml [dependencies]` đưa vào `pallets/kitties/Cargo.toml [dev-dependencies]`, bỏ `default-features = false`

`pallets/kitties/Cargo.toml`

```toml
[dev-dependencies]
sp-core = { default-features = false, version = "5.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.17" }
pallet-balances = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.17" }
pallet-randomness-collective-flip = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.17" }
pallet-timestamp = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.17" }
```

Edit file mock.rs

```rs
use crate as pallet_kitties;
use frame_support::traits::{ConstU16, ConstU64, ConstU32, ConstU128};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::parameter_types;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        KittiesModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>, Config<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet},
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = u128;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
    type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

parameter_types!{
    pub const MaxKittyOwned: u32 = 10;
}

impl pallet_kitties::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type MaxKittyOwned = MaxKittyOwned;
    type KittyRandomness = RandomnessCollectiveFlip;
    type TimeProvider = Timestamp;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
```

## Benchmark

Add benchmark của pallet-template vào runtime

[docs/add-benchmarking](https://docs.substrate.io/how-to-guides/v3/weights/add-benchmarking/)

<!-- `runtime/lib.rs`

```rs
add_benchmarks!(params, batches, pallet_template, TemplateModule);  // line 543
``` -->

Chạy lệnh

```sh
./target/release/node-template benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_template --extrinsic '*' --steps 20 --repeat 10 --json-file=raw.json --output ./pallets/template/src/weights.rs
```

- Cách 1: copy giá trị sinh ra trong file `weights.rs` vào phần khai báo weight extrinsic `do_something`.

- Cách 2: import giá trị trong module file `weights.rs` vào phần khai báo weight extrinsic `do_something`.

### Import giá trị trong module `weights.rs`

Sửa file `weight.rs` để loosely coupling import giá trị trong `lib.rs`

```rs
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

// Tạo trait để dùng loosely coupling
pub trait WeightInfo {
    fn do_something(_s: u32) -> Weight;
}

/// Weight functions for `pallet_template`.
pub struct SubstrateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeightInfo<T> {
    // Storage: TemplateModule Something (r:0 w:1)
    fn do_something(_s: u32, ) -> Weight {
        (30_807_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// implement dùng trong mock
impl WeightInfo for () {
    fn do_something() -> Weight {
        (30_807_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
```

include `sp-std` vào trong `template/Cargo.toml` để dùng trong file `weight.rs`

```toml
[dependencies]
sp-std = { default-features = false, version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.17" }

[features]
default = ["std"]
std = [
    "sp-std/std",
]
```

Loosely coupling trong `lib.rs`

```rs
// ...
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    // ...

    #[pallet::config]
    pub trait Config: frame_system::Config {
        // ...
        type WeightInfo: WeightInfo;
    }

    // ...

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::weight(T::WeightInfo::do_something())]
        // ...
    }
}
```

Sửa implement trong `mock.rs`

```rs
impl pallet_template::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}
```

Sửa implement trong `runtime/src/lib.rs`

```rs
impl pallet_template::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_template::weights::SubstrateWeightInfo<Runtime>;
}
```
