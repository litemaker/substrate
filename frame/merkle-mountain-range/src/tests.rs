// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

use crate::{mock::*, *};

use frame_support::traits::OnInitialize;
use pallet_mmr_primitives::{Compact, Proof};
use sp_core::{
	offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt},
	H256,
};

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

fn register_offchain_ext(ext: &mut sp_io::TestExternalities) {
	let (offchain, _offchain_state) = TestOffchainExt::with_offchain_db(ext.offchain_db());
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));
}

fn new_block() -> u64 {
	let number = frame_system::Pallet::<Test>::block_number() + 1;
	let hash = H256::repeat_byte(number as u8);
	LEAF_DATA.with(|r| r.borrow_mut().a = number);

	frame_system::Pallet::<Test>::initialize(
		&number,
		&hash,
		&Default::default(),
		frame_system::InitKind::Full,
	);
	MMR::on_initialize(number)
}

pub(crate) fn hex(s: &str) -> H256 {
	s.parse().unwrap()
}

type BlockNumber = <Test as frame_system::Config>::BlockNumber;

fn decode_node(
	v: Vec<u8>,
) -> mmr::Node<<Test as Config>::Hashing, ((BlockNumber, H256), LeafData)> {
	use crate::primitives::DataOrHash;
	type A = DataOrHash<<Test as Config>::Hashing, (BlockNumber, H256)>;
	type B = DataOrHash<<Test as Config>::Hashing, LeafData>;
	type Node = mmr::Node<<Test as Config>::Hashing, (A, B)>;
	let tuple: Node = codec::Decode::decode(&mut &v[..]).unwrap();

	match tuple {
		mmr::Node::Data((DataOrHash::Data(a), DataOrHash::Data(b))) => mmr::Node::Data((a, b)),
		mmr::Node::Hash(hash) => mmr::Node::Hash(hash),
		_ => unreachable!(),
	}
}

fn init_chain(blocks: usize) {
	// given
	for _ in 0..blocks {
		new_block();
	}
}

#[test]
fn should_start_empty() {
	let _ = env_logger::try_init();
	new_test_ext().execute_with(|| {
		// given
		assert_eq!(
			crate::RootHash::<Test>::get(),
			"0000000000000000000000000000000000000000000000000000000000000000"
				.parse()
				.unwrap()
		);
		assert_eq!(crate::NumberOfLeaves::<Test>::get(), 0);
		assert_eq!(crate::Nodes::<Test>::get(0), None);

		// when
		let weight = new_block();

		// then
		assert_eq!(crate::NumberOfLeaves::<Test>::get(), 1);
		assert_eq!(
			crate::Nodes::<Test>::get(0),
			Some(hex("4320435e8c3318562dba60116bdbcc0b82ffcecb9bb39aae3300cfda3ad0b8b0"))
		);
		assert_eq!(
			crate::RootHash::<Test>::get(),
			hex("4320435e8c3318562dba60116bdbcc0b82ffcecb9bb39aae3300cfda3ad0b8b0")
		);
		assert!(weight != 0);
	});
}

#[test]
fn should_append_to_mmr_when_on_initialize_is_called() {
	let _ = env_logger::try_init();
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// when
		new_block();
		new_block();

		// then
		assert_eq!(crate::NumberOfLeaves::<Test>::get(), 2);
		assert_eq!(
			(
				crate::Nodes::<Test>::get(0),
				crate::Nodes::<Test>::get(1),
				crate::Nodes::<Test>::get(2),
				crate::Nodes::<Test>::get(3),
				crate::RootHash::<Test>::get(),
			),
			(
				Some(hex("4320435e8c3318562dba60116bdbcc0b82ffcecb9bb39aae3300cfda3ad0b8b0")),
				Some(hex("ad4cbc033833612ccd4626d5f023b9dfc50a35e838514dd1f3c86f8506728705")),
				Some(hex("672c04a9cd05a644789d769daa552d35d8de7c33129f8a7cbf49e595234c4854")),
				None,
				hex("672c04a9cd05a644789d769daa552d35d8de7c33129f8a7cbf49e595234c4854"),
			)
		);
	});

	// make sure the leaves end up in the offchain DB
	ext.persist_offchain_overlay();
	let offchain_db = ext.offchain_db();
	assert_eq!(
		offchain_db.get(&MMR::offchain_key(0)).map(decode_node),
		Some(mmr::Node::Data(((0, H256::repeat_byte(1)), LeafData::new(1),)))
	);
	assert_eq!(
		offchain_db.get(&MMR::offchain_key(1)).map(decode_node),
		Some(mmr::Node::Data(((1, H256::repeat_byte(2)), LeafData::new(2),)))
	);
	assert_eq!(
		offchain_db.get(&MMR::offchain_key(2)).map(decode_node),
		Some(mmr::Node::Hash(hex(
			"672c04a9cd05a644789d769daa552d35d8de7c33129f8a7cbf49e595234c4854"
		)))
	);
	assert_eq!(offchain_db.get(&MMR::offchain_key(3)), None);
}

#[test]
fn should_construct_larger_mmr_correctly() {
	let _ = env_logger::try_init();
	new_test_ext().execute_with(|| {
		// when
		init_chain(7);

		// then
		assert_eq!(crate::NumberOfLeaves::<Test>::get(), 7);
		assert_eq!(
			(
				crate::Nodes::<Test>::get(0),
				crate::Nodes::<Test>::get(10),
				crate::RootHash::<Test>::get(),
			),
			(
				Some(hex("4320435e8c3318562dba60116bdbcc0b82ffcecb9bb39aae3300cfda3ad0b8b0")),
				Some(hex("611c2174c6164952a66d985cfe1ec1a623794393e3acff96b136d198f37a648c")),
				hex("e45e25259f7930626431347fa4dd9aae7ac83b4966126d425ca70ab343709d2c"),
			)
		);
	});
}

#[test]
fn should_generate_proofs_correctly() {
	let _ = env_logger::try_init();
	let mut ext = new_test_ext();
	// given
	ext.execute_with(|| init_chain(7));
	ext.persist_offchain_overlay();

	// Try to generate proofs now. This requires the offchain extensions to be present
	// to retrieve full leaf data.
	register_offchain_ext(&mut ext);
	ext.execute_with(|| {
		// when generate proofs for all leaves
		let proofs = (0_u64..crate::NumberOfLeaves::<Test>::get())
			.into_iter()
			.map(|leaf_index| crate::Pallet::<Test>::generate_proof(leaf_index).unwrap())
			.collect::<Vec<_>>();

		// then
		assert_eq!(
			proofs[0],
			(
				Compact::new(((0, H256::repeat_byte(1)).into(), LeafData::new(1).into(),)),
				Proof {
					leaf_index: 0,
					leaf_count: 7,
					items: vec![
						hex("ad4cbc033833612ccd4626d5f023b9dfc50a35e838514dd1f3c86f8506728705"),
						hex("cb24f4614ad5b2a5430344c99545b421d9af83c46fd632d70a332200884b4d46"),
						hex("dca421199bdcc55bb773c6b6967e8d16675de69062b52285ca63685241fdf626"),
					],
				}
			)
		);
		assert_eq!(
			proofs[4],
			(
				Compact::new(((4, H256::repeat_byte(5)).into(), LeafData::new(5).into(),)),
				Proof {
					leaf_index: 4,
					leaf_count: 7,
					items: vec![
						hex("ae88a0825da50e953e7a359c55fe13c8015e48d03d301b8bdfc9193874da9252"),
						hex("8ed25570209d8f753d02df07c1884ddb36a3d9d4770e4608b188322151c657fe"),
						hex("611c2174c6164952a66d985cfe1ec1a623794393e3acff96b136d198f37a648c"),
					],
				}
			)
		);
		assert_eq!(
			proofs[6],
			(
				Compact::new(((6, H256::repeat_byte(7)).into(), LeafData::new(7).into(),)),
				Proof {
					leaf_index: 6,
					leaf_count: 7,
					items: vec![
						hex("ae88a0825da50e953e7a359c55fe13c8015e48d03d301b8bdfc9193874da9252"),
						hex("7e4316ae2ebf7c3b6821cb3a46ca8b7a4f9351a9b40fcf014bb0a4fd8e8f29da"),
					],
				}
			)
		);
	});
}

#[test]
fn should_verify() {
	let _ = env_logger::try_init();

	// Start off with chain initialisation and storing indexing data off-chain
	// (MMR Leafs)
	let mut ext = new_test_ext();
	ext.execute_with(|| init_chain(7));
	ext.persist_offchain_overlay();

	// Try to generate proof now. This requires the offchain extensions to be present
	// to retrieve full leaf data.
	register_offchain_ext(&mut ext);
	let (leaf, proof5) = ext.execute_with(|| {
		// when
		crate::Pallet::<Test>::generate_proof(5).unwrap()
	});

	// Now to verify the proof, we really shouldn't require offchain storage or extension.
	// Hence we initialize the storage once again, using different externalities and then
	// verify.
	let mut ext2 = new_test_ext();
	ext2.execute_with(|| {
		init_chain(7);
		// then
		assert_eq!(crate::Pallet::<Test>::verify_leaf(leaf, proof5), Ok(()));
	});
}

#[test]
fn verification_should_be_stateless() {
	let _ = env_logger::try_init();

	// Start off with chain initialisation and storing indexing data off-chain
	// (MMR Leafs)
	let mut ext = new_test_ext();
	ext.execute_with(|| init_chain(7));
	ext.persist_offchain_overlay();

	// Try to generate proof now. This requires the offchain extensions to be present
	// to retrieve full leaf data.
	register_offchain_ext(&mut ext);
	let (leaf, proof5) = ext.execute_with(|| {
		// when
		crate::Pallet::<Test>::generate_proof(5).unwrap()
	});
	let root = ext.execute_with(|| crate::Pallet::<Test>::mmr_root_hash());

	// Verify proof without relying on any on-chain data.
	let leaf = crate::primitives::DataOrHash::Data(leaf);
	assert_eq!(
		crate::verify_leaf_proof::<<Test as Config>::Hashing, _>(root, leaf, proof5),
		Ok(())
	);
}

#[test]
fn verify_real_beefy_mmr_proof() {
	/*
		test this:

		mmrLeafOpaqueEncoded=0xc5010063000000d2657f7e0327d4a715d7848b8083de2c37e50f8c3797709f9da8a963b4de182a010000000000000002000000697ea2a8fe5b03468548a7a413424a6292ab44a82a6f5cc594c3fa7dda7ce402a35e7eedcf3097c9ca41785be850e6c67d94ac6379b9acc1449d0784a47011f5

		hashedOpaqueLeaf=0xbb3726f1cd600a1e1aa273990ce3adfd326298c1c6bc4ba5059c35fb32116ea2

		hashedLeaf=0x065191546a9ea776f9970c4007675b2f2a0543a067535981a92fe5ea749e8572

		_beefyMMRLeafIndex: 99

		_beefyLeafCount: 105

		beefyMMRProof :[\"0x5547c8f3d63ba09401a8830aa6adefbc6ac5598687108e74729e01ab228a59be\",\"0x1735814e29795e86a7daa647f7d3bbe922cd71d6f7b67accfd529b1a12a24c9e\",\"0x2e745fa293eb5136a89bd64df57ec66b41ee7b9bdc83accfec7243a58265f8c3\",\"0xaa2d1872fb2ca86cd6450b9c335ced9aadded7e323c38b3a76b4ade946590b2c\",\"0x92a73b500b479595da060e4543f83c2becbcf7f685f449299a60162f54a8ac5a\",\"0x173d96b6a2a46e255cc793f1c346f98faf2fd036d8e959ba5bb454f64eedc19b\"]

		mmrRootHash=0xb0e22d5808dfcbf277c71904b199a7d93c710c15e86b5f5882f8b11b8fe02858
	*/

	// Verify proof without relying on any on-chain data.
	let leaf = hex("065191546a9ea776f9970c4007675b2f2a0543a067535981a92fe5ea749e8572");
	let leaf = crate::primitives::DataOrHash::Hash::<<Test as Config>::Hashing, H256>(leaf);
	let proof = Proof {
		leaf_index: 99,
		leaf_count: 105,
		items: vec![
			hex("5547c8f3d63ba09401a8830aa6adefbc6ac5598687108e74729e01ab228a59be"),
			hex("1735814e29795e86a7daa647f7d3bbe922cd71d6f7b67accfd529b1a12a24c9e"),
			hex("2e745fa293eb5136a89bd64df57ec66b41ee7b9bdc83accfec7243a58265f8c3"),
			hex("aa2d1872fb2ca86cd6450b9c335ced9aadded7e323c38b3a76b4ade946590b2c"),
			hex("92a73b500b479595da060e4543f83c2becbcf7f685f449299a60162f54a8ac5a"),
			hex("173d96b6a2a46e255cc793f1c346f98faf2fd036d8e959ba5bb454f64eedc19b"),
		],
	};
	let root = hex("b0e22d5808dfcbf277c71904b199a7d93c710c15e86b5f5882f8b11b8fe02858");
	assert_eq!(crate::verify_leaf_proof::<<Test as Config>::Hashing, _>(root, leaf, proof), Ok(()));
}

#[test]
fn should_verify_on_the_next_block_since_there_is_no_pruning_yet() {
	let _ = env_logger::try_init();
	let mut ext = new_test_ext();
	// given
	ext.execute_with(|| init_chain(7));

	ext.persist_offchain_overlay();
	register_offchain_ext(&mut ext);

	ext.execute_with(|| {
		// when
		let (leaf, proof5) = crate::Pallet::<Test>::generate_proof(5).unwrap();
		new_block();

		// then
		assert_eq!(crate::Pallet::<Test>::verify_leaf(leaf, proof5), Ok(()));
	});
}
