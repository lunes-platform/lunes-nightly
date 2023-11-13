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
			// 5ENh7Zq3KHCseVdo4KbF5ztSDK5GDSMUiwLT3E1rqS22svsy
			hex!["663061efaa2334649267572ad07bf9004e0343bccba8569fdab0bddf570a5249"].into(),
			// 5GLPJBeC3samwR2yEmhWak7wQpLuurJAgGfauNbwc9hbyzq7
			hex!["bce661a988687eb1c23dd0de55b982ad6fb9aaa43d89a1d444133aafb9fc4551"].into(),
			// 5Hj4VkbJgzfbMgYT7DB2i62DCsUgDcxKKhrJ2ozh5SHdF63u
			hex!["fa6df89070c009cb37e2baef97b1fb6c34e46618983bad35b307d16b89f57a57"].unchecked_into(),
			// 5HYUtYaYrQKrfJqueiqEWaoGvU7cPEFFxswScpoHgaUDiRdd
			hex!["f25c62b6981a74e4be39f79e3d7a164614ea8ee7323d32530fdd80f1c5be10e4"].unchecked_into(),
			// 5FNUfUv5vn7Vw7m7QVigaFCW2HJpL69HztF2UQcJcRFZTUwC
			hex!["9242b4db14213c82948b5f6e86759017accf80c0c7e190f0af0f11a08b210509"].unchecked_into(),
		),
		(
			// 5EUaAwztzTkBXqvgwh7VE13tQMVjkwBcwcuejcZDXMGrrTf5
			hex!["6aac7cb8ad6554a15672cb6be4e7fce3d98bb0c12acf4c88e68a71bcf3fdbc30"].into(),
			// 5C7jhE181ZSBQwiJRbSyTKFjV4X9b2Dn7Ebbu4f4C68EGvJM
			hex!["024febb30883667726ac3f7708a313ab568f297bb840d3cbc10913b6776d6252"].into(),
			// 5FS1kcp2e8pYndoLKo8uAsaYGWeZD1DybZDGLQy6RuG6UqVb
			hex!["94f51898dda2d6492edad1149b81d6f7ee71eeec661dfc7852a74a6d75480b7c"].unchecked_into(),
			// 5Cj69RXSKsmJdFyyRAhh74SoLEFKpxt5syE8Hra8RHq6Ct1a
			hex!["1d465b3e1a8cbd4eead242c27483708f2c6ed185c07ba27f6ad5560d7dfad341"].unchecked_into(),
			// 5Fxmrj4Ru7EigiEy1zsuzTABSLREfV7nEHybtf2hWcYuitfw
			hex!["ac6aee0e66235cd3c82b24c0f4248124a0d557236a5092357f9484f5341f7607"].unchecked_into(),
		),
		(
			// 5EWYJCEUbGS1PDucHRzz6BZcTfaLPdyRjXi1cZApie9VRo89
			hex!["6c2caa79eda8d94521bf988d7b8ac8a7e0182490f342ec62bfb5b5c17745be0b"].into(),
			// 5GnCqu6TqYvKoFvn5srHrnmzHGuWUxUNTLnagJLy6aF5UCnR
			hex!["d09792cb6a0df126fc6f011eb5c06f4ec9eb5f52de677a59afacceebc7c8000d"].into(),
			// 5FkBrW6mmnRa1shFrzDLUWi9tiTXdpKNvQwfYiNnExfi8hB7
			hex!["a2d18494091952ffb86f0820e53f2c192d2cbf87cdb774aeb60fac90ab393664"].unchecked_into(),
			// 5GE44QJZnTDQaM7EpSZFx3idhCbGe23FZPupc1DxBNijiykr
			hex!["b81226445c97abb8db94a3d46ac0c82ec9ecd216f864c9abd4a1427862d16e0d"].unchecked_into(),
			// 5CcwwMfoeMBw7XHmxVFP6CVSBmRCs1UtHcz6YnVV87NsJni2
			hex!["189740d557e029ecbd9852fd5987dec9034cd77e994cb18bf78ce5f90eb7394c"].unchecked_into(),
		),
		(
			// 5EFfg9yNY3P916zAhUNxQYqByZPawbSYb55q1VV56Q5hmzqi
			hex!["60d4d2d5638cfd111a3159ca4e9aa9efb5b841f5f06442bafcc242200c3ed544"].into(),
			// 5CyAtbaqyMwEZBWJJ3EvXD9hNuDmr73eDjBTsHMcmsrWAcDZ
			hex!["2803c3572733aa3af365160b8f9d5609b4c019a7e648a2a9b892e918adaca415"].into(),
			// 5CwK8TQUqNARmvjHGWs1KNtrj1HZ68ZDsy452opEMhzPid2r
			hex!["269901cb7f727e506d402b858622a08b0dd9203144abfb8364af21e523d95d4c"].unchecked_into(),
			// 5FjBHnZrwuF5mEePMA7srQQaAy9kdCjeziHxqC6GhBZsWoAA
			hex!["a20c5fa77833dc4d607b3416da5eaec272afc52117d3fcd600c4012822c8b670"].unchecked_into(),
			// 5FWt8u3FuDpvXH7wY2AneXg9NfZy2civyj86NnWs9i4ttySv
			hex!["98abaf91989e0ba739e064ef37af0fd88af7008e96725548437ddc2090ce013f"].unchecked_into(),
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
	endowed_accounts.push((hex!["663061efaa2334649267572ad07bf9004e0343bccba8569fdab0bddf570a5249"].into(), INITIAL_COLLATOR_STAKING)); 
	endowed_accounts.push((hex!["6aac7cb8ad6554a15672cb6be4e7fce3d98bb0c12acf4c88e68a71bcf3fdbc30"].into(), INITIAL_COLLATOR_STAKING)); 
	endowed_accounts.push((hex!["6c2caa79eda8d94521bf988d7b8ac8a7e0182490f342ec62bfb5b5c17745be0b"].into(), INITIAL_COLLATOR_STAKING)); 
	endowed_accounts.push((hex!["60d4d2d5638cfd111a3159ca4e9aa9efb5b841f5f06442bafcc242200c3ed544"].into(), INITIAL_COLLATOR_STAKING)); 

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
		.map(|x| (x.0.clone(), x.0.clone(), MIN_VALIDATOR_BOND, StakerStatus::Validator))
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
			minimum_validator_count: 4,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			min_validator_bond: MIN_VALIDATOR_BOND,
            min_nominator_bond: MIN_NOMINATOR_BOND,
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
