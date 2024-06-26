// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-anb7yjbi-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/bounties/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_bounties`.
pub trait WeightInfo {
	fn propose_bounty(d: u32, ) -> Weight;
	fn approve_bounty() -> Weight;
	fn propose_curator() -> Weight;
	fn unassign_curator() -> Weight;
	fn accept_curator() -> Weight;
	fn award_bounty() -> Weight;
	fn claim_bounty() -> Weight;
	fn close_bounty_proposed() -> Weight;
	fn close_bounty_active() -> Weight;
	fn extend_bounty_expiry() -> Weight;
	fn spend_funds(b: u32, ) -> Weight;
}

/// Weights for `pallet_bounties` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Bounties::BountyCount` (r:1 w:1)
	/// Proof: `Bounties::BountyCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:0 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 300]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `3593`
		// Minimum execution time: 25_206_000 picoseconds.
		Weight::from_parts(26_925_800, 3593)
			// Standard Error: 239
			.saturating_add(Weight::from_parts(501, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyApprovals` (r:1 w:1)
	/// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn approve_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `3642`
		// Minimum execution time: 13_150_000 picoseconds.
		Weight::from_parts(13_708_000, 3642)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn propose_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `3642`
		// Minimum execution time: 12_277_000 picoseconds.
		Weight::from_parts(12_769_000, 3642)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn unassign_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `597`
		//  Estimated: `3642`
		// Minimum execution time: 29_041_000 picoseconds.
		Weight::from_parts(29_979_000, 3642)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn accept_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `593`
		//  Estimated: `3642`
		// Minimum execution time: 27_936_000 picoseconds.
		Weight::from_parts(28_925_000, 3642)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn award_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `605`
		//  Estimated: `3642`
		// Minimum execution time: 16_759_000 picoseconds.
		Weight::from_parts(17_699_000, 3642)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
	/// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn claim_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `8799`
		// Minimum execution time: 112_056_000 picoseconds.
		Weight::from_parts(114_275_000, 8799)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn close_bounty_proposed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649`
		//  Estimated: `3642`
		// Minimum execution time: 32_625_000 picoseconds.
		Weight::from_parts(33_719_000, 3642)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn close_bounty_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `885`
		//  Estimated: `6196`
		// Minimum execution time: 76_895_000 picoseconds.
		Weight::from_parts(79_161_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn extend_bounty_expiry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `457`
		//  Estimated: `3642`
		// Minimum execution time: 12_635_000 picoseconds.
		Weight::from_parts(13_423_000, 3642)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::BountyApprovals` (r:1 w:1)
	/// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:100 w:100)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:200 w:200)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37 + b * (297 ±0)`
		//  Estimated: `1887 + b * (5206 ±0)`
		// Minimum execution time: 2_840_000 picoseconds.
		Weight::from_parts(6_076_743, 1887)
			// Standard Error: 18_569
			.saturating_add(Weight::from_parts(34_771_846, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5206).saturating_mul(b.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Bounties::BountyCount` (r:1 w:1)
	/// Proof: `Bounties::BountyCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:0 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 300]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `3593`
		// Minimum execution time: 25_206_000 picoseconds.
		Weight::from_parts(26_925_800, 3593)
			// Standard Error: 239
			.saturating_add(Weight::from_parts(501, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyApprovals` (r:1 w:1)
	/// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn approve_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `3642`
		// Minimum execution time: 13_150_000 picoseconds.
		Weight::from_parts(13_708_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn propose_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `3642`
		// Minimum execution time: 12_277_000 picoseconds.
		Weight::from_parts(12_769_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn unassign_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `597`
		//  Estimated: `3642`
		// Minimum execution time: 29_041_000 picoseconds.
		Weight::from_parts(29_979_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn accept_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `593`
		//  Estimated: `3642`
		// Minimum execution time: 27_936_000 picoseconds.
		Weight::from_parts(28_925_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn award_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `605`
		//  Estimated: `3642`
		// Minimum execution time: 16_759_000 picoseconds.
		Weight::from_parts(17_699_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
	/// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn claim_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `8799`
		// Minimum execution time: 112_056_000 picoseconds.
		Weight::from_parts(114_275_000, 8799)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn close_bounty_proposed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649`
		//  Estimated: `3642`
		// Minimum execution time: 32_625_000 picoseconds.
		Weight::from_parts(33_719_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
	/// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(314), added: 2789, mode: `MaxEncodedLen`)
	fn close_bounty_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `885`
		//  Estimated: `6196`
		// Minimum execution time: 76_895_000 picoseconds.
		Weight::from_parts(79_161_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:1)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn extend_bounty_expiry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `457`
		//  Estimated: `3642`
		// Minimum execution time: 12_635_000 picoseconds.
		Weight::from_parts(13_423_000, 3642)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Bounties::BountyApprovals` (r:1 w:1)
	/// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:100 w:100)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:200 w:200)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37 + b * (297 ±0)`
		//  Estimated: `1887 + b * (5206 ±0)`
		// Minimum execution time: 2_840_000 picoseconds.
		Weight::from_parts(6_076_743, 1887)
			// Standard Error: 18_569
			.saturating_add(Weight::from_parts(34_771_846, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5206).saturating_mul(b.into()))
	}
}
