use crate::*;
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use parity_scale_codec::{alloc::sync::Arc, Decode};
use parking_lot::RwLock;
use sp_core::{
	offchain::{
		testing::{self, OffchainState, PoolState},
		OffchainExt, TransactionPoolExt,
	},
	sr25519::{self, Signature},
	testing::KeyStore,
	traits::KeystoreExt,
	H256,
};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentityLookup, Verify},
	Perbill,
};

use crate as ocw_demo;

impl_outer_origin! {
	pub enum Origin for TestRuntime where system = system {}
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		system<T>,
		ocw_demo<T>,
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1_000_000;
	pub const MaximumBlockLength: u32 = 10 * 1_000_000;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

// The TestRuntime implements two pallet/frame traits: system, and simple_event
impl system::Trait for TestRuntime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

// --- mocking offchain-demo trait

type TestExtrinsic = TestXt<Call<TestRuntime>, ()>;

parameter_types! {
	pub const UnsignedPriority: u64 = 100;
}

impl Trait for TestRuntime {
	type AuthorityId = crypto::TestAuthId;
	type Call = Call<TestRuntime>;
	type Event = TestEvent;
}

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for TestRuntime
where
	Call<TestRuntime>: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call<TestRuntime>,
		_public: <Signature as Verify>::Signer,
		_account: <TestRuntime as system::Trait>::AccountId,
		index: <TestRuntime as system::Trait>::Index,
	) -> Option<(
		Call<TestRuntime>,
		<TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		Some((call, (index, ())))
	}
}

impl frame_system::offchain::SigningTypes for TestRuntime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for TestRuntime
where
	Call<TestRuntime>: From<C>,
{
	type OverarchingCall = Call<TestRuntime>;
	type Extrinsic = TestExtrinsic;
}

pub type System = system::Module<TestRuntime>;
pub type OcwDemo = Module<TestRuntime>;

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.write()
			.sr25519_generate_new(KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();

		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();

		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(keystore));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}


#[test]
fn test_fetch_dot_price_info() {
	let (mut t, pool_state, _offchain_state) = ExternalityBuilder::build();

	t.execute_with(|| {
		let resp_str = "{
			\"data\": {
				\"priceUsd\": \"3.1415\"
			}
		}";

		OcwDemo::fetch_dot_price_info();
	});
}