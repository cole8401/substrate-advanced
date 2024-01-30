use hex_literal::hex;

use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, BABE_GENESIS_EPOCH_CONFIG, SessionConfig, StakingConfig, SessionKeys,
	constants::currency::*, StakerStatus, MaxNominations, ImOnlineConfig,
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sc_telemetry::TelemetryEndpoints;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use node_primitives::*;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![
		"/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWHNCwt8FPoexf5LbaZhAxPDcZx2r8zTAvngdx8A3Rnu1z".parse().unwrap(),
	];

	ChainSpec::from_genesis(
		"Substrate Ubuntu",
		"ubuntu_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// ./target/release/node-template key generate --scheme Sr25519 --password-interactive
	// export SECRET='cover tragic mistake win library cry battle suffer firm sponsor impact excuse'
	// for i in 1 2 3 4; do for j in stash controller; do ./target/release/node-template key inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do ./target/release/node-template key inspect "$SECRET//$i//$j -- --sr25519"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do ./target/release/node-template key inspect "$SECRET//$i//$j -- --ed25519"; done; done
	// for i in 1 2 3 4; do for j in im_online; do ./target/release/node-template key inspect "$SECRET//$i//$j -- --sr25519"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5C7bQYEChP2SfncdhdAGXiS1ComgWfLm26LZG2MK55hwfPu4
			hex!["02340598dde3924e35909c6186e15eb8a85648e52b4783d959d330c395f7380a"].into(),
			// 5CyjuKfxDy3Pm6C9y4LMA1ZUbD3E7mhhEvxUv4cfCQwMTDS8
			hex!["2872e4a0197bfd106a4964b647c8ae84e8fdd7f99959880323b93c4bd0669908"].into(),
			// 5F1BzQv5dPJ2i13jMzxUoZukEHnhnKknKzPzdonQ29JLFTnK
			hex!["82066cb4443b48a167801d92346d7813b0132547095534ef243645cfd4771525"]
				.unchecked_into(),
			// 5DxsjT69KAaVsTTjpbkwG1mNuLYG3LQ49yLxqnNnUjwSxFu5
			hex!["54063d112316f0da562c3138a39fb6870c8e12d4ee5ce41a4c01f07520ee3e78"]
				.unchecked_into(),
			// 5DeY38vefpS1briWavhK56HnspeeN8Z64nkgsetwmvaB7ZT2
			hex!["46098594b6a00964232112660fa868d310502ceb5ccb614ead570a20c4cd745d"]
				.unchecked_into(),
		),
		(
			// 5Cg3TJWGceuzzmduYL3iPbr24sdQrNPLL5uCFrgsLoHBy6Zm
			hex!["1af38f1a18c634d5bda3b2fda1c88813a393bdf1f9d1ee682054534fddd81d2e"].into(),
			// 5Dc6ABh8ZiEifFheaCw4R1sC1bHirWPLptBX3kjMDSMcHsWB
			hex!["442bea8e73eb25e401f2f8b4dd3f8df89415671178b56042474d0586e216d162"].into(),
			// HCKEnFTKsnihRsse1yQ3nPNkQz4J7TYUQkuYWaYM3gh6GrG
			hex!["e2fafd2e0cf63997850c90476194541aefb3c1e186cfdba773b867601e732809"]
				.unchecked_into(),
			// 5EqVMW1v8hGkfdHQGVEUzEaTqzJVZYSvFsA9KVWwVJ29Q35G
			hex!["7aa06bef4d52a412130a1332aad1f3126cbf6a09feba9101b15845cd1d4f7828"]
				.unchecked_into(),
			// 5FKpnKAKQAU8VLMySDgtta7EEgtKsLjqPTM11bNvYx2PZfgF
			hex!["903cb1598b9b170bdf83a037c18cc753ada9f8f961980f48c950e492459a4a3d"]
				.unchecked_into(),
		),
		(
			// 5EC9ZatAJNcG1zrbkuJJjwfe4k2Dbs3XmDvk4MBjKanw6tpL
			hex!["5e25b78d7ef73fb03c48b5550c7762f2fffaff54ef6cac0d670157cf2ba18563"].into(),
			// 5CcwzmrQg71nAQX8XpFKXojxF7GUHY5C74c4HeXdxa5uVFxK
			hex!["1897739a555a3ffc548045b2d3580510e9d30e4529d7b92bc35da4421200d160"].into(),
			// 5FkAYRqRghSYoSb6z5EtARoUvqW8yCMeAPDXV6fKZG3uYJux
			hex!["a2cd1a44275ba6e8bed6aaa781340bdddd6a5d5692ea4321f1f3c2756d287568"]
				.unchecked_into(),
			// 5EXY3YmUQDw414XyP5KzWccAthGDBhxR2Yp3Xa9tmg875n7p
			hex!["6cef0fb01be6fb2867d38fa6d9aac93a17835c504da8ceedd77fa009a8c13747"]
				.unchecked_into(),
			// 5CJUD6rsQkN1WnQ7KW2xszwxVABQovmMQQ1jTq9Hk3XFpW7Z
			hex!["0a7f7e4eccdab5b75aa548081741f286614b12f908832123b26cfa59f3acc03c"]
				.unchecked_into(),
		),
		(
			// 5FZcArDEk2ddwCRDVn6NzuL68X6cAcoqAxZ2kkLLMjG3wBgb
			hex!["9abfacaa81504b72ed3d0e4379e6604320b394cb5dc75f89bac64fc0798a901a"].into(),
			// 5F1dcFSJkE9RweJqsjKbH8VFdpo1Pibo6s5m4UxWs1zqH59E
			hex!["825ca97d686833d181f76235eab34d3c415947a3363f559db3d966616291cb53"].into(),
			// 5DURgG1vMnYFetfdY9ZHbGd7Tss7Y45xmBZDABbY9LtYdFD6
			hex!["3e53a8810f62b9ea7fa222e55614a48d05dba0673452adde0aa477d60d798129"]
				.unchecked_into(),
			// 5FRYEcsZTbVUFGsQExbuGGUoWKsXEXWeBv21Jpx63hJDbSGf
			hex!["949877008633194e024c652eb37bad176f0292c39d5a3d33aef241436973ef79"]
				.unchecked_into(),
			// 5GExdfAjzKg7FxTpzczQ1EUY4Hvg1Nem5J7oSwNifazvhXjW
			hex!["b8c32094aa63b039ed16a04d0208fd1f32b87d8b957e774dadd18452e550b115"]
				.unchecked_into(),
		),
	];

	// generated with secret: ./target/release/node-template key inspect "$secret"/fir
	// 5Ct1SLsTZB4bMeKCdchDWL2oN7hNTpjmrVqDsrWqspERFSDo
	let root_key: AccountId = hex![
		"2413b72fb065b202a7b84c9a663c167c1435ec0814edf40a3c7ccbcf7a23ba33"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		wasm_binary,
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts,
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
