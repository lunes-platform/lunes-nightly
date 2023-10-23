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
			// 5FWPxoqLoMY8ZYYP6phwPd2SKKUACXqhk4q5B8JGoQm3sGgu
			hex!["984cd803c81167d49c5af6149d3e55f567b73fd9109251223dc567e628681f49"].into(),
			// 5FZXKSRR3DRcBEaKRFD4vMr9zy4pbCZ3wBVwT8rBLwJaBjPK
			hex!["9e2fdb5afc527563afced6f4bbf3ee0b0917c5f88edb1a15288c69d5916f6d6e"].into(),
			// 5EAQkhVywtAofQf2tfFQztAB48vs1v2JqHmNBuyaBiqw5Eyc
			hex!["5cd25da206a3165b5b65a8dec838016968758c49ead751cf90387ec3178dc14b"].unchecked_into(),
			// 5FAxyg6b9eZ6xue9UDhfU3kdfKnw84hK86LdSEtGog7QFBdD
			hex!["897b12ec6f2fd26173112ac5ea935d8961aa5979ceba5cce89b96e04e70ead49"].unchecked_into(),
			// 5DwRGGZ8AXJoucTUd5qK6ZcfMBg4sDAf8W6SoPuZj75u23ue
			hex!["52e9e44800a82d6c1a8aa8bbaa21b5cdb3be3d6b9e1298a5694192a9e6558b49"].unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5CfptqCCc5Y5xnqVSz8FQNKHXnmBPfRotrPJL1FbXH6MLNxg
		"1ac9475ee6d6446eaa9657cd6b1bbef5c94b041b308dfcd59bad0f97ef86b849"
	]
	.into();
	//c<AccountId>, <Balnance>
	let mut endowed_accounts: Vec<(sp_runtime::AccountId32, Balance)> = vec![];
	endowed_accounts.push((hex!["984cd803c81167d49c5af6149d3e55f567b73fd9109251223dc567e628681f49"].into(), INITIAL_COLLATOR_STAKING)); //Stash
	endowed_accounts.push((hex!["9e2fdb5afc527563afced6f4bbf3ee0b0917c5f88edb1a15288c69d5916f6d6e"].into(), INITIAL_COLLATOR_STAKING)); //Controle
	endowed_accounts.push((hex!["5cd25da206a3165b5b65a8dec838016968758c49ead751cf90387ec3178dc14b"].into(), INITIAL_COLLATOR_STAKING)); //Base
	endowed_accounts.push((hex!["897b12ec6f2fd26173112ac5ea935d8961aa5979ceba5cce89b96e04e70ead49"].into(), INITIAL_COLLATOR_STAKING)); //Granpar
	endowed_accounts.push((hex!["52e9e44800a82d6c1a8aa8bbaa21b5cdb3be3d6b9e1298a5694192a9e6558b49"].into(), INITIAL_COLLATOR_STAKING)); //Inline

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
