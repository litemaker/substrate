// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/elections-phragmen/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections_phragmen.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn remove_member_wrong_refund() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections_phragmen using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn vote_equal(v: u32, ) -> Weight {
		(43_911_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((324_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vote_more(v: u32, ) -> Weight {
		(68_236_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((359_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vote_less(v: u32, ) -> Weight {
		(68_162_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((350_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_voter() -> Weight {
		(63_005_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn submit_candidacy(c: u32, ) -> Weight {
		(58_498_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((305_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(52_062_000 as Weight)
			// Standard Error: 0
			.saturating_add((173_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_members() -> Weight {
		(73_234_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn renounce_candidacy_runners_up() -> Weight {
		(51_689_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_member_with_replacement() -> Weight {
		(79_906_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn remove_member_wrong_refund() -> Weight {
		(6_877_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 39_000
			.saturating_add((112_381_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_789_000
			.saturating_add((42_600_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 744_000
			.saturating_add((60_743_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 50_000
			.saturating_add((3_837_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn vote_equal(v: u32, ) -> Weight {
		(43_911_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((324_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn vote_more(v: u32, ) -> Weight {
		(68_236_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((359_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn vote_less(v: u32, ) -> Weight {
		(68_162_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((350_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn remove_voter() -> Weight {
		(63_005_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn submit_candidacy(c: u32, ) -> Weight {
		(58_498_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((305_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(52_062_000 as Weight)
			// Standard Error: 0
			.saturating_add((173_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_members() -> Weight {
		(73_234_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn renounce_candidacy_runners_up() -> Weight {
		(51_689_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_member_with_replacement() -> Weight {
		(79_906_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn remove_member_wrong_refund() -> Weight {
		(6_877_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 39_000
			.saturating_add((112_381_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_789_000
			.saturating_add((42_600_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 744_000
			.saturating_add((60_743_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 50_000
			.saturating_add((3_837_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}
