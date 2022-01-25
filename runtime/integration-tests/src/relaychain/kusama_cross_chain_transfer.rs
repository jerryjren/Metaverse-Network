// This file is part of Bit.Country.

// Copyright (C) 2020-2021 Bit.Country.
// SPDX-License-Identifier: Apache-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Cross-chain transfer tests within Kusama network.

use frame_support::assert_ok;
use orml_traits::MultiCurrency;
use xcm::v0::OriginKind::Native;
use xcm_emulator::TestExt;

// use module_asset_registry::AssetMetadata;
use module_relaychain::RelayChainCallBuilder;
use module_support::CallBuilder;
use pioneer_runtime::TreasuryModuleAccount;

use crate::relaychain::kusama_test_net::*;
use crate::setup::*;

#[test]
fn transfer_from_relay_chain() {
	KusamaNet::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(2000).into().into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			Box::new((Here, dollar(KSM)).into()),
			0
		));
	});

	Pioneer::execute_with(|| {
		assert_eq!(
			Currencies::free_balance(RELAY_CHAIN_CURRENCY_ID, &AccountId::from(BOB)),
			999_872_000_000
		);
	});
}

// #[test]
// fn transfer_to_relay_chain() {
// 	Pioneer::execute_with(|| {
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(ALICE.into()),
// 			KSM,
// 			dollar(KSM),
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X1(Junction::AccountId32 {
// 						id: BOB,
// 						network: NetworkId::Any,
// 					})
// 				)
// 				.into()
// 			),
// 			4_000_000_000
// 		));
// 	});
//
// 	KusamaNet::execute_with(|| {
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
// 			999_893_333_340
// 		);
// 	});
// }
//
// #[test]
// fn transfer_to_sibling() {
// 	TestNet::reset();
//
// 	fn neer_reserve_account() -> AccountId {
// 		use sp_runtime::traits::AccountIdConversion;
// 		polkadot_parachain::primitives::Sibling::from(2000).into_account()
// 	}
//
// 	Pioneer::execute_with(|| {
// 		assert_ok!(Tokens::deposit(BNC, &AccountId::from(ALICE), 100_000_000_000_000));
// 	});
//
// 	Sibling::execute_with(|| {
// 		assert_ok!(Tokens::deposit(BNC, &neer_reserve_account(), 100_000_000_000_000));
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(ALICE.into()),
// 			BNC,
// 			10_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2001),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: BOB.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));
//
// 		assert_eq!(Currencies::free_balance(BNC, &AccountId::from(ALICE)), 90_000_000_000_000);
// 	});
//
// 	Sibling::execute_with(|| {
// 		assert_eq!(Currencies::free_balance(BNC, &neer_reserve_account()), 90_000_000_000_000);
// 		assert_eq!(Currencies::free_balance(BNC, &AccountId::from(BOB)), 9_989_760_000_000);
//
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			BNC,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));
//
// 		assert_eq!(Currencies::free_balance(BNC, &neer_reserve_account()), 95_000_000_000_000);
// 		assert_eq!(Currencies::free_balance(BNC, &AccountId::from(BOB)), 4_989_760_000_000);
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(Currencies::free_balance(BNC, &AccountId::from(ALICE)), 94_989_760_000_000);
// 	});
// }
//
// #[test]
// fn transfer_from_relay_chain_deposit_to_treasury_if_below_ed() {
// 	KusamaNet::execute_with(|| {
// 		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			Box::new(Parachain(2000).into().into()),
// 			Box::new(
// 				Junction::AccountId32 {
// 					id: BOB,
// 					network: NetworkId::Any
// 				}
// 				.into()
// 				.into()
// 			),
// 			Box::new((Here, 128_000_111).into()),
// 			0
// 		));
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(Currencies::free_balance(KSM, &AccountId::from(BOB)), 0);
// 		assert_eq!(
// 			Currencies::free_balance(KSM, &pioneer_runtime::TreasuryModuleAccount::get()),
// 			1_000_128_000_111
// 		);
// 	});
// }
//
// #[test]
// fn xcm_transfer_execution_barrier_trader_works() {
// 	let expect_weight_limit = 600_000_000;
// 	let weight_limit_too_low = 500_000_000;
// 	let unit_instruction_weight = 200_000_000;
//
// 	// relay-chain use normal account to send xcm, destination para-chain can't pass Barrier check
// 	let message = Xcm(vec![
// 		ReserveAssetDeposited((Parent, 100).into()),
// 		BuyExecution {
// 			fees: (Parent, 100).into(),
// 			weight_limit: Unlimited,
// 		},
// 		DepositAsset {
// 			assets: All.into(),
// 			max_assets: 1,
// 			beneficiary: Here.into(),
// 		},
// 	]);
// 	KusamaNet::execute_with(|| {
// 		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::send(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			Box::new(Parachain(2000).into().into()),
// 			Box::new(xcm::VersionedXcm::from(message)),
// 		);
// 		assert_ok!(r);
// 	});
// 	Pioneer::execute_with(|| {
// 		assert!(System::events().iter().any(|r| matches!(
// 			r.event,
// 			Event::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward(
// 				_,
// 				Outcome::Error(XcmError::Barrier)
// 			))
// 		)));
// 	});
//
// 	// AllowTopLevelPaidExecutionFrom barrier test case:
// 	// para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
// 	// if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't
// pass. 	// other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then it's
// ok. 	let message = Xcm::<pioneer_runtime::Call>(vec![
// 		ReserveAssetDeposited((Parent, 100).into()),
// 		BuyExecution {
// 			fees: (Parent, 100).into(),
// 			weight_limit: Limited(weight_limit_too_low),
// 		},
// 		DepositAsset {
// 			assets: All.into(),
// 			max_assets: 1,
// 			beneficiary: Here.into(),
// 		},
// 	]);
// 	Pioneer::execute_with(|| {
// 		let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
// 		assert_eq!(r, Outcome::Error(XcmError::Barrier));
// 	});
//
// 	// trader inside BuyExecution have TooExpensive error if payment less than calculated weight
// amount. 	// the minimum of calculated weight amount(`FixedRateOfFungible<KsmPerSecond>`) is
// 96_000_000 	let message = Xcm::<pioneer_runtime::Call>(vec![
// 		ReserveAssetDeposited((Parent, 95_999_999).into()),
// 		BuyExecution {
// 			fees: (Parent, 95_999_999).into(),
// 			weight_limit: Limited(expect_weight_limit),
// 		},
// 		DepositAsset {
// 			assets: All.into(),
// 			max_assets: 1,
// 			beneficiary: Here.into(),
// 		},
// 	]);
// 	Pioneer::execute_with(|| {
// 		let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
// 		assert_eq!(
// 			r,
// 			Outcome::Incomplete(expect_weight_limit - unit_instruction_weight, XcmError::TooExpensive)
// 		);
// 	});
//
// 	// all situation fulfilled, execute success
// 	let message = Xcm::<pioneer_runtime::Call>(vec![
// 		ReserveAssetDeposited((Parent, 96_000_000).into()),
// 		BuyExecution {
// 			fees: (Parent, 96_000_000).into(),
// 			weight_limit: Limited(expect_weight_limit),
// 		},
// 		DepositAsset {
// 			assets: All.into(),
// 			max_assets: 1,
// 			beneficiary: Here.into(),
// 		},
// 	]);
// 	Pioneer::execute_with(|| {
// 		let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
// 		assert_eq!(r, Outcome::Complete(expect_weight_limit));
// 	});
// }
//
// #[test]
// fn subscribe_version_notify_works() {
// 	// relay chain subscribe version notify of para chain
// 	KusamaNet::execute_with(|| {
// 		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::force_subscribe_version_notify(
// 			kusama_runtime::Origin::root(),
// 			Box::new(Parachain(2000).into().into()),
// 		);
// 		assert_ok!(r);
// 	});
// 	KusamaNet::execute_with(|| {
// 		kusama_runtime::System::assert_has_event(kusama_runtime::Event::XcmPallet(
// 			pallet_xcm::Event::SupportedVersionChanged(
// 				MultiLocation {
// 					parents: 0,
// 					interior: X1(Parachain(2000)),
// 				},
// 				2,
// 			),
// 		));
// 	});
//
// 	// para chain subscribe version notify of relay chain
// 	Pioneer::execute_with(|| {
// 		let r = pallet_xcm::Pallet::<pioneer_runtime::Runtime>::force_subscribe_version_notify(
// 			Origin::root(),
// 			Box::new(Parent.into()),
// 		);
// 		assert_ok!(r);
// 	});
// 	Pioneer::execute_with(|| {
// 		System::assert_has_event(pioneer_runtime::Event::PolkadotXcm(
// 			pallet_xcm::Event::SupportedVersionChanged(
// 				MultiLocation {
// 					parents: 1,
// 					interior: Here,
// 				},
// 				2,
// 			),
// 		));
// 	});
//
// 	// para chain subscribe version notify of sibling chain
// 	Pioneer::execute_with(|| {
// 		let r = pallet_xcm::Pallet::<pioneer_runtime::Runtime>::force_subscribe_version_notify(
// 			Origin::root(),
// 			Box::new((Parent, Parachain(2001)).into()),
// 		);
// 		assert_ok!(r);
// 	});
// 	Pioneer::execute_with(|| {
// 		assert!(pioneer_runtime::System::events().iter().any(|r| matches!(
// 			r.event,
// 			pioneer_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(Some(_)))
// 		)));
// 	});
// 	Sibling::execute_with(|| {
// 		assert!(System::events().iter().any(|r| matches!(
// 			r.event,
// 			pioneer_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(Some(_)))
// 				| pioneer_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success(Some(_)))
// 		)));
// 	});
// }
//
// #[test]
// fn test_asset_registry_module() {
// 	TestNet::reset();
//
// 	fn pioneer_reserve_account() -> AccountId {
// 		use sp_runtime::traits::AccountIdConversion;
// 		polkadot_parachain::primitives::Sibling::from(2000).into_account()
// 	}
//
// 	Pioneer::execute_with(|| {
// 		// register foreign asset
// 		assert_ok!(AssetRegistry::register_foreign_asset(
// 			Origin::root(),
// 			Box::new(MultiLocation::new(1, X2(Parachain(2001), GeneralKey(KAR.encode()))).into()),
// 			Box::new(AssetMetadata {
// 				name: b"Sibling Token".to_vec(),
// 				symbol: b"ST".to_vec(),
// 				decimals: 12,
// 				minimal_balance: Balances::minimum_balance() / 10, // 10%
// 			})
// 		));
//
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &TreasuryAccount::get()),
// 			0
// 		);
// 	});
//
// 	Sibling::execute_with(|| {
// 		let _ = Balances::deposit_creating(&AccountId::from(BOB), 100_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&pioneer_reserve_account()), 0);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 100_000_000_000_000);
//
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			KAR,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));
//
// 		assert_eq!(Balances::free_balance(&pioneer_reserve_account()), 5_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_000_000_000_000);
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &AccountId::from(ALICE)),
// 			4_999_360_000_000
// 		);
// 		// ToTreasury
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &TreasuryAccount::get()),
// 			640_000_000
// 		);
//
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(ALICE.into()),
// 			FungibleTokenId::NativeToken(0),
// 			1_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2001),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: BOB.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));
//
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &AccountId::from(ALICE)),
// 			3_999_360_000_000
// 		);
// 	});
//
// 	Sibling::execute_with(|| {
// 		assert_eq!(Balances::free_balance(&pioneer_reserve_account()), 4_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_993_600_000_000);
// 	});
//
// 	// remove it
// 	Pioneer::execute_with(|| {
// 		// register foreign asset
// 		assert_ok!(AssetRegistry::update_foreign_asset(
// 			Origin::root(),
// 			0,
// 			Box::new(MultiLocation::new(1, X2(Parachain(2001), GeneralKey(KAR.encode()))).into()),
// 			Box::new(AssetMetadata {
// 				name: b"Sibling Token".to_vec(),
// 				symbol: b"ST".to_vec(),
// 				decimals: 12,
// 				minimal_balance: 0, // buy_weight 0
// 			})
// 		));
// 	});
//
// 	Sibling::execute_with(|| {
// 		assert_eq!(Balances::free_balance(&pioneer_reserve_account()), 4_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_993_600_000_000);
//
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			KAR,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));
//
// 		assert_eq!(Balances::free_balance(&pioneer_reserve_account()), 9_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 90_993_600_000_000);
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &AccountId::from(ALICE)),
// 			8_999_360_000_000
// 		);
//
// 		// ToTreasury
// 		assert_eq!(
// 			Currencies::free_balance(FungibleTokenId::NativeToken(0), &TreasuryAccount::get()),
// 			640_000_000
// 		);
// 	});
// }
//
// #[test]
// fn unspent_xcm_fee_is_returned_correctly() {
// 	let mut parachain_account: AccountId = AccountId::default();
// 	let sub_account: AccountId =
// 		hex_literal::hex!["d7b8926b326dd349355a9a7cca6606c1e0eb6fd2b506066b518c7155ff0d8297"].into();
// 	Pioneer::execute_with(|| {
// 		parachain_account = ParachainAccount::get();
// 	});
// 	KusamaNet::execute_with(|| {
// 		assert_ok!(kusama_runtime::Balances::transfer(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			MultiAddress::Id(sub_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		));
// 		assert_ok!(kusama_runtime::Balances::transfer(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			MultiAddress::Id(parachain_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		));
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(ALICE)),
// 			2 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&sub_account),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		assert_eq!(kusama_runtime::Balances::free_balance(&AccountId::from(BOB)), 0);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_002 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 	});
//
// 	Pioneer::execute_with(|| {
// 		// Construct a transfer XCM call with returning the deposit
// 		let transfer_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::balances_transfer_keep_alive( 			AccountId::from(BOB),
// 			dollar(NATIVE_CURRENCY_ID),
// 		);
// 		let batch_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::utility_as_derivative_call(transfer_call, 0); 		let weight = 10_000_000_000;
// 		// Fee to transfer into the hold register
// 		let asset = MultiAsset {
// 			id: Concrete(MultiLocation::here()),
// 			fun: Fungibility::Fungible(dollar(NATIVE_CURRENCY_ID)),
// 		};
// 		let xcm_msg = Xcm(vec![
// 			WithdrawAsset(asset.clone().into()),
// 			BuyExecution {
// 				fees: asset,
// 				weight_limit: Unlimited,
// 			},
// 			Transact {
// 				origin_type: OriginKind::SovereignAccount,
// 				require_weight_at_most: weight,
// 				call: batch_call.encode().into(),
// 			},
// 		]);
//
// 		let res = PolkadotXcm::send_xcm(Here, Parent, xcm_msg);
// 		assert!(res.is_ok());
// 	});
//
// 	KusamaNet::execute_with(|| {
// 		// 1 dollar is transferred to BOB
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&sub_account),
// 			999 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
// 			dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		// 1 dollar is given to Hold Register for XCM call and never returned.
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_001 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 	});
//
// 	Pioneer::execute_with(|| {
// 		// Construct a transfer using the RelaychainCallBuilder
// 		let transfer_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::balances_transfer_keep_alive( 			AccountId::from(BOB),
// 			dollar(NATIVE_CURRENCY_ID),
// 		);
// 		let batch_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::utility_as_derivative_call(transfer_call, 0); 		let finalized_call =
// RelayChainCallBuilder::<Runtime, ParachainInfo>::finalize_call_into_xcm_message( 			batch_call,
// 			dollar(NATIVE_CURRENCY_ID),
// 			10_000_000_000,
// 		);
//
// 		let res = PolkadotXcm::send_xcm(Here, Parent, finalized_call);
// 		assert!(res.is_ok());
// 	});
//
// 	KusamaNet::execute_with(|| {
// 		// 1 dollar is transferred to BOB
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&sub_account),
// 			998 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
// 			2 * dollar(RELAY_CHAIN_CURRENCY_ID)
// 		);
// 		// Unspent fund from the 1 dollar XCM fee is returned to the sovereign account.
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY_ID) + 999_626_666_690
// 		);
// 	});
// }
//
// #[test]
// fn trap_assets_larger_than_ed_works() {
// 	TestNet::reset();
//
// 	let mut neer_treasury_amount = 0;
// 	let (ksm_asset_amount, kar_asset_amount) = (dollar(KSM), dollar(KAR));
// 	let trader_weight_to_treasury: u128 = 96_000_000;
//
// 	Pioneer::execute_with(|| {
// 		assert_ok!(Tokens::deposit(KSM, &AccountId::from(DEFAULT), 100 * dollar(KSM)));
// 		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&AccountId::from(DEFAULT), 100 *
// dollar(KAR));
//
// 		neer_treasury_amount = Currencies::free_balance(KAR, &TreasuryModuleAccount::get());
// 	});
//
// 	let assets: MultiAsset = (Parent, ksm_asset_amount).into();
// 	KusamaNet::execute_with(|| {
// 		let xcm = vec![
// 			WithdrawAsset(assets.clone().into()),
// 			BuyExecution {
// 				fees: assets,
// 				weight_limit: Limited(dollar(KSM) as u64),
// 			},
// 			WithdrawAsset(
// 				(
// 					(Parent, X2(Parachain(2000), GeneralKey(KAR.encode()))),
// 					kar_asset_amount,
// 				)
// 					.into(),
// 			),
// 		];
// 		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
// 			Here,
// 			Parachain(2000).into(),
// 			Xcm(xcm),
// 		));
// 	});
// 	Pioneer::execute_with(|| {
// 		assert!(System::events()
// 			.iter()
// 			.any(|r| matches!(r.event, Event::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _)))));
//
// 		assert_eq!(
// 			trader_weight_to_treasury + dollar(KSM),
// 			Currencies::free_balance(KSM, &TreasuryModuleAccount::get())
// 		);
// 		assert_eq!(
// 			neer_treasury_amount,
// 			Currencies::free_balance(KAR, &TreasuryModuleAccount::get())
// 		);
// 	});
// }
//
// #[test]
// fn trap_assets_lower_than_ed_works() {
// 	TestNet::reset();
//
// 	let mut neer_treasury_amount = 0;
// 	let (ksm_asset_amount, kar_asset_amount) = (dollar(100), dollar(10000));
//
// 	Pioneer::execute_with(|| {
// 		assert_ok!(Tokens::deposit(
// 			KSM,
// 			&AccountId::from(DEFAULT),
// 			dollar(RELAY_CHAIN_CURRENCY_ID)
// 		));
// 		let _ =
// 			pallet_balances::Pallet::<Runtime>::deposit_creating(&AccountId::from(DEFAULT),
// dollar(NATIVE_CURRENCY_ID)); 		neer_treasury_amount = Currencies::free_balance(NATIVE_CURRENCY,
// &TreasuryModuleAccount::get()); 	});
//
// 	let assets: MultiAsset = (Parent, ksm_asset_amount).into();
// 	KusamaNet::execute_with(|| {
// 		let xcm = vec![
// 			WithdrawAsset(assets.clone().into()),
// 			BuyExecution {
// 				fees: assets,
// 				weight_limit: Limited(dollar(RELAY_CHAIN_CURRENCY_ID) as u64),
// 			},
// 			WithdrawAsset(
// 				(
// 					(Parent, X2(Parachain(2000), GeneralKey(NATIVE_CURRENCY.encode()))),
// 					kar_asset_amount,
// 				)
// 					.into(),
// 			),
// 			// two asset left in holding register, they both lower than ED, so goes to treasury.
// 		];
// 		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
// 			Here,
// 			Parachain(2000).into(),
// 			Xcm(xcm),
// 		));
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(
// 			System::events()
// 				.iter()
// 				.find(|r| matches!(r.event, Event::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _)))),
// 			None
// 		);
//
// 		assert_eq!(
// 			ksm_asset_amount + dollar(RELAY_CHAIN_CURRENCY_ID),
// 			Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryModuleAccount::get())
// 		);
// 		assert_eq!(
// 			kar_asset_amount,
// 			Currencies::free_balance(NATIVE_CURRENCY, &TreasuryModuleAccount::get()) - neer_treasury_amount
// 		);
// 	});
// }
//
// #[test]
// fn sibling_trap_assets_works() {
// 	TestNet::reset();
//
// 	let mut neer_treasury_amount = 0;
// 	let (kar_asset_amount, neer_asset_amount) = (dollar(100), dollar(10000));
//
// 	fn sibling_account() -> AccountId {
// 		use sp_runtime::traits::AccountIdConversion;
// 		polkadot_parachain::primitives::Sibling::from(2001).into_account()
// 	}
//
// 	Pioneer::execute_with(|| {
// 		assert_ok!(Tokens::deposit(BNC, &sibling_account(), dollar(PARACHAIN_CURRENCY_ID)));
// 		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&sibling_account(),
// dollar(NATIVE_CURRENCY_ID)); 		neer_treasury_amount = Currencies::free_balance(NATIVE_CURRENCY,
// &TreasuryModuleAccount::get()); 	});
//
// 	Sibling::execute_with(|| {
// 		let assets: MultiAsset = (
// 			(Parent, X2(Parachain(2000), GeneralKey(NATIVE_CURRENCY.encode()))),
// 			neer_asset_amount,
// 		)
// 			.into();
// 		let xcm = vec![
// 			WithdrawAsset(assets.clone().into()),
// 			BuyExecution {
// 				fees: assets,
// 				weight_limit: Unlimited,
// 			},
// 			WithdrawAsset(
// 				(
// 					(
// 						Parent,
// 						X2(Parachain(2001), GeneralKey(parachains::karura::KAR_KEY.to_vec())),
// 					),
// 					kar_asset_amount,
// 				)
// 					.into(),
// 			),
// 		];
// 		assert_ok!(pallet_xcm::Pallet::<Runtime>::send_xcm(
// 			Here,
// 			(Parent, Parachain(2000)),
// 			Xcm(xcm),
// 		));
// 	});
//
// 	Pioneer::execute_with(|| {
// 		assert_eq!(
// 			System::events()
// 				.iter()
// 				.find(|r| matches!(r.event, Event::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _)))),
// 			None
// 		);
// 		assert_eq!(
// 			Currencies::free_balance(NATIVE_CURRENCY, &TreasuryModuleAccount::get()) - neer_treasury_amount,
// 			neer_asset_amount
// 		);
// 		assert_eq!(
// 			Currencies::free_balance(PARA_CHAIN_CURRENCY, &TreasuryModuleAccount::get()),
// 			kar_asset_amount
// 		);
// 	});
// }
