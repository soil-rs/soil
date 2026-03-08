// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The Substrate runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limits.
#![recursion_limit = "1024"]

extern crate alloc;

use subsoil::runtime::str_array as s;

#[cfg(feature = "runtime-benchmarks")]
use subsoil::core::crypto::FromEntropy;
#[cfg(feature = "runtime-benchmarks")]
use plant_asset_rate::AssetKindFactory;
#[cfg(feature = "runtime-benchmarks")]
use plant_multi_asset_bounties::ArgumentsFactory as PalletMultiAssetBountiesArgumentsFactory;
#[cfg(feature = "runtime-benchmarks")]
use plant_treasury::ArgumentsFactory as PalletTreasuryArgumentsFactory;

use alloc::{vec, vec::Vec};
use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
pub use soil_test_staging_node_primitives::{AccountId, Signature};
use soil_test_staging_node_primitives::{AccountIndex, Balance, BlockNumber, Hash, Moment, Nonce};
use plant_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use static_assertions::const_assert;
use subsoil::consensus::beefy::{
	ecdsa_crypto::{AuthorityId as BeefyId, Signature as BeefySignature},
	mmr::MmrLeafVersion,
};
use subsoil::consensus::grandpa::AuthorityId as GrandpaId;
use subsoil::core::{crypto::KeyTypeId, OpaqueMetadata};
use subsoil::inherents::{CheckInherentsResult, InherentData};
use subsoil::runtime::{
	curve::PiecewiseLinear,
	traits::{
		self, AccountIdConversion, BlakeTwo256, Block as BlockT, Bounded, ConvertInto,
		MaybeConvert, NumberFor, OpaqueKeys, SaturatedConversion, StaticLookup,
	},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Debug, FixedPointNumber, FixedU128, MultiSignature, MultiSigner, Perbill,
	Percent, Permill, Perquintill,
};
use subsoil::std::{borrow::Cow, prelude::*};
#[cfg(any(feature = "std", test))]
use subsoil::version::NativeVersion;
use subsoil::version::RuntimeVersion;
use plant_asset_conversion::{AccountIdConverter, Ascending, Chain, WithFirstAsset};
use plant_asset_conversion_tx_payment::SwapAssetAdapter;
use plant_broker::{CoreAssignment, CoreIndex, CoretimeInterface, PartsOf57600, TaskId};
use plant_election_provider_multi_phase::{GeometricDepositBase, SolutionAccuracyOf};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, BalancingConfig, ElectionDataProvider, SequentialPhragmen, VoteWeight,
};
use plant_identity::legacy::IdentityInfo;
use plant_im_online::sr25519::AuthorityId as ImOnlineId;
use plant_nfts::PalletFeatures;
use plant_nis::WithMaximumOf;
use plant_nomination_pools::PoolId;
use plant_session::historical as pallet_session_historical;
use topsoil_support::weights::IdentityFee;
use topsoil_support::{
	derive_impl,
	dispatch::DispatchClass,
	dynamic_params::{dynamic_pallet_params, dynamic_params},
	genesis_builder_helper::{build_state, get_preset},
	instances::{Instance1, Instance2},
	ord_parameter_types,
	pallet_prelude::Get,
	parameter_types,
	traits::{
		fungible::{
			Balanced, Credit, HoldConsideration, ItemOf, NativeFromLeft, NativeOrWithId, UnionOf,
		},
		tokens::{
			imbalance::{ResolveAssetTo, ResolveTo},
			nonfungibles_v2::Inspect,
			pay::PayAssetFromAccount,
			GetSalary, PayFromAccount, PayWithFungibles,
		},
		AsEnsureOriginWithArg, ConstBool, ConstU128, ConstU16, ConstU32, ConstU64,
		ConstantStoragePrice, Contains, Currency, EitherOfDiverse, EnsureOriginWithArg,
		EqualPrivilegeOnly, InsideBoth, InstanceFilter, KeyOwnerProofSystem, LinearStoragePrice,
		OnInitialize,
		LockIdentifier, Nothing, OnUnbalanced, VariantCountOf, WithdrawReasons,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		ConstantMultiplier, Weight,
	},
	BoundedVec, PalletId,
};
use topsoil_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot, EnsureRootWithSuccess, EnsureSigned, EnsureSignedBy, EnsureWithSuccess,
};
use plant_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
pub use plant_transaction_payment::{FungibleAdapter, Multiplier, TargetedFeeAdjustment};
use plant_tx_pause::RuntimeCallNameOf;

#[cfg(any(feature = "std", test))]
pub use subsoil::runtime::BuildStorage;
#[cfg(any(feature = "std", test))]
pub use plant_balances::Call as BalancesCall;
#[cfg(any(feature = "std", test))]
pub use plant_sudo::Call as SudoCall;
#[cfg(any(feature = "std", test))]
pub use topsoil_system::Call as SystemCall;

pub use plant_staking::StakerStatus;

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod impls;
#[cfg(not(feature = "runtime-benchmarks"))]
use impls::AllianceIdentityVerifier;
use impls::AllianceProposalProvider;

/// Constant values used within the runtime.
pub mod constants;
use constants::{currency::*, time::*};
use subsoil::runtime::{generic, generic::Era};

/// Generated voter bag information.
mod voter_bags;

/// Runtime API definition for assets.
pub mod assets_api;

/// Genesis presets used by this runtime.
pub mod genesis_config_presets;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Max size for serialized extrinsic params for this testing runtime.
/// This is a quite arbitrary but empirically battle tested value.
#[cfg(test)]
pub const CALL_PARAMS_MAX_SIZE: usize = 512;

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	)
}

/// Runtime version.
#[subsoil::version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("node"),
	impl_name: alloc::borrow::Cow::Borrowed("soil-test-staging-node"),
	authoring_version: 10,
	// Per convention: if the runtime behavior changes, increment spec_version
	// and set impl_version to 0. If only runtime
	// implementation changes and behavior does not, then leave spec_version as
	// is and increment impl_version.
	spec_version: 268,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 2,
	system_version: 1,
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: subsoil::consensus::babe::BabeEpochConfiguration =
	subsoil::consensus::babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: subsoil::consensus::babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};

/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength = BlockLength::builder()
		.max_length(5 * 1024 * 1024)
		.modify_max_length_for_class(DispatchClass::Normal, |m| {
			*m = NORMAL_DISPATCH_RATIO * *m
		})
		.build();
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub MaxCollectivesProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

/// Calls that can bypass the safe-mode pallet.
pub struct SafeModeWhitelistedCalls;
impl Contains<RuntimeCall> for SafeModeWhitelistedCalls {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::System(_) | RuntimeCall::SafeMode(_) | RuntimeCall::TxPause(_) => true,
			_ => false,
		}
	}
}

/// Calls that cannot be paused by the tx-pause pallet.
pub struct TxPauseWhitelistedCalls;
/// Whitelist `Balances::transfer_keep_alive`, all others are pauseable.
impl Contains<RuntimeCallNameOf<Runtime>> for TxPauseWhitelistedCalls {
	fn contains(full_name: &RuntimeCallNameOf<Runtime>) -> bool {
		match (full_name.0.as_slice(), full_name.1.as_slice()) {
			(b"Balances", b"transfer_keep_alive") => true,
			_ => false,
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct AssetRateArguments;
#[cfg(feature = "runtime-benchmarks")]
impl AssetKindFactory<NativeOrWithId<u32>> for AssetRateArguments {
	fn create_asset_kind(seed: u32) -> NativeOrWithId<u32> {
		if !seed.is_multiple_of(2) {
			NativeOrWithId::Native
		} else {
			NativeOrWithId::WithId(seed / 2)
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct PalletTreasuryArguments;
#[cfg(feature = "runtime-benchmarks")]
impl PalletTreasuryArgumentsFactory<NativeOrWithId<u32>, AccountId> for PalletTreasuryArguments {
	fn create_asset_kind(seed: u32) -> NativeOrWithId<u32> {
		if !seed.is_multiple_of(2) {
			NativeOrWithId::Native
		} else {
			NativeOrWithId::WithId(seed / 2)
		}
	}

	fn create_beneficiary(seed: [u8; 32]) -> AccountId {
		AccountId::from_entropy(&mut seed.as_slice()).unwrap()
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct PalletMultiAssetBountiesArguments;
#[cfg(feature = "runtime-benchmarks")]
impl PalletMultiAssetBountiesArgumentsFactory<NativeOrWithId<u32>, AccountId, u128>
	for PalletMultiAssetBountiesArguments
{
	fn create_asset_kind(seed: u32) -> NativeOrWithId<u32> {
		if !seed.is_multiple_of(2) {
			NativeOrWithId::Native
		} else {
			NativeOrWithId::WithId(seed / 2)
		}
	}

	fn create_beneficiary(seed: [u8; 32]) -> AccountId {
		AccountId::from_entropy(&mut seed.as_slice()).unwrap()
	}
}

impl plant_tx_pause::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PauseOrigin = EnsureRoot<AccountId>;
	type UnpauseOrigin = EnsureRoot<AccountId>;
	type WhitelistedCalls = TxPauseWhitelistedCalls;
	type MaxNameLen = ConstU32<256>;
	type WeightInfo = plant_tx_pause::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const EnterDuration: BlockNumber = 4 * HOURS;
	pub const EnterDepositAmount: Balance = 2_000_000 * DOLLARS;
	pub const ExtendDuration: BlockNumber = 2 * HOURS;
	pub const ExtendDepositAmount: Balance = 1_000_000 * DOLLARS;
	pub const ReleaseDelay: u32 = 2 * DAYS;
}

impl plant_safe_mode::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type WhitelistedCalls = SafeModeWhitelistedCalls;
	type EnterDuration = EnterDuration;
	type EnterDepositAmount = EnterDepositAmount;
	type ExtendDuration = ExtendDuration;
	type ExtendDepositAmount = ExtendDepositAmount;
	type ForceEnterOrigin = EnsureRootWithSuccess<AccountId, ConstU32<9>>;
	type ForceExtendOrigin = EnsureRootWithSuccess<AccountId, ConstU32<11>>;
	type ForceExitOrigin = EnsureRoot<AccountId>;
	type ForceDepositOrigin = EnsureRoot<AccountId>;
	type ReleaseDelay = ReleaseDelay;
	type Notify = ();
	type WeightInfo = plant_safe_mode::weights::SubstrateWeight<Runtime>;
}

#[derive_impl(topsoil_system::config_preludes::SolochainDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BaseCallFilter = InsideBoth<SafeMode, TxPause>;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type DbWeight = RocksDbWeight;
	type Nonce = Nonce;
	type Hash = Hash;
	type AccountId = AccountId;
	type Lookup = Indices;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type Version = Version;
	type AccountData = plant_balances::AccountData<Balance>;
	type SystemWeightInfo = topsoil_system::weights::SubstrateWeight<Runtime>;
	type SS58Prefix = ConstU16<42>;
	type MaxConsumers = ConstU32<16>;
	type MultiBlockMigrator = ();
	type SingleBlockMigrations = Migrations;
}

impl plant_insecure_randomness_collective_flip::Config for Runtime {}

impl plant_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = plant_utility::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32);
}

impl plant_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = ConstU32<100>;
	type WeightInfo = plant_multisig::weights::SubstrateWeight<Runtime>;
	type BlockNumberProvider = topsoil_system::Pallet<Runtime>;
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	DecodeWithMemTracking,
	Debug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances(..)
					| RuntimeCall::Assets(..)
					| RuntimeCall::Uniques(..)
					| RuntimeCall::Nfts(..)
					| RuntimeCall::Vesting(plant_vesting::Call::vested_transfer { .. })
					| RuntimeCall::Indices(plant_indices::Call::transfer { .. })
			),
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..)
					| RuntimeCall::Council(..)
					| RuntimeCall::Society(..)
					| RuntimeCall::TechnicalCommittee(..)
					| RuntimeCall::Elections(..)
					| RuntimeCall::Treasury(..)
			),
			ProxyType::Staking => {
				matches!(c, RuntimeCall::Staking(..) | RuntimeCall::FastUnstake(..))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl plant_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = plant_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type BlockNumberProvider = topsoil_system::Pallet<Runtime>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		RuntimeBlockWeights::get().max_block;
}

impl plant_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	#[cfg(feature = "runtime-benchmarks")]
	type MaxScheduledPerBlock = ConstU32<512>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = plant_scheduler::weights::SubstrateWeight<Runtime>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = Preimage;
	type BlockNumberProvider = topsoil_system::Pallet<Runtime>;
}

impl plant_glutton::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = plant_glutton::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const PreimageHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::Preimage(plant_preimage::HoldReason::Preimage);
}

impl plant_preimage::Config for Runtime {
	type WeightInfo = plant_preimage::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<
			dynamic_params::storage::BaseDeposit,
			dynamic_params::storage::ByteDeposit,
			Balance,
		>,
	>;
}

parameter_types! {
	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl plant_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = plant_babe::ExternalTrigger;
	type DisabledValidators = Session;
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
	type MaxNominators = MaxNominators;
	type KeyOwnerProof = subsoil::session::MembershipProof;
	type EquivocationReportSystem =
		plant_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

parameter_types! {
	pub const IndexDeposit: Balance = 1 * DOLLARS;
}

impl plant_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = plant_indices::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1 * DOLLARS;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl plant_balances::Config for Runtime {
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = topsoil_system::Pallet<Runtime>;
	type WeightInfo = plant_balances::weights::SubstrateWeight<Runtime>;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
	type DoneSlashHandler = ();
}

parameter_types! {
	pub const TransactionByteFee: Balance = 10 * MILLICENTS;
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 10u128);
	pub MaximumMultiplier: Multiplier = Bounded::max_value();
}

impl plant_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = FungibleAdapter<Balances, ResolveTo<TreasuryAccount, Balances>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = TargetedFeeAdjustment<
		Self,
		TargetBlockFullness,
		AdjustmentVariable,
		MinimumMultiplier,
		MaximumMultiplier,
	>;
	type WeightInfo = plant_transaction_payment::weights::SubstrateWeight<Runtime>;
}

pub type AssetsFreezerInstance = plant_assets_freezer::Instance1;
impl plant_assets_freezer::Config<AssetsFreezerInstance> for Runtime {
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RuntimeEvent = RuntimeEvent;
}

impl plant_asset_conversion_tx_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = NativeOrWithId<u32>;
	type OnChargeAssetTransaction = SwapAssetAdapter<
		Native,
		NativeAndAssets,
		AssetConversion,
		ResolveAssetTo<TreasuryAccount, NativeAndAssets>,
	>;
	type WeightInfo = plant_asset_conversion_tx_payment::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetConversionTxHelper;
}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl plant_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = plant_timestamp::weights::SubstrateWeight<Runtime>;
}

impl plant_authorship::Config for Runtime {
	type FindAuthor = plant_session::FindAccountFromAuthorIndex<Self, Babe>;
	type EventHandler = (Staking, ImOnline);
}

subsoil::impl_opaque_keys! {
	pub struct SessionKeys {
		pub grandpa: Grandpa,
		pub babe: Babe,
		pub im_online: ImOnline,
		pub authority_discovery: AuthorityDiscovery,
		pub mixnet: Mixnet,
		pub beefy: Beefy,
	}
}

impl plant_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as topsoil_system::Config>::AccountId;
	type ValidatorIdOf = subsoil::runtime::traits::ConvertInto;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = plant_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type DisablingStrategy = plant_session::disabling::UpToLimitWithReEnablingDisablingStrategy;
	type WeightInfo = plant_session::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type KeyDeposit = ();
}

impl plant_session::historical::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = ();
	type FullIdentificationOf = plant_staking::UnitIdentificationOf<Self>;
}

plant_staking_macros::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	pub const SessionsPerEra: subsoil::staking::SessionIndex = 6;
	pub const BondingDuration: subsoil::staking::EraIndex = 24 * 28;
	pub const SlashDeferDuration: subsoil::staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominators: u32 = 64;
	pub const MaxControllersInDeprecationBatch: u32 = 5900;
	pub OffchainRepeat: BlockNumber = 5;
	pub HistoryDepth: u32 = 84;
}

/// Upper limit on the number of NPOS nominations.
const MAX_QUOTA_NOMINATIONS: u32 = 16;

pub struct StakingBenchmarkingConfig;
impl plant_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = ConstU32<5000>;
	type MaxValidators = ConstU32<1000>;
}

impl plant_staking::Config for Runtime {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = subsoil::staking::currency_to_vote::U128CurrencyToVote;
	type RewardRemainder = ResolveTo<TreasuryAccount, Balances>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Slash = ResolveTo<TreasuryAccount, Balances>; // send the slashed funds to the treasury.
	type Reward = (); // rewards are minted from the void
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	/// A super-majority of the council can cancel the slash.
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
	type SessionInterface = Self;
	type EraPayout = plant_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type MaxExposurePageSize = ConstU32<256>;
	type ElectionProvider = ElectionProviderMultiPhase;
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type VoterList = VoterList;
	type NominationsQuota = plant_staking::FixedNominationsQuota<MAX_QUOTA_NOMINATIONS>;
	// This a placeholder, to be introduced in the next PR as an instance of bags-list
	type TargetList = plant_staking::UseValidatorsMap<Self>;
	type MaxUnlockingChunks = ConstU32<32>;
	type MaxControllersInDeprecationBatch = MaxControllersInDeprecationBatch;
	type HistoryDepth = HistoryDepth;
	type EventListeners = (NominationPools, DelegatedStaking);
	type WeightInfo = plant_staking::weights::SubstrateWeight<Runtime>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type Filter = Nothing;
	type MaxValidatorSet = ConstU32<1000>;
}

impl plant_fast_unstake::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ControlOrigin = topsoil_system::EnsureRoot<AccountId>;
	type BatchSize = ConstU32<64>;
	type Deposit = ConstU128<{ DOLLARS }>;
	type Currency = Balances;
	type Staking = Staking;
	type MaxErasToCheckPerBlock = ConstU32<1>;
	type WeightInfo = ();
}
parameter_types! {
	// phase durations. 1/4 of the last session for each.
	pub const SignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;
	pub const UnsignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;

	// signed config
	pub const SignedRewardBase: Balance = 1 * DOLLARS;
	pub const SignedFixedDeposit: Balance = 1 * DOLLARS;
	pub const SignedDepositIncreaseFactor: Percent = Percent::from_percent(10);
	pub const SignedDepositByte: Balance = 1 * CENTS;

	// miner configs
	pub const MultiPhaseUnsignedPriority: TransactionPriority = StakingUnsignedPriority::get() - 1u64;
	pub MinerMaxWeight: Weight = RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic.expect("Normal extrinsics have a weight limit configured; qed")
		.saturating_sub(BlockExecutionWeight::get());
	// Solution can occupy 90% of normal block size
	pub MinerMaxLength: u32 = Perbill::from_rational(9u32, 10) *
		*RuntimeBlockLength::get()
		.max
		.get(DispatchClass::Normal);
}

plant_election_provider::generate_solution_type!(
	#[compact]
	pub struct NposSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = subsoil::runtime::PerU16,
		MaxVoters = MaxElectingVotersSolution,
	>(16)
);

parameter_types! {
	// Note: the EPM in this runtime runs the election on-chain. The election bounds must be
	// carefully set so that an election round fits in one block.
	pub ElectionBoundsMultiPhase: ElectionBounds = ElectionBoundsBuilder::default()
		.voters_count(10_000.into()).targets_count(1_500.into()).build();
	pub ElectionBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default()
		.voters_count(5_000.into()).targets_count(1_250.into()).build();

	pub MaxNominations: u32 = <NposSolution16 as plant_election_provider::NposSolution>::LIMIT as u32;
	pub MaxElectingVotersSolution: u32 = 40_000;
	// The maximum winners that can be elected by the Election pallet which is equivalent to the
	// maximum active validators the staking pallet can have.
	pub MaxActiveValidators: u32 = 1000;
}

/// The numbers configured here could always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory. For now, we set them to smaller values
/// since the staking is bounded and the weight pipeline takes hours for this single pallet.
pub struct ElectionProviderBenchmarkConfig;
impl plant_election_provider_multi_phase::BenchmarkingConfig for ElectionProviderBenchmarkConfig {
	const VOTERS: [u32; 2] = [1000, 2000];
	const TARGETS: [u32; 2] = [500, 1000];
	const ACTIVE_VOTERS: [u32; 2] = [500, 800];
	const DESIRED_TARGETS: [u32; 2] = [200, 400];
	const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
	const MINER_MAXIMUM_VOTERS: u32 = 1000;
	const MAXIMUM_TARGETS: u32 = 300;
}

/// Maximum number of iterations for balancing that will be executed in the embedded OCW
/// miner of election provider multi phase.
pub const MINER_MAX_ITERATIONS: u32 = 10;

/// A source of random balance for NposSolver, which is meant to be run by the OCW election miner.
pub struct OffchainRandomBalancing;
impl Get<Option<BalancingConfig>> for OffchainRandomBalancing {
	fn get() -> Option<BalancingConfig> {
		use subsoil::runtime::traits::TrailingZeroInput;
		let iterations = match MINER_MAX_ITERATIONS {
			0 => 0,
			max => {
				let seed = subsoil::io::offchain::random_seed();
				let random = <u32>::decode(&mut TrailingZeroInput::new(&seed))
					.expect("input is padded with zeroes; qed")
					% max.saturating_add(1);
				random as usize
			},
		};

		let config = BalancingConfig { iterations, tolerance: 0 };
		Some(config)
	}
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type Sort = ConstBool<true>;
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, SolutionAccuracyOf<Runtime>>;
	type DataProvider = Staking;
	type WeightInfo = plant_election_provider::weights::SubstrateWeight<Runtime>;
	type Bounds = ElectionBoundsOnChain;
	type MaxBackersPerWinner = MaxElectingVotersSolution;
	type MaxWinnersPerPage = MaxActiveValidators;
}

impl plant_election_provider_multi_phase::MinerConfig for Runtime {
	type AccountId = AccountId;
	type MaxLength = MinerMaxLength;
	type MaxWeight = MinerMaxWeight;
	type Solution = NposSolution16;
	type MaxVotesPerVoter =
	<<Self as plant_election_provider_multi_phase::Config>::DataProvider as ElectionDataProvider>::MaxVotesPerVoter;
	type MaxWinners = MaxActiveValidators;
	type MaxBackersPerWinner = MaxElectingVotersSolution;

	// The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
	// weight estimate function is wired to this call's weight.
	fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
		<
			<Self as plant_election_provider_multi_phase::Config>::WeightInfo
			as
			plant_election_provider_multi_phase::WeightInfo
		>::submit_unsigned(v, t, a, d)
	}
}

impl plant_election_provider_multi_phase::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EstimateCallFee = TransactionPayment;
	type SignedPhase = SignedPhase;
	type UnsignedPhase = UnsignedPhase;
	type BetterSignedThreshold = ();
	type OffchainRepeat = OffchainRepeat;
	type MinerTxPriority = MultiPhaseUnsignedPriority;
	type MinerConfig = Self;
	type SignedMaxSubmissions = ConstU32<10>;
	type SignedRewardBase = SignedRewardBase;
	type SignedDepositBase =
		GeometricDepositBase<Balance, SignedFixedDeposit, SignedDepositIncreaseFactor>;
	type SignedDepositByte = SignedDepositByte;
	type SignedMaxRefunds = ConstU32<3>;
	type SignedDepositWeight = ();
	type SignedMaxWeight = MinerMaxWeight;
	type SlashHandler = (); // burn slashes
	type RewardHandler = (); // rewards are minted from the void
	type DataProvider = Staking;
	type Fallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GovernanceFallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type Solver = SequentialPhragmen<AccountId, SolutionAccuracyOf<Self>, OffchainRandomBalancing>;
	type ForceOrigin = EnsureRootOrHalfCouncil;
	type MaxWinners = MaxActiveValidators;
	type ElectionBounds = ElectionBoundsMultiPhase;
	type BenchmarkingConfig = ElectionProviderBenchmarkConfig;
	type WeightInfo = plant_election_provider_multi_phase::weights::SubstrateWeight<Self>;
	type MaxBackersPerWinner = MaxElectingVotersSolution;
}

parameter_types! {
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
	pub const AutoRebagNumber: u32 = 10;
}

type VoterBagsListInstance = plant_bags_list::Instance1;
impl plant_bags_list::Config<VoterBagsListInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = plant_bags_list::weights::SubstrateWeight<Runtime>;
	/// The voter bags-list is loosely kept up to date, and the real source of truth for the score
	/// of each node is the staking pallet.
	type ScoreProvider = Staking;
	type BagThresholds = BagThresholds;
	type MaxAutoRebagPerBlock = AutoRebagNumber;
	type Score = VoteWeight;
}

parameter_types! {
	pub const DelegatedStakingPalletId: PalletId = PalletId(*b"py/dlstk");
	pub const SlashRewardFraction: Perbill = Perbill::from_percent(1);
}

impl plant_delegated_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = DelegatedStakingPalletId;
	type Currency = Balances;
	type OnSlash = ();
	type SlashRewardFraction = SlashRewardFraction;
	type RuntimeHoldReason = RuntimeHoldReason;
	type CoreStaking = Staking;
}

parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}

use subsoil::runtime::traits::{Convert, Keccak256};
pub struct BalanceToU256;
impl Convert<Balance, subsoil::core::U256> for BalanceToU256 {
	fn convert(balance: Balance) -> subsoil::core::U256 {
		subsoil::core::U256::from(balance)
	}
}
pub struct U256ToBalance;
impl Convert<subsoil::core::U256, Balance> for U256ToBalance {
	fn convert(n: subsoil::core::U256) -> Balance {
		n.try_into().unwrap_or(Balance::max_value())
	}
}

impl plant_nomination_pools::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RewardCounter = FixedU128;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type StakeAdapter =
		plant_nomination_pools::adapter::DelegateStake<Self, Staking, DelegatedStaking>;
	type PostUnbondingPoolsWindow = PostUnbondPoolsWindow;
	type MaxMetadataLen = ConstU32<256>;
	type MaxUnbonding = ConstU32<8>;
	type PalletId = NominationPoolsPalletId;
	type MaxPointsToBalance = MaxPointsToBalance;
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
	type BlockNumberProvider = System;
	type Filter = Nothing;
}

parameter_types! {
	pub const VoteLockingPeriod: BlockNumber = 30 * DAYS;
}

impl plant_conviction_voting::Config for Runtime {
	type WeightInfo = plant_conviction_voting::weights::SubstrateWeight<Self>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type VoteLockingPeriod = VoteLockingPeriod;
	type MaxVotes = ConstU32<512>;
	type MaxTurnout = topsoil_support::traits::TotalIssuanceOf<Balances, Self::AccountId>;
	type Polls = Referenda;
	type BlockNumberProvider = System;
	type VotingHooks = ();
}

parameter_types! {
	pub const AlarmInterval: BlockNumber = 1;
	pub const SubmissionDeposit: Balance = 100 * DOLLARS;
	pub const UndecidingTimeout: BlockNumber = 28 * DAYS;
}

pub struct TracksInfo;
impl plant_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = u16;
	type RuntimeOrigin = <RuntimeOrigin as topsoil_support::traits::OriginTrait>::PalletsOrigin;

	fn tracks(
	) -> impl Iterator<Item = Cow<'static, plant_referenda::Track<Self::Id, Balance, BlockNumber>>>
	{
		dynamic_params::referenda::Tracks::get().into_iter().map(Cow::Owned)
	}
	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		dynamic_params::referenda::Origins::get()
			.iter()
			.find(|(o, _)| id == o)
			.map(|(_, track_id)| *track_id)
			.ok_or(())
	}
}

impl plant_referenda::Config for Runtime {
	type WeightInfo = plant_referenda::weights::SubstrateWeight<Self>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	type Currency = plant_balances::Pallet<Self>;
	type SubmitOrigin = EnsureSigned<AccountId>;
	type CancelOrigin = EnsureRoot<AccountId>;
	type KillOrigin = EnsureRoot<AccountId>;
	type Slash = ();
	type Votes = plant_conviction_voting::VotesOf<Runtime>;
	type Tally = plant_conviction_voting::TallyOf<Runtime>;
	type SubmissionDeposit = SubmissionDeposit;
	type MaxQueued = ConstU32<100>;
	type UndecidingTimeout = UndecidingTimeout;
	type AlarmInterval = AlarmInterval;
	type Tracks = TracksInfo;
	type Preimages = Preimage;
	type BlockNumberProvider = System;
}

impl plant_referenda::Config<plant_referenda::Instance2> for Runtime {
	type WeightInfo = plant_referenda::weights::SubstrateWeight<Self>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	type Currency = plant_balances::Pallet<Self>;
	type SubmitOrigin = EnsureSigned<AccountId>;
	type CancelOrigin = EnsureRoot<AccountId>;
	type KillOrigin = EnsureRoot<AccountId>;
	type Slash = ();
	type Votes = plant_ranked_collective::Votes;
	type Tally = plant_ranked_collective::TallyOf<Runtime>;
	type SubmissionDeposit = SubmissionDeposit;
	type MaxQueued = ConstU32<100>;
	type UndecidingTimeout = UndecidingTimeout;
	type AlarmInterval = AlarmInterval;
	type Tracks = TracksInfo;
	type Preimages = Preimage;
	type BlockNumberProvider = System;
}

impl plant_ranked_collective::Config for Runtime {
	type WeightInfo = plant_ranked_collective::weights::SubstrateWeight<Self>;
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = Self::DemoteOrigin;
	type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type DemoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type ExchangeOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type Polls = RankedPolls;
	type MinRankOfClass = traits::Identity;
	type VoteWeight = plant_ranked_collective::Geometric;
	type MemberSwappedHandler = (CoreFellowship, Salary);
	type MaxMemberCount = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkSetup = (CoreFellowship, Salary);
}

impl plant_remark::Config for Runtime {
	type WeightInfo = plant_remark::weights::SubstrateWeight<Self>;
	type RuntimeEvent = RuntimeEvent;
}

impl plant_test_root::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const VotingPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * 24 * 60 * MINUTES;
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub const EnactmentPeriod: BlockNumber = 30 * 24 * 60 * MINUTES;
	pub const CooloffPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const MaxProposals: u32 = 100;
}

impl plant_democracy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod; // Same as EnactmentPeriod
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	/// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>;
	type SubmitOrigin = EnsureSigned<AccountId>;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>;
	type InstantOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>;
	type InstantAllowed = ConstBool<true>;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin =
		plant_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>,
	>;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cool-off period.
	type VetoOrigin = plant_collective::EnsureMember<AccountId, TechnicalCollective>;
	type CooloffPeriod = CooloffPeriod;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = ConstU32<100>;
	type WeightInfo = plant_democracy::weights::SubstrateWeight<Runtime>;
	type MaxProposals = MaxProposals;
	type Preimages = Preimage;
	type MaxDeposits = ConstU32<100>;
	type MaxBlacklisted = ConstU32<100>;
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub const ProposalDepositOffset: Balance = ExistentialDeposit::get() + ExistentialDeposit::get();
	pub const ProposalHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::Council(plant_collective::HoldReason::ProposalSubmission);
}

type CouncilCollective = plant_collective::Instance1;
impl plant_collective::Config<CouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = plant_collective::PrimeDefaultVote;
	type WeightInfo = plant_collective::weights::SubstrateWeight<Runtime>;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		ProposalHoldReason,
		plant_collective::deposit::Delayed<
			ConstU32<2>,
			plant_collective::deposit::Linear<ConstU32<2>, ProposalDepositOffset>,
		>,
		u32,
	>;
}

parameter_types! {
	pub const CandidacyBond: Balance = 10 * DOLLARS;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub const VotingBondBase: Balance = deposit(1, 64);
	// additional data per vote is 32 bytes (account id).
	pub const VotingBondFactor: Balance = deposit(0, 32);
	pub const TermDuration: BlockNumber = 7 * DAYS;
	pub const DesiredMembers: u32 = 13;
	pub const DesiredRunnersUp: u32 = 7;
	pub const MaxVotesPerVoter: u32 = 16;
	pub const MaxVoters: u32 = 256;
	pub const MaxCandidates: u32 = 128;
	pub const ElectionsPhragmenPalletId: LockIdentifier = *b"phrelect";
}

// Make sure that there are no more than `MaxMembers` members elected via elections-phragmen.
const_assert!(DesiredMembers::get() <= CouncilMaxMembers::get());

impl plant_elections_phragmen::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = ElectionsPhragmenPalletId;
	type Currency = Balances;
	type ChangeMembers = Council;
	// NOTE: this implies that council's genesis members cannot be set directly and must come from
	// this module.
	type InitializeMembers = Council;
	type CurrencyToVote = subsoil::staking::currency_to_vote::U128CurrencyToVote;
	type CandidacyBond = CandidacyBond;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;
	type LoserCandidate = ();
	type KickedMember = ();
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration;
	type MaxVoters = MaxVoters;
	type MaxVotesPerVoter = MaxVotesPerVoter;
	type MaxCandidates = MaxCandidates;
	type WeightInfo = plant_elections_phragmen::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 5 * DAYS;
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;
}

type TechnicalCollective = plant_collective::Instance2;
impl plant_collective::Config<TechnicalCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = TechnicalMaxProposals;
	type MaxMembers = TechnicalMaxMembers;
	type DefaultVote = plant_collective::PrimeDefaultVote;
	type WeightInfo = plant_collective::weights::SubstrateWeight<Runtime>;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
}

type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	plant_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;
impl plant_membership::Config<plant_membership::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrHalfCouncil;
	type RemoveOrigin = EnsureRootOrHalfCouncil;
	type SwapOrigin = EnsureRootOrHalfCouncil;
	type ResetOrigin = EnsureRootOrHalfCouncil;
	type PrimeOrigin = EnsureRootOrHalfCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = TechnicalMaxMembers;
	type WeightInfo = plant_membership::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: BlockNumber = 1 * DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = 1 * DOLLARS;
	pub const DataDepositPerByte: Balance = 1 * CENTS;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaximumReasonLength: u32 = 300;
	pub const MaxApprovals: u32 = 100;
	pub const MaxBalance: Balance = Balance::max_value();
	pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
}

impl plant_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type RejectOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
	>;
	type RuntimeEvent = RuntimeEvent;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = plant_treasury::weights::SubstrateWeight<Runtime>;
	type MaxApprovals = MaxApprovals;
	type SpendOrigin = EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxBalance>;
	type AssetKind = NativeOrWithId<u32>;
	type Beneficiary = AccountId;
	type BeneficiaryLookup = Indices;
	type Paymaster = PayAssetFromAccount<NativeAndAssets, TreasuryAccount>;
	type BalanceConverter = AssetRate;
	type PayoutPeriod = SpendPayoutPeriod;
	type BlockNumberProvider = System;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = PalletTreasuryArguments;
}

impl plant_asset_rate::Config for Runtime {
	type CreateOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type AssetKind = NativeOrWithId<u32>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = plant_asset_rate::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRateArguments;
}

parameter_types! {
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 5 * DOLLARS;
	pub const BountyDepositBase: Balance = 1 * DOLLARS;
	pub const CuratorDepositFromFeeMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 1 * DOLLARS;
	pub const CuratorDepositMax: Balance = 100 * DOLLARS;
	pub const BountyDepositPayoutDelay: BlockNumber = 1 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 14 * DAYS;
}

impl plant_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositFromFeeMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = plant_bounties::weights::SubstrateWeight<Runtime>;
	type ChildBountyManager = ChildBounties;
	type OnSlash = Treasury;
	type TransferAllAssets = ();
}

parameter_types! {
	/// Allocate at most 20% of each block for message processing.
	///
	/// Is set to 20% since the scheduler can already consume a maximum of 80%.
	pub MessageQueueServiceWeight: Option<Weight> = Some(Perbill::from_percent(20) * RuntimeBlockWeights::get().max_block);
}

impl plant_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	/// NOTE: Always set this to `NoopMessageProcessor` for benchmarking.
	type MessageProcessor = plant_message_queue::mock_helpers::NoopMessageProcessor<u32>;
	type Size = u32;
	type QueueChangeHandler = ();
	type QueuePausedQuery = ();
	type HeapSize = ConstU32<{ 64 * 1024 }>;
	type MaxStale = ConstU32<128>;
	type ServiceWeight = MessageQueueServiceWeight;
	type IdleMaxServiceWeight = ();
}

parameter_types! {
	pub const ChildBountyValueMinimum: Balance = 1 * DOLLARS;
	pub const MaxActiveChildBountyCount: u32 = 5;
}

impl plant_child_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
	type ChildBountyValueMinimum = ChildBountyValueMinimum;
	type WeightInfo = plant_child_bounties::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const CuratorDepositFromValueMultiplier: Permill = Permill::from_percent(10);
	pub const CuratorHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::MultiAssetBounties(plant_multi_asset_bounties::HoldReason::CuratorDeposit);
}

impl plant_multi_asset_bounties::Config for Runtime {
	type Balance = Balance;
	type RejectOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
	>;
	type SpendOrigin = EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxBalance>;
	type AssetKind = NativeOrWithId<u32>;
	type Beneficiary = AccountId;
	type BeneficiaryLookup = Indices;
	type BountyValueMinimum = BountyValueMinimum;
	type ChildBountyValueMinimum = ChildBountyValueMinimum;
	type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
	type WeightInfo = plant_multi_asset_bounties::weights::SubstrateWeight<Runtime>;
	type FundingSource = plant_multi_asset_bounties::PalletIdAsFundingSource<
		TreasuryPalletId,
		Runtime,
		subsoil::runtime::traits::Identity,
	>;
	type BountySource = plant_multi_asset_bounties::BountySourceFromPalletId<
		TreasuryPalletId,
		Runtime,
		subsoil::runtime::traits::Identity,
	>;
	type ChildBountySource = plant_multi_asset_bounties::ChildBountySourceFromPalletId<
		TreasuryPalletId,
		Runtime,
		subsoil::runtime::traits::Identity,
	>;
	type Paymaster = PayWithFungibles<NativeAndAssets, AccountId>;
	type BalanceConverter = AssetRate;
	type Preimages = Preimage;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		CuratorHoldReason,
		plant_multi_asset_bounties::CuratorDepositAmount<
			CuratorDepositFromValueMultiplier,
			CuratorDepositMin,
			CuratorDepositMax,
			Balance,
		>,
		Balance,
	>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = PalletMultiAssetBountiesArguments;
}

impl plant_tips::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = Elections;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type MaxTipAmount = ConstU128<{ 500 * DOLLARS }>;
	type WeightInfo = plant_tips::weights::SubstrateWeight<Runtime>;
	type OnSlash = Treasury;
}

impl plant_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = plant_sudo::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	/// We prioritize im-online heartbeats over election solution submission.
	pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
	pub const MaxAuthorities: u32 = 1000;
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
}

impl<LocalCall> topsoil_system::offchain::CreateTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	type Extension = TxExtension;

	fn create_transaction(call: RuntimeCall, extension: TxExtension) -> UncheckedExtrinsic {
		generic::UncheckedExtrinsic::new_transaction(call, extension)
	}
}

impl<LocalCall> topsoil_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_signed_transaction<
		C: topsoil_system::offchain::AppCrypto<Self::Public, Self::Signature>,
	>(
		call: RuntimeCall,
		public: <Signature as traits::Verify>::Signer,
		account: AccountId,
		nonce: Nonce,
	) -> Option<UncheckedExtrinsic> {
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let era = Era::mortal(period, current_block);
		let tx_ext: TxExtension = (
			topsoil_system::AuthorizeCall::<Runtime>::new(),
			topsoil_system::CheckNonZeroSender::<Runtime>::new(),
			topsoil_system::CheckSpecVersion::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(era),
			topsoil_system::CheckNonce::<Runtime>::from(nonce),
			topsoil_system::CheckWeight::<Runtime>::new(),
			plant_asset_conversion_tx_payment::ChargeAssetTxPayment::<Runtime>::from(tip, None),
			plant_metadata_hash_extension::CheckMetadataHash::new(false),
			topsoil_system::WeightReclaim::<Runtime>::new(),
		);

		let raw_payload = SignedPayload::new(call, tx_ext)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = Indices::unlookup(account);
		let (call, tx_ext, _) = raw_payload.deconstruct();
		Some(generic::UncheckedExtrinsic::new_signed(call, address, signature, tx_ext))
	}
}

impl<LocalCall> topsoil_system::offchain::CreateBare<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_bare(call: RuntimeCall) -> UncheckedExtrinsic {
		generic::UncheckedExtrinsic::new_bare(call)
	}
}

impl topsoil_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> topsoil_system::offchain::CreateTransactionBase<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type RuntimeCall = RuntimeCall;
}

impl<C> topsoil_system::offchain::CreateAuthorizedTransaction<C> for Runtime
where
	RuntimeCall: From<C>,
{
	fn create_extension() -> Self::Extension {
		(
			topsoil_system::AuthorizeCall::<Runtime>::new(),
			topsoil_system::CheckNonZeroSender::<Runtime>::new(),
			topsoil_system::CheckSpecVersion::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(Era::Immortal),
			topsoil_system::CheckNonce::<Runtime>::from(0),
			topsoil_system::CheckWeight::<Runtime>::new(),
			plant_asset_conversion_tx_payment::ChargeAssetTxPayment::<Runtime>::from(0, None),
			plant_metadata_hash_extension::CheckMetadataHash::new(false),
			topsoil_system::WeightReclaim::<Runtime>::new(),
		)
	}
}

impl plant_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = Babe;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = plant_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
}

impl plant_offences::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = plant_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

impl plant_authority_discovery::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const MaxSetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl plant_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
	type MaxNominators = MaxNominators;
	type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
	type KeyOwnerProof = subsoil::session::MembershipProof;
	type EquivocationReportSystem =
		plant_grandpa::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

parameter_types! {
	// difference of 26 bytes on-chain for the registration and 9 bytes on-chain for the identity
	// information, already accounted for by the byte deposit
	pub const BasicDeposit: Balance = deposit(1, 17);
	pub const ByteDeposit: Balance = deposit(0, 1);
	pub const UsernameDeposit: Balance = deposit(0, 32);
	pub const SubAccountDeposit: Balance = 2 * DOLLARS;   // 53 bytes on-chain
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl plant_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type ByteDeposit = ByteDeposit;
	type UsernameDeposit = UsernameDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type IdentityInformation = IdentityInfo<MaxAdditionalFields>;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRootOrHalfCouncil;
	type RegistrarOrigin = EnsureRootOrHalfCouncil;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as traits::Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
	type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
	type UsernameGracePeriod = ConstU32<{ 30 * DAYS }>;
	type MaxSuffixLength = ConstU32<7>;
	type MaxUsernameLength = ConstU32<32>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type WeightInfo = plant_identity::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const ConfigDepositBase: Balance = 5 * DOLLARS;
	pub const FriendDepositFactor: Balance = 50 * CENTS;
	pub const MaxFriends: u16 = 9;
	pub const RecoveryDeposit: Balance = 5 * DOLLARS;
}

impl plant_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = plant_recovery::weights::SubstrateWeight<Runtime>;
	type RuntimeCall = RuntimeCall;
	type BlockNumberProvider = System;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}

parameter_types! {
	pub const GraceStrikes: u32 = 10;
	pub const SocietyVotingPeriod: BlockNumber = 80 * HOURS;
	pub const ClaimPeriod: BlockNumber = 80 * HOURS;
	pub const PeriodSpend: Balance = 500 * DOLLARS;
	pub const MaxLockDuration: BlockNumber = 36 * 30 * DAYS;
	pub const ChallengePeriod: BlockNumber = 7 * DAYS;
	pub const MaxPayouts: u32 = 10;
	pub const MaxBids: u32 = 10;
	pub const SocietyPalletId: PalletId = PalletId(*b"py/socie");
}

impl plant_society::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = SocietyPalletId;
	type Currency = Balances;
	type Randomness = RandomnessCollectiveFlip;
	type GraceStrikes = GraceStrikes;
	type PeriodSpend = PeriodSpend;
	type VotingPeriod = SocietyVotingPeriod;
	type ClaimPeriod = ClaimPeriod;
	type MaxLockDuration = MaxLockDuration;
	type FounderSetOrigin =
		plant_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>;
	type ChallengePeriod = ChallengePeriod;
	type MaxPayouts = MaxPayouts;
	type MaxBids = MaxBids;
	type BlockNumberProvider = System;
	type WeightInfo = plant_society::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 100 * DOLLARS;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl plant_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = plant_vesting::weights::SubstrateWeight<Runtime>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
	// `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
	// highest number of schedules that encodes less than 2^10.
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

impl plant_mmr::Config for Runtime {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";
	type Hashing = Keccak256;
	type LeafData = plant_mmr::ParentNumberAndHash<Self>;
	type OnNewRoot = plant_beefy_mmr::DepositBeefyDigest<Runtime>;
	type BlockHashProvider = plant_mmr::DefaultBlockHashProvider<Runtime>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl plant_beefy_mmr::Config for Runtime {
	type LeafVersion = LeafVersion;
	type BeefyAuthorityToMerkleLeaf = plant_beefy_mmr::BeefyEcdsaToEthereum;
	type LeafExtra = Vec<u8>;
	type BeefyDataProvider = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const LotteryPalletId: PalletId = PalletId(*b"py/lotto");
	pub const MaxCalls: u32 = 10;
	pub const MaxGenerateRandom: u32 = 10;
}

impl plant_lottery::Config for Runtime {
	type PalletId = LotteryPalletId;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type Randomness = RandomnessCollectiveFlip;
	type RuntimeEvent = RuntimeEvent;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxCalls = MaxCalls;
	type ValidateCall = Lottery;
	type MaxGenerateRandom = MaxGenerateRandom;
	type WeightInfo = plant_lottery::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const AssetDeposit: Balance = 100 * DOLLARS;
	pub const ApprovalDeposit: Balance = 1 * DOLLARS;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * DOLLARS;
	pub const MetadataDepositPerByte: Balance = 1 * DOLLARS;
}

impl plant_assets::Config<Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<DOLLARS>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Holder = ();
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = plant_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

ord_parameter_types! {
	pub const AssetConversionOrigin: AccountId = AccountIdConversion::<AccountId>::into_account_truncating(&AssetConversionPalletId::get());
}

impl plant_assets::Config<Instance2> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<DOLLARS>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Holder = ();
	type Freezer = ();
	type Extra = ();
	type WeightInfo = plant_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
	pub const PoolSetupFee: Balance = 1 * DOLLARS; // should be more or equal to the existential deposit
	pub const MintMinLiquidity: Balance = 100;  // 100 is good enough when the main currency has 10-12 decimals.
	pub const LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
}

pub type NativeAndAssets =
	UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, AccountId>;

impl plant_asset_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type HigherPrecisionBalance = subsoil::core::U256;
	type AssetKind = NativeOrWithId<u32>;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = Chain<
		WithFirstAsset<
			Native,
			AccountId,
			NativeOrWithId<u32>,
			AccountIdConverter<AssetConversionPalletId, Self::PoolId>,
		>,
		Ascending<
			AccountId,
			NativeOrWithId<u32>,
			AccountIdConverter<AssetConversionPalletId, Self::PoolId>,
		>,
	>;
	type PoolAssetId = <Self as plant_assets::Config<Instance2>>::AssetId;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = PoolSetupFee;
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type PalletId = AssetConversionPalletId;
	type LPFee = ConstU32<3>; // means 0.3%
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type WeightInfo = plant_asset_conversion::weights::SubstrateWeight<Runtime>;
	type MaxSwapPathLength = ConstU32<4>;
	type MintMinLiquidity = MintMinLiquidity;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

pub type NativeAndAssetsFreezer =
	UnionOf<Balances, AssetsFreezer, NativeFromLeft, NativeOrWithId<u32>, AccountId>;

/// Benchmark Helper
#[cfg(feature = "runtime-benchmarks")]
pub struct AssetRewardsBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl plant_asset_rewards::benchmarking::BenchmarkHelper<NativeOrWithId<u32>>
	for AssetRewardsBenchmarkHelper
{
	fn staked_asset() -> NativeOrWithId<u32> {
		NativeOrWithId::<u32>::WithId(100)
	}
	fn reward_asset() -> NativeOrWithId<u32> {
		NativeOrWithId::<u32>::WithId(101)
	}
}

parameter_types! {
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"py/stkrd");
	pub const CreationHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::AssetRewards(plant_asset_rewards::HoldReason::PoolCreation);
	// 1 item, 135 bytes into the storage on pool creation.
	pub const StakePoolCreationDeposit: Balance = deposit(1, 135);
}

impl plant_asset_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type AssetId = NativeOrWithId<u32>;
	type Balance = Balance;
	type Assets = NativeAndAssets;
	type PalletId = StakingRewardsPalletId;
	type CreatePoolOrigin = EnsureSigned<AccountId>;
	type WeightInfo = ();
	type AssetsFreezer = NativeAndAssetsFreezer;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		CreationHoldReason,
		ConstantStoragePrice<StakePoolCreationDeposit, Balance>,
	>;
	type BlockNumberProvider = topsoil_system::Pallet<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRewardsBenchmarkHelper;
}

impl plant_asset_conversion_ops::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PriorAccountIdConverter = plant_asset_conversion::AccountIdConverterNoSeed<(
		NativeOrWithId<u32>,
		NativeOrWithId<u32>,
	)>;
	type AssetsRefund = <Runtime as plant_asset_conversion::Config>::Assets;
	type PoolAssetsRefund = <Runtime as plant_asset_conversion::Config>::PoolAssets;
	type PoolAssetsTeam = <Runtime as plant_asset_conversion::Config>::PoolAssets;
	type DepositAsset = Balances;
	type WeightInfo = plant_asset_conversion_ops::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const QueueCount: u32 = 300;
	pub const MaxQueueLen: u32 = 1000;
	pub const FifoQueueLen: u32 = 500;
	pub const NisBasePeriod: BlockNumber = 30 * DAYS;
	pub const MinBid: Balance = 100 * DOLLARS;
	pub const MinReceipt: Perquintill = Perquintill::from_percent(1);
	pub const IntakePeriod: BlockNumber = 10;
	pub MaxIntakeWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 10;
	pub const ThawThrottle: (Perquintill, BlockNumber) = (Perquintill::from_percent(25), 5);
	pub Target: Perquintill = Perquintill::zero();
	pub const NisPalletId: PalletId = PalletId(*b"py/nis  ");
}

impl plant_nis::Config for Runtime {
	type WeightInfo = plant_nis::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type FundOrigin = topsoil_system::EnsureSigned<AccountId>;
	type Counterpart = ItemOf<Assets, ConstU32<9u32>, AccountId>;
	type CounterpartAmount = WithMaximumOf<ConstU128<21_000_000_000_000_000_000u128>>;
	type Deficit = ();
	type IgnoredIssuance = ();
	type Target = Target;
	type PalletId = NisPalletId;
	type QueueCount = QueueCount;
	type MaxQueueLen = MaxQueueLen;
	type FifoQueueLen = FifoQueueLen;
	type BasePeriod = NisBasePeriod;
	type MinBid = MinBid;
	type MinReceipt = MinReceipt;
	type IntakePeriod = IntakePeriod;
	type MaxIntakeWeight = MaxIntakeWeight;
	type ThawThrottle = ThawThrottle;
	type RuntimeHoldReason = RuntimeHoldReason;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkSetup = SetupAsset;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct SetupAsset;
#[cfg(feature = "runtime-benchmarks")]
impl plant_nis::BenchmarkSetup for SetupAsset {
	fn create_counterpart_asset() {
		let owner = AccountId::from([0u8; 32]);
		// this may or may not fail depending on if the chain spec or runtime genesis is used.
		let _ = Assets::force_create(
			RuntimeOrigin::root(),
			9u32.into(),
			subsoil::runtime::MultiAddress::Id(owner),
			true,
			1,
		);
	}
}

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * DOLLARS;
	pub const ItemDeposit: Balance = 1 * DOLLARS;
	pub const ApprovalsLimit: u32 = 20;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
}

impl plant_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = topsoil_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = ConstU32<128>;
	type KeyLimit = ConstU32<32>;
	type ValueLimit = ConstU32<64>;
	type WeightInfo = plant_uniques::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}

parameter_types! {
	pub const Budget: Balance = 10_000 * DOLLARS;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}

pub struct SalaryForRank;
impl GetSalary<u16, AccountId, Balance> for SalaryForRank {
	fn get_salary(a: u16, _: &AccountId) -> Balance {
		Balance::from(a) * 1000 * DOLLARS
	}
}

impl plant_salary::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
	type Members = RankedCollective;
	type Salary = SalaryForRank;
	type RegistrationPeriod = ConstU32<200>;
	type PayoutPeriod = ConstU32<200>;
	type Budget = Budget;
}

impl plant_core_fellowship::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Members = RankedCollective;
	type Balance = Balance;
	type ParamsOrigin = topsoil_system::EnsureRoot<AccountId>;
	type InductOrigin = plant_core_fellowship::EnsureInducted<Runtime, (), 1>;
	type ApproveOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type FastPromoteOrigin = Self::PromoteOrigin;
	type EvidenceSize = ConstU32<16_384>;
	type MaxRank = ConstU16<9>;
}

parameter_types! {
	pub const NftFractionalizationPalletId: PalletId = PalletId(*b"fraction");
	pub NewAssetSymbol: BoundedVec<u8, StringLimit> = (*b"FRAC").to_vec().try_into().unwrap();
	pub NewAssetName: BoundedVec<u8, StringLimit> = (*b"Frac").to_vec().try_into().unwrap();
}

impl plant_nft_fractionalization::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Deposit = AssetDeposit;
	type Currency = Balances;
	type NewAssetSymbol = NewAssetSymbol;
	type NewAssetName = NewAssetName;
	type StringLimit = StringLimit;
	type NftCollectionId = <Self as plant_nfts::Config>::CollectionId;
	type NftId = <Self as plant_nfts::Config>::ItemId;
	type AssetBalance = <Self as plant_balances::Config>::Balance;
	type AssetId = <Self as plant_assets::Config<Instance1>>::AssetId;
	type Assets = Assets;
	type Nfts = Nfts;
	type PalletId = NftFractionalizationPalletId;
	type WeightInfo = plant_nft_fractionalization::weights::SubstrateWeight<Runtime>;
	type RuntimeHoldReason = RuntimeHoldReason;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
	pub const MaxAttributesPerCall: u32 = 10;
}

impl plant_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = topsoil_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = ConstU32<256>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<256>;
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as traits::Verify>::Signer;
	type WeightInfo = plant_nfts::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
	type BlockNumberProvider = topsoil_system::Pallet<Runtime>;
}

impl plant_transaction_storage::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeCall = RuntimeCall;
	type FeeDestination = ();
	type WeightInfo = plant_transaction_storage::weights::SubstrateWeight<Runtime>;
	type MaxBlockTransactions =
		ConstU32<{ plant_transaction_storage::DEFAULT_MAX_BLOCK_TRANSACTIONS }>;
	type MaxTransactionSize =
		ConstU32<{ plant_transaction_storage::DEFAULT_MAX_TRANSACTION_SIZE }>;
}

impl plant_verify_signature::Config for Runtime {
	type Signature = MultiSignature;
	type AccountIdentifier = MultiSigner;
	type WeightInfo = plant_verify_signature::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl plant_whitelist::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WhitelistOrigin = EnsureRoot<AccountId>;
	type DispatchWhitelistedOrigin = EnsureRoot<AccountId>;
	type Preimages = Preimage;
	type WeightInfo = plant_whitelist::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MigrationSignedDepositPerItem: Balance = 1 * CENTS;
	pub const MigrationSignedDepositBase: Balance = 20 * DOLLARS;
	pub const MigrationMaxKeyLen: u32 = 512;
}

impl plant_state_trie_migration::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ControlOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxKeyLen = MigrationMaxKeyLen;
	type SignedDepositPerItem = MigrationSignedDepositPerItem;
	type SignedDepositBase = MigrationSignedDepositBase;
	// Warning: this is not advised, as it might allow the chain to be temporarily DOS-ed.
	// Preferably, if the chain's governance/maintenance team is planning on using a specific
	// account for the migration, put it here to make sure only that account can trigger the signed
	// migrations.
	type SignedFilter = EnsureSigned<Self::AccountId>;
	type WeightInfo = ();
}

const ALLIANCE_MOTION_DURATION_IN_BLOCKS: BlockNumber = 5 * DAYS;

parameter_types! {
	pub const AllianceMotionDuration: BlockNumber = ALLIANCE_MOTION_DURATION_IN_BLOCKS;
	pub const AllianceMaxProposals: u32 = 100;
	pub const AllianceMaxMembers: u32 = 100;
}

type AllianceCollective = plant_collective::Instance3;
impl plant_collective::Config<AllianceCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = AllianceMotionDuration;
	type MaxProposals = AllianceMaxProposals;
	type MaxMembers = AllianceMaxMembers;
	type DefaultVote = plant_collective::PrimeDefaultVote;
	type WeightInfo = plant_collective::weights::SubstrateWeight<Runtime>;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
}

parameter_types! {
	pub const MaxFellows: u32 = AllianceMaxMembers::get();
	pub const MaxAllies: u32 = 100;
	pub const AllyDeposit: Balance = 10 * DOLLARS;
	pub const RetirementPeriod: BlockNumber = ALLIANCE_MOTION_DURATION_IN_BLOCKS + (1 * DAYS);
}

impl plant_alliance::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Proposal = RuntimeCall;
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionMoreThan<AccountId, AllianceCollective, 2, 3>,
	>;
	type MembershipManager = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionMoreThan<AccountId, AllianceCollective, 2, 3>,
	>;
	type AnnouncementOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		plant_collective::EnsureProportionMoreThan<AccountId, AllianceCollective, 2, 3>,
	>;
	type Currency = Balances;
	type Slashed = Treasury;
	type InitializeMembers = AllianceMotion;
	type MembershipChanged = AllianceMotion;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type IdentityVerifier = AllianceIdentityVerifier;
	#[cfg(feature = "runtime-benchmarks")]
	type IdentityVerifier = ();
	type ProposalProvider = AllianceProposalProvider;
	type MaxProposals = AllianceMaxProposals;
	type MaxFellows = MaxFellows;
	type MaxAllies = MaxAllies;
	type MaxUnscrupulousItems = ConstU32<100>;
	type MaxWebsiteUrlLength = ConstU32<255>;
	type MaxAnnouncementsCount = ConstU32<100>;
	type MaxMembersCount = AllianceMaxMembers;
	type AllyDeposit = AllyDeposit;
	type WeightInfo = plant_alliance::weights::SubstrateWeight<Runtime>;
	type RetirementPeriod = RetirementPeriod;
}

impl topsoil_benchmarking_pallet_pov::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub StatementCost: Balance = 1 * DOLLARS;
	pub StatementByteCost: Balance = 100 * MILLICENTS;
	pub const MinAllowedStatements: u32 = 4;
	pub const MaxAllowedStatements: u32 = 10;
	pub const MinAllowedBytes: u32 = 1024;
	pub const MaxAllowedBytes: u32 = 4096;
}

impl plant_statement::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type StatementCost = StatementCost;
	type ByteCost = StatementByteCost;
	type MinAllowedStatements = MinAllowedStatements;
	type MaxAllowedStatements = MaxAllowedStatements;
	type MinAllowedBytes = MinAllowedBytes;
	type MaxAllowedBytes = MaxAllowedBytes;
}

parameter_types! {
	pub const BrokerPalletId: PalletId = PalletId(*b"py/broke");
	pub const MinimumCreditPurchase: Balance =  100 * MILLICENTS;
}

pub struct IntoAuthor;
impl OnUnbalanced<Credit<AccountId, Balances>> for IntoAuthor {
	fn on_nonzero_unbalanced(credit: Credit<AccountId, Balances>) {
		if let Some(author) = Authorship::author() {
			let _ = <Balances as Balanced<_>>::resolve(&author, credit);
		}
	}
}

pub struct CoretimeProvider;
impl CoretimeInterface for CoretimeProvider {
	type AccountId = AccountId;
	type Balance = Balance;
	type RelayChainBlockNumberProvider = System;
	fn request_core_count(_count: CoreIndex) {}
	fn request_revenue_info_at(_when: u32) {}
	fn credit_account(_who: Self::AccountId, _amount: Self::Balance) {}
	fn assign_core(
		_core: CoreIndex,
		_begin: u32,
		_assignment: Vec<(CoreAssignment, PartsOf57600)>,
		_end_hint: Option<u32>,
	) {
	}
}

pub struct SovereignAccountOf;
// Dummy implementation which converts `TaskId` to `AccountId`.
impl MaybeConvert<TaskId, AccountId> for SovereignAccountOf {
	fn maybe_convert(task: TaskId) -> Option<AccountId> {
		let mut account: [u8; 32] = [0; 32];
		account[..4].copy_from_slice(&task.to_le_bytes());
		Some(account.into())
	}
}
impl plant_broker::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type OnRevenue = IntoAuthor;
	type TimeslicePeriod = ConstU32<2>;
	type MaxLeasedCores = ConstU32<5>;
	type MaxReservedCores = ConstU32<5>;
	type Coretime = CoretimeProvider;
	type ConvertBalance = traits::Identity;
	type WeightInfo = ();
	type PalletId = BrokerPalletId;
	type AdminOrigin = EnsureRoot<AccountId>;
	type SovereignAccountOf = SovereignAccountOf;
	type MaxAutoRenewals = ConstU32<10>;
	type PriceAdapter = plant_broker::CenterTargetPrice<Balance>;
	type MinimumCreditPurchase = MinimumCreditPurchase;
}

parameter_types! {
	pub const MixnetNumCoverToCurrentBlocks: BlockNumber = 3;
	pub const MixnetNumRequestsToCurrentBlocks: BlockNumber = 3;
	pub const MixnetNumCoverToPrevBlocks: BlockNumber = 3;
	pub const MixnetNumRegisterStartSlackBlocks: BlockNumber = 3;
	pub const MixnetNumRegisterEndSlackBlocks: BlockNumber = 3;
	pub const MixnetRegistrationPriority: TransactionPriority = ImOnlineUnsignedPriority::get() - 1;
}

impl plant_mixnet::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
	type MaxExternalAddressSize = ConstU32<128>;
	type MaxExternalAddressesPerMixnode = ConstU32<16>;
	type NextSessionRotation = Babe;
	type NumCoverToCurrentBlocks = MixnetNumCoverToCurrentBlocks;
	type NumRequestsToCurrentBlocks = MixnetNumRequestsToCurrentBlocks;
	type NumCoverToPrevBlocks = MixnetNumCoverToPrevBlocks;
	type NumRegisterStartSlackBlocks = MixnetNumRegisterStartSlackBlocks;
	type NumRegisterEndSlackBlocks = MixnetNumRegisterEndSlackBlocks;
	type RegistrationPriority = MixnetRegistrationPriority;
	type MinMixnodes = ConstU32<7>; // Low to allow small testing networks
}

/// Dynamic parameters that can be changed at runtime through the
/// `plant_parameters::set_parameter`.
#[dynamic_params(RuntimeParameters, plant_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
	use super::*;

	#[dynamic_pallet_params]
	#[codec(index = 0)]
	pub mod storage {
		/// Configures the base deposit of storing some data.
		#[codec(index = 0)]
		pub static BaseDeposit: Balance = 1 * DOLLARS;

		/// Configures the per-byte deposit of storing some data.
		#[codec(index = 1)]
		pub static ByteDeposit: Balance = 1 * CENTS;
	}

	#[dynamic_pallet_params]
	#[codec(index = 1)]
	pub mod referenda {
		/// The configuration for the tracks
		#[codec(index = 0)]
		pub static Tracks: BoundedVec<
			plant_referenda::Track<u16, Balance, BlockNumber>,
			ConstU32<100>,
		> = BoundedVec::truncate_from(vec![plant_referenda::Track {
			id: 0u16,
			info: plant_referenda::TrackInfo {
				name: s("root"),
				max_deciding: 1,
				decision_deposit: 10,
				prepare_period: 4,
				decision_period: 4,
				confirm_period: 2,
				min_enactment_period: 4,
				min_approval: plant_referenda::Curve::LinearDecreasing {
					length: Perbill::from_percent(100),
					floor: Perbill::from_percent(50),
					ceil: Perbill::from_percent(100),
				},
				min_support: plant_referenda::Curve::LinearDecreasing {
					length: Perbill::from_percent(100),
					floor: Perbill::from_percent(0),
					ceil: Perbill::from_percent(100),
				},
			},
		}]);

		/// A list mapping every origin with a track Id
		#[codec(index = 1)]
		pub static Origins: BoundedVec<(OriginCaller, u16), ConstU32<100>> =
			BoundedVec::truncate_from(vec![(
				OriginCaller::system(topsoil_system::RawOrigin::Root),
				0,
			)]);
	}
}

#[cfg(feature = "runtime-benchmarks")]
impl Default for RuntimeParameters {
	fn default() -> Self {
		RuntimeParameters::Storage(dynamic_params::storage::Parameters::BaseDeposit(
			dynamic_params::storage::BaseDeposit,
			Some(1 * DOLLARS),
		))
	}
}

pub struct DynamicParametersManagerOrigin;
impl EnsureOriginWithArg<RuntimeOrigin, RuntimeParametersKey> for DynamicParametersManagerOrigin {
	type Success = ();

	fn try_origin(
		origin: RuntimeOrigin,
		key: &RuntimeParametersKey,
	) -> Result<Self::Success, RuntimeOrigin> {
		match key {
			RuntimeParametersKey::Storage(_) => {
				topsoil_system::ensure_root(origin.clone()).map_err(|_| origin)?;
				return Ok(());
			},
			RuntimeParametersKey::Referenda(_) => {
				topsoil_system::ensure_root(origin.clone()).map_err(|_| origin)?;
				return Ok(());
			},
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(_key: &RuntimeParametersKey) -> Result<RuntimeOrigin, ()> {
		Ok(RuntimeOrigin::root())
	}
}

impl plant_parameters::Config for Runtime {
	type RuntimeParameters = RuntimeParameters;
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = DynamicParametersManagerOrigin;
	type WeightInfo = ();
}

pub type MetaTxExtension = (
	plant_verify_signature::VerifySignature<Runtime>,
	plant_meta_tx::MetaTxMarker<Runtime>,
	topsoil_system::CheckNonZeroSender<Runtime>,
	topsoil_system::CheckSpecVersion<Runtime>,
	topsoil_system::CheckTxVersion<Runtime>,
	topsoil_system::CheckGenesis<Runtime>,
	topsoil_system::CheckEra<Runtime>,
	topsoil_system::CheckNonce<Runtime>,
	plant_metadata_hash_extension::CheckMetadataHash<Runtime>,
);

impl plant_meta_tx::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type Extension = MetaTxExtension;
	#[cfg(feature = "runtime-benchmarks")]
	type Extension = plant_meta_tx::WeightlessExtension<Runtime>;
}

#[topsoil_support::runtime]
mod runtime {
	use super::*;

	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	#[runtime::pallet_index(0)]
	pub type System = topsoil_system::Pallet<Runtime>;

	#[runtime::pallet_index(1)]
	pub type Utility = plant_utility::Pallet<Runtime>;

	#[runtime::pallet_index(2)]
	pub type Babe = plant_babe::Pallet<Runtime>;

	#[runtime::pallet_index(3)]
	pub type Timestamp = plant_timestamp::Pallet<Runtime>;

	// Authorship must be before session in order to note author in the correct session and era
	// for im-online and staking.
	#[runtime::pallet_index(4)]
	pub type Authorship = plant_authorship::Pallet<Runtime>;

	#[runtime::pallet_index(5)]
	pub type Indices = plant_indices::Pallet<Runtime>;

	#[runtime::pallet_index(6)]
	pub type Balances = plant_balances::Pallet<Runtime>;

	#[runtime::pallet_index(7)]
	pub type TransactionPayment = plant_transaction_payment::Pallet<Runtime>;

	#[runtime::pallet_index(9)]
	pub type AssetConversionTxPayment = plant_asset_conversion_tx_payment::Pallet<Runtime>;

	#[runtime::pallet_index(10)]
	pub type ElectionProviderMultiPhase = plant_election_provider_multi_phase::Pallet<Runtime>;

	#[runtime::pallet_index(11)]
	pub type Staking = plant_staking::Pallet<Runtime>;

	#[runtime::pallet_index(12)]
	pub type Session = plant_session::Pallet<Runtime>;

	#[runtime::pallet_index(13)]
	pub type Democracy = plant_democracy::Pallet<Runtime>;

	#[runtime::pallet_index(14)]
	pub type Council = plant_collective::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(15)]
	pub type TechnicalCommittee = plant_collective::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(16)]
	pub type Elections = plant_elections_phragmen::Pallet<Runtime>;

	#[runtime::pallet_index(17)]
	pub type TechnicalMembership = plant_membership::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(18)]
	pub type Grandpa = plant_grandpa::Pallet<Runtime>;

	#[runtime::pallet_index(19)]
	pub type Treasury = plant_treasury::Pallet<Runtime>;

	#[runtime::pallet_index(20)]
	pub type AssetRate = plant_asset_rate::Pallet<Runtime>;

	#[runtime::pallet_index(22)]
	pub type Sudo = plant_sudo::Pallet<Runtime>;

	#[runtime::pallet_index(23)]
	pub type ImOnline = plant_im_online::Pallet<Runtime>;

	#[runtime::pallet_index(24)]
	pub type AuthorityDiscovery = plant_authority_discovery::Pallet<Runtime>;

	#[runtime::pallet_index(25)]
	pub type Offences = plant_offences::Pallet<Runtime>;

	#[runtime::pallet_index(26)]
	pub type Historical = pallet_session_historical::Pallet<Runtime>;

	#[runtime::pallet_index(27)]
	pub type RandomnessCollectiveFlip =
		plant_insecure_randomness_collective_flip::Pallet<Runtime>;

	#[runtime::pallet_index(28)]
	pub type Identity = plant_identity::Pallet<Runtime>;

	#[runtime::pallet_index(29)]
	pub type Society = plant_society::Pallet<Runtime>;

	#[runtime::pallet_index(30)]
	pub type Recovery = plant_recovery::Pallet<Runtime>;

	#[runtime::pallet_index(31)]
	pub type Vesting = plant_vesting::Pallet<Runtime>;

	#[runtime::pallet_index(32)]
	pub type Scheduler = plant_scheduler::Pallet<Runtime>;

	#[runtime::pallet_index(33)]
	pub type Glutton = plant_glutton::Pallet<Runtime>;

	#[runtime::pallet_index(34)]
	pub type Preimage = plant_preimage::Pallet<Runtime>;

	#[runtime::pallet_index(35)]
	pub type Proxy = plant_proxy::Pallet<Runtime>;

	#[runtime::pallet_index(36)]
	pub type Multisig = plant_multisig::Pallet<Runtime>;

	#[runtime::pallet_index(37)]
	pub type Bounties = plant_bounties::Pallet<Runtime>;

	#[runtime::pallet_index(38)]
	pub type Tips = plant_tips::Pallet<Runtime>;

	#[runtime::pallet_index(39)]
	pub type Assets = plant_assets::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(40)]
	pub type PoolAssets = plant_assets::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(41)]
	pub type Beefy = plant_beefy::Pallet<Runtime>;

	// MMR leaf construction must be after session in order to have a leaf's next_auth_set
	// refer to block<N>. See issue polkadot-fellows/runtimes#160 for details.
	#[runtime::pallet_index(42)]
	pub type Mmr = plant_mmr::Pallet<Runtime>;

	#[runtime::pallet_index(43)]
	pub type MmrLeaf = plant_beefy_mmr::Pallet<Runtime>;

	#[runtime::pallet_index(44)]
	pub type Lottery = plant_lottery::Pallet<Runtime>;

	#[runtime::pallet_index(45)]
	pub type Nis = plant_nis::Pallet<Runtime>;

	#[runtime::pallet_index(46)]
	pub type Uniques = plant_uniques::Pallet<Runtime>;

	#[runtime::pallet_index(47)]
	pub type Nfts = plant_nfts::Pallet<Runtime>;

	#[runtime::pallet_index(48)]
	pub type NftFractionalization = plant_nft_fractionalization::Pallet<Runtime>;

	#[runtime::pallet_index(49)]
	pub type Salary = plant_salary::Pallet<Runtime>;

	#[runtime::pallet_index(50)]
	pub type CoreFellowship = plant_core_fellowship::Pallet<Runtime>;

	#[runtime::pallet_index(51)]
	pub type TransactionStorage = plant_transaction_storage::Pallet<Runtime>;

	#[runtime::pallet_index(52)]
	pub type VoterList = plant_bags_list::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(53)]
	pub type StateTrieMigration = plant_state_trie_migration::Pallet<Runtime>;

	#[runtime::pallet_index(54)]
	pub type ChildBounties = plant_child_bounties::Pallet<Runtime>;

	#[runtime::pallet_index(55)]
	pub type Referenda = plant_referenda::Pallet<Runtime>;

	#[runtime::pallet_index(56)]
	pub type Remark = plant_remark::Pallet<Runtime>;

	#[runtime::pallet_index(57)]
	pub type RootTesting = plant_test_root::Pallet<Runtime>;

	#[runtime::pallet_index(58)]
	pub type ConvictionVoting = plant_conviction_voting::Pallet<Runtime>;

	#[runtime::pallet_index(59)]
	pub type Whitelist = plant_whitelist::Pallet<Runtime>;

	#[runtime::pallet_index(60)]
	pub type AllianceMotion = plant_collective::Pallet<Runtime, Instance3>;

	#[runtime::pallet_index(61)]
	pub type Alliance = plant_alliance::Pallet<Runtime>;

	#[runtime::pallet_index(62)]
	pub type NominationPools = plant_nomination_pools::Pallet<Runtime>;

	#[runtime::pallet_index(63)]
	pub type RankedPolls = plant_referenda::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(64)]
	pub type RankedCollective = plant_ranked_collective::Pallet<Runtime>;

	#[runtime::pallet_index(65)]
	pub type AssetConversion = plant_asset_conversion::Pallet<Runtime>;

	#[runtime::pallet_index(66)]
	pub type FastUnstake = plant_fast_unstake::Pallet<Runtime>;

	#[runtime::pallet_index(67)]
	pub type MessageQueue = plant_message_queue::Pallet<Runtime>;

	#[runtime::pallet_index(68)]
	pub type Pov = topsoil_benchmarking_pallet_pov::Pallet<Runtime>;

	#[runtime::pallet_index(69)]
	pub type TxPause = plant_tx_pause::Pallet<Runtime>;

	#[runtime::pallet_index(70)]
	pub type SafeMode = plant_safe_mode::Pallet<Runtime>;

	#[runtime::pallet_index(71)]
	pub type Statement = plant_statement::Pallet<Runtime>;

	#[runtime::pallet_index(73)]
	pub type Broker = plant_broker::Pallet<Runtime>;

	#[runtime::pallet_index(75)]
	pub type Mixnet = plant_mixnet::Pallet<Runtime>;

	#[runtime::pallet_index(76)]
	pub type Parameters = plant_parameters::Pallet<Runtime>;

	#[runtime::pallet_index(79)]
	pub type AssetConversionMigration = plant_asset_conversion_ops::Pallet<Runtime>;

	#[runtime::pallet_index(81)]
	pub type VerifySignature = plant_verify_signature::Pallet<Runtime>;

	#[runtime::pallet_index(82)]
	pub type DelegatedStaking = plant_delegated_staking::Pallet<Runtime>;

	#[runtime::pallet_index(83)]
	pub type AssetRewards = plant_asset_rewards::Pallet<Runtime>;

	#[runtime::pallet_index(84)]
	pub type AssetsFreezer = plant_assets_freezer::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(85)]
	pub type Oracle = plant_oracle::Pallet<Runtime>;

	#[runtime::pallet_index(89)]
	pub type MetaTx = plant_meta_tx::Pallet<Runtime>;

	#[runtime::pallet_index(90)]
	pub type MultiAssetBounties = plant_multi_asset_bounties::Pallet<Runtime>;
}

/// The address format for describing accounts.
pub type Address = subsoil::runtime::MultiAddress<AccountId, AccountIndex>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The TransactionExtension to the basic transaction logic.
///
/// When you change this, you **MUST** modify [`sign`] in `bin/node/testing/src/keyring.rs`!
///
/// [`sign`]: <../../testing/src/keyring.rs.html>
pub type TxExtension = (
	topsoil_system::AuthorizeCall<Runtime>,
	topsoil_system::CheckNonZeroSender<Runtime>,
	topsoil_system::CheckSpecVersion<Runtime>,
	topsoil_system::CheckTxVersion<Runtime>,
	topsoil_system::CheckGenesis<Runtime>,
	topsoil_system::CheckEra<Runtime>,
	topsoil_system::CheckNonce<Runtime>,
	topsoil_system::CheckWeight<Runtime>,
	plant_asset_conversion_tx_payment::ChargeAssetTxPayment<Runtime>,
	plant_metadata_hash_extension::CheckMetadataHash<Runtime>,
	topsoil_system::WeightReclaim<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;
/// Unchecked signature payload type as expected by this runtime.
pub type UncheckedSignaturePayload =
	generic::UncheckedSignaturePayload<Address, Signature, TxExtension>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, TxExtension>;
/// Executive: handles dispatch to the various modules.
pub type Executive = topsoil_executive::Executive<
	Runtime,
	Block,
	topsoil_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

// We don't have a limit in the Relay Chain.
const IDENTITY_MIGRATION_KEY_LIMIT: u64 = u64::MAX;

// All migrations executed on runtime upgrade as a nested tuple of types implementing
// `OnRuntimeUpgrade`. Note: These are examples and do not need to be run directly
// after the genesis block.
type Migrations = (
	plant_nomination_pools::migration::versioned::V6ToV7<Runtime>,
	plant_alliance::migration::Migration<Runtime>,
	plant_identity::migration::versioned::V0ToV1<Runtime, IDENTITY_MIGRATION_KEY_LIMIT>,
);

parameter_types! {
	pub const BeefySetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl plant_beefy::Config for Runtime {
	type BeefyId = BeefyId;
	type MaxAuthorities = MaxAuthorities;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = BeefySetIdSessionEntries;
	type OnNewValidatorSet = MmrLeaf;
	type AncestryHelper = MmrLeaf;
	type WeightInfo = ();
	type KeyOwnerProof = subsoil::session::MembershipProof;
	type EquivocationReportSystem =
		plant_beefy::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

parameter_types! {
	pub const OracleMaxHasDispatchedSize: u32 = 20;
	pub const RootOperatorAccountId: AccountId = AccountId::new([0xffu8; 32]);

	pub const OracleMaxFeedValues: u32 = 10;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct OracleBenchmarkingHelper;

#[cfg(feature = "runtime-benchmarks")]
impl plant_oracle::BenchmarkHelper<u32, u128, OracleMaxFeedValues> for OracleBenchmarkingHelper {
	fn get_currency_id_value_pairs() -> BoundedVec<(u32, u128), OracleMaxFeedValues> {
		use rand::{distributions::Uniform, prelude::*};

		// Use seeded RNG like in contracts benchmarking
		let mut rng = rand_pcg::Pcg32::seed_from_u64(0x1234567890ABCDEF);
		let max_values = OracleMaxFeedValues::get() as usize;

		// Generate random pairs like in election-provider-multi-phase
		let currency_range = Uniform::new_inclusive(1, 1000);
		let value_range = Uniform::new_inclusive(1000, 1_000_000);

		let pairs: Vec<(u32, u128)> = (0..max_values)
			.map(|_| {
				let currency_id = rng.sample(currency_range);
				let value = rng.sample(value_range);
				(currency_id, value)
			})
			.collect();

		// Use try_from pattern like in core-fellowship and broker
		BoundedVec::try_from(pairs).unwrap_or_default()
	}
}

parameter_types! {
	pub const OraclePalletId: PalletId = PalletId(*b"py/oracl");
}

impl plant_oracle::Config for Runtime {
	type OnNewData = ();
	type CombineData = plant_oracle::DefaultCombineData<Self, ConstU32<5>, ConstU64<3600>>;
	type Time = Timestamp;
	type OracleKey = u32;
	type OracleValue = u128;
	type PalletId = OraclePalletId;
	type Members = TechnicalMembership;
	type WeightInfo = ();
	type MaxHasDispatchedSize = OracleMaxHasDispatchedSize;
	type MaxFeedValues = OracleMaxFeedValues;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = OracleBenchmarkingHelper;
}

/// MMR helper types.
mod mmr {
	use super::*;
	pub use plant_mmr::primitives::*;

	pub type Leaf = <<Runtime as plant_mmr::Config>::LeafData as LeafDataProvider>::LeafData;
	pub type Hash = <Hashing as subsoil::runtime::traits::Hash>::Output;
	pub type Hashing = <Runtime as plant_mmr::Config>::Hashing;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct AssetConversionTxHelper;

#[cfg(feature = "runtime-benchmarks")]
impl
	plant_asset_conversion_tx_payment::BenchmarkHelperTrait<
		AccountId,
		NativeOrWithId<u32>,
		NativeOrWithId<u32>,
	> for AssetConversionTxHelper
{
	fn create_asset_id_parameter(seed: u32) -> (NativeOrWithId<u32>, NativeOrWithId<u32>) {
		(NativeOrWithId::WithId(seed), NativeOrWithId::WithId(seed))
	}

	fn setup_balances_and_pool(asset_id: NativeOrWithId<u32>, account: AccountId) {
		use topsoil_support::{assert_ok, traits::fungibles::Mutate};
		let NativeOrWithId::WithId(asset_idx) = asset_id.clone() else { unimplemented!() };
		assert_ok!(Assets::force_create(
			RuntimeOrigin::root(),
			asset_idx.into(),
			account.clone().into(), // owner
			true,                   // is_sufficient
			1,
		));

		let lp_provider = account.clone();
		let _ = Balances::deposit_creating(&lp_provider, ((u64::MAX as u128) * 100).into());
		assert_ok!(Assets::mint_into(
			asset_idx.into(),
			&lp_provider,
			((u64::MAX as u128) * 100).into()
		));

		let token_native = alloc::boxed::Box::new(NativeOrWithId::Native);
		let token_second = alloc::boxed::Box::new(asset_id);

		assert_ok!(AssetConversion::create_pool(
			RuntimeOrigin::signed(lp_provider.clone()),
			token_native.clone(),
			token_second.clone()
		));

		assert_ok!(AssetConversion::add_liquidity(
			RuntimeOrigin::signed(lp_provider.clone()),
			token_native,
			token_second,
			u64::MAX.into(), // 1 desired
			u64::MAX.into(), // 2 desired
			1,               // 1 min
			1,               // 2 min
			lp_provider,
		));
	}
}

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	topsoil_benchmarking::define_benchmarks!(
		[topsoil_benchmarking, BaselineBench::<Runtime>]
		[topsoil_benchmarking_pallet_pov, Pov]
		[plant_alliance, Alliance]
		[plant_assets, Assets]
		[plant_babe, Babe]
		[plant_bags_list, VoterList]
		[plant_balances, Balances]
		[plant_beefy_mmr, MmrLeaf]
		[plant_bounties, Bounties]
		[plant_broker, Broker]
		[plant_child_bounties, ChildBounties]
		[plant_collective, Council]
		[plant_conviction_voting, ConvictionVoting]
		[plant_core_fellowship, CoreFellowship]
		[plant_democracy, Democracy]
		[plant_asset_conversion, AssetConversion]
		[plant_asset_rewards, AssetRewards]
		[plant_asset_conversion_tx_payment, AssetConversionTxPayment]
		[plant_transaction_payment, TransactionPayment]
		[plant_election_provider_multi_phase, ElectionProviderMultiPhase]
		[plant_election_provider, EPSBench::<Runtime>]
		[plant_elections_phragmen, Elections]
		[plant_fast_unstake, FastUnstake]
		[plant_nis, Nis]
		[plant_parameters, Parameters]
		[plant_grandpa, Grandpa]
		[plant_identity, Identity]
		[plant_im_online, ImOnline]
		[plant_indices, Indices]
		[plant_lottery, Lottery]
		[plant_membership, TechnicalMembership]
		[plant_message_queue, MessageQueue]
		[plant_mmr, Mmr]
		[plant_multi_asset_bounties, MultiAssetBounties]
		[plant_multisig, Multisig]
		[plant_nomination_pools, NominationPoolsBench::<Runtime>]
		[plant_offences, OffencesBench::<Runtime>]
		[plant_oracle, Oracle]
		[plant_preimage, Preimage]
		[plant_proxy, Proxy]
		[plant_ranked_collective, RankedCollective]
		[plant_referenda, Referenda]
		[plant_recovery, Recovery]
		[plant_remark, Remark]
		[plant_salary, Salary]
		[plant_scheduler, Scheduler]
		[plant_glutton, Glutton]
		[plant_session, SessionBench::<Runtime>]
		[plant_society, Society]
		[plant_staking, Staking]
		[plant_state_trie_migration, StateTrieMigration]
		[plant_sudo, Sudo]
		[topsoil_system, SystemBench::<Runtime>]
		[frame_system_extensions, SystemExtensionsBench::<Runtime>]
		[plant_timestamp, Timestamp]
		[plant_tips, Tips]
		[plant_transaction_storage, TransactionStorage]
		[plant_treasury, Treasury]
		[plant_asset_rate, AssetRate]
		[plant_uniques, Uniques]
		[plant_nfts, Nfts]
		[plant_nft_fractionalization, NftFractionalization]
		[plant_utility, Utility]
		[plant_vesting, Vesting]
		[plant_whitelist, Whitelist]
		[plant_tx_pause, TxPause]
		[plant_safe_mode, SafeMode]
		[plant_asset_conversion_ops, AssetConversionMigration]
		[plant_verify_signature, VerifySignature]
		[plant_meta_tx, MetaTx]
	);
}

#[cfg(feature = "runtime-benchmarks")]
type SessionMembershipBenchmarkSetup =
	((subsoil::runtime::KeyTypeId, Vec<u8>), subsoil::session::MembershipProof);

#[cfg(feature = "runtime-benchmarks")]
type RuntimeIdentificationTuple = plant_session::historical::IdentificationTuple<Runtime>;

#[cfg(feature = "runtime-benchmarks")]
struct BenchmarkOffender {
	controller: AccountId,
	#[allow(dead_code)]
	stash: AccountId,
	#[allow(dead_code)]
	nominator_stashes: Vec<AccountId>,
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_generate_session_keys_and_proof(owner: AccountId) -> (SessionKeys, Vec<u8>) {
	let keys = SessionKeys::generate(&owner.encode(), None);
	(keys.keys, keys.proof.encode())
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_setup_session_controller() -> Result<AccountId, &'static str> {
	let max_nominations = plant_staking::MaxNominationsOf::<Runtime>::get();
	let (stash, _) = plant_staking::benchmarking::create_validator_with_nominators::<Runtime>(
		max_nominations,
		max_nominations,
		false,
		true,
		plant_staking::RewardDestination::Staked,
	)?;

	Staking::bonded(&stash).ok_or("not stash")
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_setup_session_membership_proof(
	n: u32,
) -> Result<SessionMembershipBenchmarkSetup, &'static str> {
	plant_staking::ValidatorCount::<Runtime>::put(n);
	let mut first_key = None;

	for who in plant_staking::testing_utils::create_validators::<Runtime>(n, 1000)? {
		let validator =
			<Runtime as topsoil_system::Config>::Lookup::lookup(who).map_err(|_| "lookup failed")?;
		let controller = Staking::bonded(&validator).ok_or("not stash")?;
		let (keys, proof) = benchmark_generate_session_keys_and_proof(controller.clone());

		if first_key.is_none() {
			let key_type = SessionKeys::key_ids()[0];
			let key_data = keys.get_raw(key_type).to_vec();
			first_key = Some((key_type, key_data));
		}

		Session::set_keys(
			topsoil_system::RawOrigin::Signed(controller).into(),
			keys,
			proof,
		)
		.map_err(|_| "failed to set keys")?;
	}

	plant_session::benchmarking::Pallet::<Runtime>::on_initialize(
		subsoil::runtime::traits::One::one(),
	);

	while plant_session::Validators::<Runtime>::get().len() < n as usize {
		Session::rotate_session();
	}

	let key = first_key.ok_or("missing benchmark key")?;
	let proof = Historical::prove((key.0, key.1.clone())).ok_or("failed to prove")?;
	Ok((key, proof))
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_offence_bond_amount() -> Balance {
	plant_staking::asset::existential_deposit::<Runtime>().saturating_mul(10_000u32.into())
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_create_offender(
	index: u32,
	nominators: u32,
) -> Result<BenchmarkOffender, &'static str> {
	const SEED: u32 = 0;
	const MAX_BENCH_NOMINATORS: u32 = 100;

	let stash: AccountId = topsoil_benchmarking::account("stash", index, SEED);
	let stash_lookup = <Runtime as topsoil_system::Config>::Lookup::unlookup(stash.clone());
	let reward_destination = plant_staking::RewardDestination::Staked;
	let amount = benchmark_offence_bond_amount();
	let free_amount = amount.saturating_mul(2u32.into());

	plant_staking::asset::set_stakeable_balance::<Runtime>(&stash, free_amount);
	Staking::bond(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		amount,
		reward_destination.clone(),
	)
	.map_err(|_| "failed to bond stash")?;

	let validator_prefs = plant_staking::ValidatorPrefs {
		commission: Perbill::from_percent(50),
		..Default::default()
	};
	Staking::validate(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		validator_prefs,
	)
	.map_err(|_| "failed to validate")?;

	let (keys, proof) = benchmark_generate_session_keys_and_proof(stash.clone());
	Session::ensure_can_pay_key_deposit(&stash).map_err(|_| "key deposit")?;
	Session::set_keys(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		keys,
		proof,
	)
	.map_err(|_| "failed to set keys")?;

	let mut individual_exposures = Vec::new();
	let mut nominator_stashes = Vec::new();
	for i in 0..nominators {
		let nominator_stash: AccountId =
			topsoil_benchmarking::account("nominator stash", index * MAX_BENCH_NOMINATORS + i, SEED);
		plant_staking::asset::set_stakeable_balance::<Runtime>(&nominator_stash, free_amount);
		Staking::bond(
			topsoil_system::RawOrigin::Signed(nominator_stash.clone()).into(),
			amount,
			reward_destination.clone(),
		)
		.map_err(|_| "failed to bond nominator")?;
		Staking::nominate(
			topsoil_system::RawOrigin::Signed(nominator_stash.clone()).into(),
			vec![stash_lookup.clone()],
		)
		.map_err(|_| "failed to nominate")?;
		individual_exposures.push(plant_staking::IndividualExposure {
			who: nominator_stash.clone(),
			value: amount,
		});
		nominator_stashes.push(nominator_stash);
	}

	let exposure = plant_staking::Exposure {
		total: amount.saturating_mul((nominators + 1).into()),
		own: amount,
		others: individual_exposures,
	};
	Staking::add_era_stakers(0, stash.clone(), exposure);

	Ok(BenchmarkOffender { controller: stash.clone(), stash, nominator_stashes })
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_make_offenders(
	num_offenders: u32,
	num_nominators: u32,
) -> Result<Vec<RuntimeIdentificationTuple>, &'static str> {
	let mut offenders = Vec::new();
	for i in 0..num_offenders {
		let offender = benchmark_create_offender(i + 1, num_nominators)?;
		plant_session::Validators::<Runtime>::mutate(|validators| {
			validators.push(offender.controller.clone())
		});
		offenders.push(offender);
	}

	let id_tuples = offenders
		.iter()
		.map(|offender| offender.controller.clone())
		.map(|validator_id| {
			<Runtime as plant_session::historical::Config>::FullIdentificationOf::convert(
				validator_id.clone(),
			)
			.map(|full_id| (validator_id, full_id))
			.expect("historical identification should exist")
		})
		.collect::<Vec<_>>();

	if plant_staking::ActiveEra::<Runtime>::get().is_none() {
		plant_staking::ActiveEra::<Runtime>::put(plant_staking::ActiveEraInfo {
			index: 0,
			start: Some(0),
		});
	}

	Ok(id_tuples)
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_setup_grandpa_offence(
	n: u32,
) -> Result<
	(
		Vec<AccountId>,
		plant_grandpa::EquivocationOffence<RuntimeIdentificationTuple>,
	),
	&'static str,
> {
	let reporters = vec![topsoil_benchmarking::account("reporter", 1, 0)];
	Staking::set_slash_reward_fraction(Perbill::one());
	let mut offenders = benchmark_make_offenders(1, n)?;
	let validator_set_count = Session::validators().len() as u32;
	let offence = plant_grandpa::EquivocationOffence {
		time_slot: plant_grandpa::TimeSlot { set_id: 0, round: 0 },
		session_index: 0,
		validator_set_count,
		offender: offenders.pop().ok_or("missing offender")?,
	};
	Ok((reporters, offence))
}

#[cfg(feature = "runtime-benchmarks")]
fn benchmark_setup_babe_offence(
	n: u32,
) -> Result<
	(
		Vec<AccountId>,
		plant_babe::EquivocationOffence<RuntimeIdentificationTuple>,
	),
	&'static str,
> {
	let reporters = vec![topsoil_benchmarking::account("reporter", 1, 0)];
	Staking::set_slash_reward_fraction(Perbill::one());
	let mut offenders = benchmark_make_offenders(1, n)?;
	let validator_set_count = Session::validators().len() as u32;
	let offence = plant_babe::EquivocationOffence {
		slot: 0u64.into(),
		session_index: 0,
		validator_set_count,
		offender: offenders.pop().ok_or("missing offender")?,
	};
	Ok((reporters, offence))
}

subsoil::api::impl_runtime_apis! {
	impl subsoil::api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: <Block as BlockT>::LazyBlock) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) -> subsoil::runtime::ExtrinsicInclusionMode {
			Executive::initialize_block(header)
		}
	}

	impl subsoil::api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> alloc::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl topsoil_support::view_functions::runtime_api::RuntimeViewFunction<Block> for Runtime {
		fn execute_view_function(id: topsoil_support::view_functions::ViewFunctionId, input: Vec<u8>) -> Result<Vec<u8>, topsoil_support::view_functions::ViewFunctionDispatchError> {
			Runtime::execute_view_function(id, input)
		}
	}

	impl subsoil::block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: <Block as BlockT>::LazyBlock, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl subsoil::txpool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl subsoil::offchain_worker::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl subsoil::consensus::grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> subsoil::consensus::grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> subsoil::consensus::grandpa::SetId {
			plant_grandpa::CurrentSetId::<Runtime>::get()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: subsoil::consensus::grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: subsoil::consensus::grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: subsoil::consensus::grandpa::SetId,
			authority_id: GrandpaId,
		) -> Option<subsoil::consensus::grandpa::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((subsoil::consensus::grandpa::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(subsoil::consensus::grandpa::OpaqueKeyOwnershipProof::new)
		}
	}

	impl plant_nomination_pools_runtime_api::NominationPoolsApi<Block, AccountId, Balance> for Runtime {
		fn pending_rewards(who: AccountId) -> Balance {
			NominationPools::api_pending_rewards(who).unwrap_or_default()
		}

		fn points_to_balance(pool_id: PoolId, points: Balance) -> Balance {
			NominationPools::api_points_to_balance(pool_id, points)
		}

		fn balance_to_points(pool_id: PoolId, new_funds: Balance) -> Balance {
			NominationPools::api_balance_to_points(pool_id, new_funds)
		}

		fn pool_pending_slash(pool_id: PoolId) -> Balance {
			NominationPools::api_pool_pending_slash(pool_id)
		}

		fn member_pending_slash(member: AccountId) -> Balance {
			NominationPools::api_member_pending_slash(member)
		}

		fn pool_needs_delegate_migration(pool_id: PoolId) -> bool {
			NominationPools::api_pool_needs_delegate_migration(pool_id)
		}

		fn member_needs_delegate_migration(member: AccountId) -> bool {
			NominationPools::api_member_needs_delegate_migration(member)
		}

		fn member_total_balance(member: AccountId) -> Balance {
			NominationPools::api_member_total_balance(member)
		}

		fn pool_balance(pool_id: PoolId) -> Balance {
			NominationPools::api_pool_balance(pool_id)
		}

		fn pool_accounts(pool_id: PoolId) -> (AccountId, AccountId) {
			NominationPools::api_pool_accounts(pool_id)
		}
	}

	impl plant_staking::runtime_api::StakingApi<Block, Balance, AccountId> for Runtime {
		fn nominations_quota(balance: Balance) -> u32 {
			Staking::api_nominations_quota(balance)
		}

		fn eras_stakers_page_count(era: subsoil::staking::EraIndex, account: AccountId) -> subsoil::staking::Page {
			Staking::api_eras_stakers_page_count(era, account)
		}

		fn pending_rewards(era: subsoil::staking::EraIndex, account: AccountId) -> bool {
			Staking::api_pending_rewards(era, account)
		}
	}

	impl subsoil::consensus::babe::BabeApi<Block> for Runtime {
		fn configuration() -> subsoil::consensus::babe::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			subsoil::consensus::babe::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> subsoil::consensus::babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> subsoil::consensus::babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> subsoil::consensus::babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: subsoil::consensus::babe::Slot,
			authority_id: subsoil::consensus::babe::AuthorityId,
		) -> Option<subsoil::consensus::babe::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((subsoil::consensus::babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(subsoil::consensus::babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: subsoil::consensus::babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: subsoil::consensus::babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl plant_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl plant_oracle_runtime_api::OracleApi<Block, u32, u32, u128> for Runtime {
		fn get_value(_provider_id: u32, key: u32) -> Option<u128> {
			// ProviderId is unused as we only have 1 provider
			plant_oracle::Pallet::<Runtime>::get(&key).map(|v| v.value)
		}

		fn get_all_values(_provider_id: u32) -> Vec<(u32, Option<u128>)> {
			use plant_oracle::DataProviderExtended;
			plant_oracle::Pallet::<Runtime>::get_all_values()
				.map(|(k, v)| (k, v.map(|tv| tv.value)))
				.collect()
		}
	}

	impl topsoil_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl assets_api::AssetsApi<
		Block,
		AccountId,
		Balance,
		u32,
	> for Runtime
	{
		fn account_balances(account: AccountId) -> Vec<(u32, Balance)> {
			Assets::account_balances(account)
		}
	}

	impl plant_transaction_payment::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl plant_asset_conversion::AssetConversionApi<
		Block,
		Balance,
		NativeOrWithId<u32>
	> for Runtime
	{
		fn quote_price_exact_tokens_for_tokens(asset1: NativeOrWithId<u32>, asset2: NativeOrWithId<u32>, amount: Balance, include_fee: bool) -> Option<Balance> {
			AssetConversion::quote_price_exact_tokens_for_tokens(asset1, asset2, amount, include_fee)
		}

		fn quote_price_tokens_for_exact_tokens(asset1: NativeOrWithId<u32>, asset2: NativeOrWithId<u32>, amount: Balance, include_fee: bool) -> Option<Balance> {
			AssetConversion::quote_price_tokens_for_exact_tokens(asset1, asset2, amount, include_fee)
		}

		fn get_reserves(asset1: NativeOrWithId<u32>, asset2: NativeOrWithId<u32>) -> Option<(Balance, Balance)> {
			AssetConversion::get_reserves(asset1, asset2).ok()
		}
	}

	impl plant_transaction_payment::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl plant_nfts_runtime_api::NftsApi<Block, AccountId, u32, u32> for Runtime {
		fn owner(collection: u32, item: u32) -> Option<AccountId> {
			<Nfts as Inspect<AccountId>>::owner(&collection, &item)
		}

		fn collection_owner(collection: u32) -> Option<AccountId> {
			<Nfts as Inspect<AccountId>>::collection_owner(&collection)
		}

		fn attribute(
			collection: u32,
			item: u32,
			key: Vec<u8>,
		) -> Option<Vec<u8>> {
			<Nfts as Inspect<AccountId>>::attribute(&collection, &item, &key)
		}

		fn custom_attribute(
			account: AccountId,
			collection: u32,
			item: u32,
			key: Vec<u8>,
		) -> Option<Vec<u8>> {
			<Nfts as Inspect<AccountId>>::custom_attribute(
				&account,
				&collection,
				&item,
				&key,
			)
		}

		fn system_attribute(
			collection: u32,
			item: Option<u32>,
			key: Vec<u8>,
		) -> Option<Vec<u8>> {
			<Nfts as Inspect<AccountId>>::system_attribute(&collection, item.as_ref(), &key)
		}

		fn collection_attribute(collection: u32, key: Vec<u8>) -> Option<Vec<u8>> {
			<Nfts as Inspect<AccountId>>::collection_attribute(&collection, &key)
		}
	}

	#[api_version(6)]
	impl subsoil::consensus::beefy::BeefyApi<Block, BeefyId> for Runtime {
		fn beefy_genesis() -> Option<BlockNumber> {
			plant_beefy::GenesisBlock::<Runtime>::get()
		}

		fn validator_set() -> Option<subsoil::consensus::beefy::ValidatorSet<BeefyId>> {
			Beefy::validator_set()
		}

		fn submit_report_double_voting_unsigned_extrinsic(
			equivocation_proof: subsoil::consensus::beefy::DoubleVotingProof<
				BlockNumber,
				BeefyId,
				BeefySignature,
			>,
			key_owner_proof: subsoil::consensus::beefy::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Beefy::submit_unsigned_double_voting_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn submit_report_fork_voting_unsigned_extrinsic(
			equivocation_proof:
				subsoil::consensus::beefy::ForkVotingProof<
					<Block as BlockT>::Header,
					BeefyId,
					subsoil::runtime::OpaqueValue
				>,
			key_owner_proof: subsoil::consensus::beefy::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			Beefy::submit_unsigned_fork_voting_report(
				equivocation_proof.try_into()?,
				key_owner_proof.decode()?,
			)
		}

		fn submit_report_future_block_voting_unsigned_extrinsic(
			equivocation_proof: subsoil::consensus::beefy::FutureBlockVotingProof<BlockNumber, BeefyId>,
			key_owner_proof: subsoil::consensus::beefy::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			Beefy::submit_unsigned_future_block_voting_report(
				equivocation_proof,
				key_owner_proof.decode()?,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: subsoil::consensus::beefy::ValidatorSetId,
			authority_id: BeefyId,
		) -> Option<subsoil::consensus::beefy::OpaqueKeyOwnershipProof> {
			Historical::prove((subsoil::consensus::beefy::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(subsoil::consensus::beefy::OpaqueKeyOwnershipProof::new)
		}
	}

	#[api_version(3)]
	impl plant_mmr::primitives::MmrApi<
		Block,
		mmr::Hash,
		BlockNumber,
	> for Runtime {
		fn mmr_root() -> Result<mmr::Hash, mmr::Error> {
			Ok(plant_mmr::RootHash::<Runtime>::get())
		}

		fn mmr_leaf_count() -> Result<mmr::LeafIndex, mmr::Error> {
			Ok(plant_mmr::NumberOfLeaves::<Runtime>::get())
		}

		fn generate_proof(
			block_numbers: Vec<BlockNumber>,
			best_known_block_number: Option<BlockNumber>,
		) -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::LeafProof<mmr::Hash>), mmr::Error> {
			Mmr::generate_proof(block_numbers, best_known_block_number).map(
				|(leaves, proof)| {
					(
						leaves
							.into_iter()
							.map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf))
							.collect(),
						proof,
					)
				},
			)
		}

		fn verify_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::LeafProof<mmr::Hash>)
			-> Result<(), mmr::Error>
		{
			let leaves = leaves.into_iter().map(|leaf|
				leaf.into_opaque_leaf()
				.try_decode()
				.ok_or(mmr::Error::Verify)).collect::<Result<Vec<mmr::Leaf>, mmr::Error>>()?;
			Mmr::verify_leaves(leaves, proof)
		}

		fn generate_ancestry_proof(
			prev_block_number: BlockNumber,
			best_known_block_number: Option<BlockNumber>,
		) -> Result<mmr::AncestryProof<mmr::Hash>, mmr::Error> {
			Mmr::generate_ancestry_proof(prev_block_number, best_known_block_number)
		}

		fn verify_proof_stateless(
			root: mmr::Hash,
			leaves: Vec<mmr::EncodableOpaqueLeaf>,
			proof: mmr::LeafProof<mmr::Hash>
		) -> Result<(), mmr::Error> {
			let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
			plant_mmr::verify_leaves_proof::<mmr::Hashing, _>(root, nodes, proof)
		}
	}

	impl subsoil::mixnet::runtime_api::MixnetApi<Block> for Runtime {
		fn session_status() -> subsoil::mixnet::types::SessionStatus {
			Mixnet::session_status()
		}

		fn prev_mixnodes() -> Result<Vec<subsoil::mixnet::types::Mixnode>, subsoil::mixnet::types::MixnodesErr> {
			Mixnet::prev_mixnodes()
		}

		fn current_mixnodes() -> Result<Vec<subsoil::mixnet::types::Mixnode>, subsoil::mixnet::types::MixnodesErr> {
			Mixnet::current_mixnodes()
		}

		fn maybe_register(session_index: subsoil::mixnet::types::SessionIndex, mixnode: subsoil::mixnet::types::Mixnode) -> bool {
			Mixnet::maybe_register(session_index, mixnode)
		}
	}

	impl subsoil::session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(owner: Vec<u8>, seed: Option<Vec<u8>>) -> subsoil::session::OpaqueGeneratedSessionKeys {
			SessionKeys::generate(&owner, seed).into()
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl plant_asset_rewards::AssetRewards<Block, Balance> for Runtime {
		fn pool_creation_cost() -> Balance {
			StakePoolCreationDeposit::get()
		}
	}

	impl soil_transaction_storage_proof::runtime_api::TransactionStorageApi<Block> for Runtime {
		fn retention_period() -> NumberFor<Block> {
			TransactionStorage::retention_period()
		}
	}

	#[cfg(feature = "try-runtime")]
	impl topsoil_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: topsoil_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: <Block as BlockT>::LazyBlock,
			state_root_check: bool,
			signature_check: bool,
			select: topsoil_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl topsoil_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<topsoil_benchmarking::BenchmarkList>,
			Vec<topsoil_support::traits::StorageInfo>,
		) {
			use topsoil_benchmarking::{baseline, BenchmarkList};
			use topsoil_support::traits::StorageInfoTrait;

			use plant_session::benchmarking::Pallet as SessionBench;
			use plant_offences::benchmarking::Pallet as OffencesBench;
			use plant_election_provider::benchmarking::Pallet as EPSBench;
			use topsoil_system_benchmarking::Pallet as SystemBench;
			use topsoil_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
			use baseline::Pallet as BaselineBench;
			use plant_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		#[allow(non_local_definitions)]
		fn dispatch_benchmark(
			config: topsoil_benchmarking::BenchmarkConfig
		) -> Result<Vec<topsoil_benchmarking::BenchmarkBatch>, alloc::string::String> {
			use topsoil_benchmarking::{baseline, BenchmarkBatch};
			use subsoil::storage::TrackedStorageKey;

			use plant_session::benchmarking::Pallet as SessionBench;
			use plant_offences::benchmarking::Pallet as OffencesBench;
			use plant_election_provider::benchmarking::Pallet as EPSBench;
			use topsoil_system_benchmarking::Pallet as SystemBench;
			use topsoil_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
			use baseline::Pallet as BaselineBench;
			use plant_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

			impl plant_session::benchmarking::Config for Runtime {
				fn generate_session_keys_and_proof(owner: Self::AccountId) -> (Self::Keys, Vec<u8>) {
					benchmark_generate_session_keys_and_proof(owner)
				}

				fn setup_benchmark_controller() -> Result<Self::AccountId, &'static str> {
					benchmark_setup_session_controller()
				}

				fn setup_membership_proof_benchmark(
					n: u32,
				) -> Result<
					(
						(subsoil::runtime::KeyTypeId, Vec<u8>),
						subsoil::session::MembershipProof,
					),
					&'static str,
				> {
					benchmark_setup_session_membership_proof(n)
				}
			}

			impl plant_offences::benchmarking::Config for Runtime {
				type MaxNominators = plant_staking::MaxNominationsOf<Self>;
				type BabeBenchmarkOffence =
					plant_babe::EquivocationOffence<plant_session::historical::IdentificationTuple<Self>>;
				type GrandpaBenchmarkOffence =
					plant_grandpa::EquivocationOffence<plant_session::historical::IdentificationTuple<Self>>;

				fn setup_babe_benchmark(
					n: u32,
				) -> Result<(Vec<Self::AccountId>, Self::BabeBenchmarkOffence), &'static str> {
					benchmark_setup_babe_offence(n)
				}

				fn setup_grandpa_benchmark(
					n: u32,
				) -> Result<(Vec<Self::AccountId>, Self::GrandpaBenchmarkOffence), &'static str> {
					benchmark_setup_grandpa_offence(n)
				}
			}

			impl plant_election_provider::benchmarking::Config for Runtime {}
			impl topsoil_system_benchmarking::Config for Runtime {}
			impl plant_transaction_payment::BenchmarkConfig for Runtime {}
			impl baseline::Config for Runtime {}
			impl plant_nomination_pools_benchmarking::Config for Runtime {}

			use topsoil_support::traits::WhitelistedStorageKeys;
			let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			// Treasury Account
			// TODO: this is manual for now, someday we might be able to use a
			// macro for this particular key
			let treasury_key = topsoil_system::Account::<Runtime>::hashed_key_for(Treasury::account_id());
			whitelist.push(treasury_key.to_vec().into());

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);
			Ok(batches)
		}
	}

	impl subsoil::genesis_builder::GenesisBuilder<Block> for Runtime {
		fn build_state(config: Vec<u8>) -> subsoil::genesis_builder::Result {
			build_state::<RuntimeGenesisConfig>(config)
		}

		fn get_preset(id: &Option<subsoil::genesis_builder::PresetId>) -> Option<Vec<u8>> {
			get_preset::<RuntimeGenesisConfig>(id, &genesis_config_presets::get_preset)
		}

		fn preset_names() -> Vec<subsoil::genesis_builder::PresetId> {
			genesis_config_presets::preset_names()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use topsoil_system::offchain::CreateSignedTransaction;

	#[test]
	fn validate_transaction_submitter_bounds() {
		fn is_submit_signed_transaction<T>()
		where
			T: CreateSignedTransaction<RuntimeCall>,
		{
		}

		is_submit_signed_transaction::<Runtime>();
	}

	#[test]
	fn call_size() {
		let size = core::mem::size_of::<RuntimeCall>();
		assert!(
			size <= CALL_PARAMS_MAX_SIZE,
			"size of RuntimeCall {} is more than {CALL_PARAMS_MAX_SIZE} bytes.
			 Some calls have too big arguments, use Box to reduce the size of RuntimeCall.
			 If the limit is too strong, maybe consider increase the limit.",
			size,
		);
	}
}
