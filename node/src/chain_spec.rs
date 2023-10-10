use hex_literal::hex;

use lunes_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, BABE_GENESIS_EPOCH_CONFIG, SessionConfig, StakingConfig, SessionKeys,
	constants::currency::*, StakerStatus, ImOnlineConfig,Balance,IndicesConfig,
	CouncilConfig,DemocracyConfig,TechnicalCommitteeConfig,
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
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LUNES".into());
	properties.insert("tokenDecimals".into(), 8.into());
	properties.insert("ss58Format".into(), 57.into());

	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut balances = vec![];
	balances.push((get_account_id_from_seed::<sr25519::Public>("Bob").clone(), INITIAL_COLLATOR_STAKING));
	balances.push((get_account_id_from_seed::<sr25519::Public>("Alice//stash").clone(), INITIAL_COLLATOR_STAKING));
	balances.push((get_account_id_from_seed::<sr25519::Public>("Bob//stash").clone(), INITIAL_COLLATOR_STAKING));
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
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				balances.clone(),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("lunes-development"),
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut balances = vec![];
	balances.push((get_account_id_from_seed::<sr25519::Public>("Bob").clone(), INITIAL_COLLATOR_STAKING));
	balances.push((get_account_id_from_seed::<sr25519::Public>("Alice//stash").clone(), INITIAL_COLLATOR_STAKING));
	balances.push((get_account_id_from_seed::<sr25519::Public>("Bob//stash").clone(), INITIAL_COLLATOR_STAKING));
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
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				balances.clone(),
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
	let boot_nodes = vec![];
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LUNES".into());
	properties.insert("tokenDecimals".into(), 8.into());
	properties.insert("ss58Format".into(), 57.into());
	ChainSpec::from_genesis(
		"Lunes Nigthly",
		"local_node",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		// Protocol ID
		Some("lunes-mainnet"),
		None,
		// Properties
		Some(properties),
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
			(
				// 5FRcxav7mqTxtNfXcKgGs7EURyi4xnDWkW49QbAHmzAYo4rE
				hex!["94a85da278c784b238c790b54c352d676e4a6a23e6f8548cde35a2097313943c"].into(),
				// 5FZXKSRR3DRcBEaKRFD4vMr9zy4pbCZ3wBVwT8rBLwJaBjPK
				hex!["9aaf576e4efeecda3e9447f43a560bd73b8366bdc02369e72cb46053d7818215"].into(),
				// 5GnggbKqQjrCmP4TaAPkQ6fVoU2X8HyBHoUX3nqxfAchgb4q
				hex!["d0f54a287728c3db1ec36925fd44d9d800dfbdef2fdb063d20a0de0fc2e8fc78"].unchecked_into(),
				// 5EKteVtU6i1i2ohk4UjujhiU2kpEb7R3RR4RnKkvAMHRHCnb
				hex!["640d788f4f42174ad158d088bb8a66623c3257d5f0da7f9af2808a55ee55c8ce"].unchecked_into(),
				// 5F7GGrtCXRfmJCogM9wEUSTEsxpVwUem8twKeSCivbxUX5YW
				hex!["86a8511e8514fa95633b6cd1c0e0d7eaed9e031e457c4e0479116e576e136b22"].unchecked_into(),
			),
			(
				// 5FCrm8WuNSeWEjH2sdrebT7PkMtTaQ827knV4F74oyNQaXqX
				hex!["8aeca38bb0029ff889d832e1560edeeb1d0554c97d19403b1539f3024e7a5f66"].into(),
				// 5FUGF8X3ypT4yvhoEjkHWEfEgAtVMb9sUguTNN9vtrSGgh5N
				hex!["96ac5eb3c0b8f9c4efdbd4bf7c33c12a5cfbafc2dcb76747eebd9722348d955f"].into(),
				// 5HDovSJuHG6kjewXfyhzRHBZRPgx6h9VFqrnvHsexEa8LV4j
				hex!["e41ecadc32c07618b6c21b9fa001c2ff4050a7234627d53303d837a5b7e4bb2c"].unchecked_into(),
				// 5Dij1fKjDz6QoxcW8PxqZ4CbUMDRK8kndgBPRRehfwhQsip9
				hex!["493b72726616af9fac0ce8e2a888548129713b33706dc17bec3b4a62b5908a96"].unchecked_into(),
				// 5GYmPhuhbwfRmQDkYqj9A5myv3WgUAXxZSrLwhafnhTFZMbp
				hex!["c657b4974a19891d678329133721943ff36e04f2f508bf9a841e35b4b8a35a45"].unchecked_into(),
			)
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5CfptqCCc5Y5xnqVSz8FQNKHXnmBPfRotrPJL1FbXH6MLNxg
		"1ac9475ee6d6446eaa9657cd6b1bbef5c94b041b308dfcd59bad0f97ef86b849"
	]
	.into();
	//c<AccountId>, <Balnance>
	let mut endowed_accounts: Vec<(sp_runtime::AccountId32, Balance)> = vec![];
	endowed_accounts.push((hex!["94a85da278c784b238c790b54c352d676e4a6a23e6f8548cde35a2097313943c"].into(), INITIAL_COLLATOR_STAKING)); //validator
	endowed_accounts.push((hex!["8aeca38bb0029ff889d832e1560edeeb1d0554c97d19403b1539f3024e7a5f66"].into(), INITIAL_COLLATOR_STAKING)); //validator
	testnet_genesis(
		wasm_binary,
		initial_authorities,
		root_key,
		endowed_accounts.clone(),
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId, 
		AccountId, 
		BabeId, 
		GrandpaId, 
		ImOnlineId
	)>,
	root_key: AccountId,
	mut endowed_accounts: Vec<(AccountId , Balance)>,
	_enable_println: bool,
) -> GenesisConfig {
	// endow all authorities and nominators.
	let mut genesis_issuance = TOTAL_INITIAL_ISSUANCE_LUNES;
	for balance in endowed_accounts.clone() {
		genesis_issuance -= balance.1;
	}
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), EXISTENTIAL_DEPOSIT_LUNES, StakerStatus::Validator))
		.collect::<Vec<_>>();
	endowed_accounts.push((root_key.clone(), genesis_issuance));
	let council: Vec<_> = endowed_accounts
		.iter()
		.map(|address| address.0.clone())
		.collect();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts,
		},
		indices: IndicesConfig { indices: vec![] },
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
		democracy: DemocracyConfig::default(),
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: council,
			..Default::default()
		},
		evm: Default::default(),
		dynamic_fee: Default::default(),
		treasury: Default::default(),
		alliance_motion: Default::default(),
		assets: pallet_assets::GenesisConfig {
			..Default::default()
		},
		scored_pool: pallet_scored_pool::GenesisConfig {
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
