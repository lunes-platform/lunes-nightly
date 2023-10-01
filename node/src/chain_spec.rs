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
			// 5GwxVe4GAAhdgrrBwScAoCeEpKEREKM9VGL9zoZUL5Vuvz6j
			hex!["d807b931e48fd11ed532db5f9ebaed8907efd403cfb544f8ea06d73f46fd9c17"].into(),
			// 5FhASkzaTqBPBWGM71wnySrG9H1VWVgUmAEBKEb4Px28WFQ6
			hex!["a083098df44b0c289f7ae4eaccef610658411fbfc65cd78e056876950e718243"].into(),
			// 5EeZ97pBeNPXXNZe2vuemowsCqeSZQVyZmxTKVrmTyNUMejL
			hex!["724977fb5bf7e008fda248280074de06900eb52f17588299f08318b279c958a6"].unchecked_into(),
			// 5FNU5fBaZaKtDevwJTnysoEavj8o7QvpcPPnEaesq7Qb2Jjv
			hex!["9240be4f0ccd314f5924e783382c6b3b958a3f5e37b23de71a4d35f02732881b"].unchecked_into(),
			// 5G7GAMfM7ea6ZYRbcVVy8sSc8Z1Hrd2FhTJYP6diy2ZCGnWg
			hex!["b2e42c3f98fcf68e27f8e761f706ac450ec64e7af3fef1a8d46a7353e15a6176"].unchecked_into(),
			),
			(
			// 5EnozpXWkEYC4y7G3cC3gZsv8ihzzi8C2rjL5QqVcWXuTn8h
			hex!["789571de54135d535761c7da67b6c37c9730fb84a4216b3f96764612fcc00c79"].into(),
			// 5HCHVVrNvSC8tVVL1j4QybYGb3B4mGeAtt1LLmhAv4STiLhJ
			hex!["e2f51c4e493cc93393578efdf86a9f57c89de3de97ecfb3abdea2cd0623c9964"].into(),
			// 5E3C9Jq2dfggkyquz5J9sS7MR4RLSsqDFCBZL2pY89cKrCwB
			hex!["575132c0517188681bac5fc130dd3be16f699560a2cecccc76e46439407cd906"].unchecked_into(),
			// 5HiuLn2cVdRFmTDjKxodzNLvT4p6RyP2BuK6byaBkLXJDDNo
			hex!["fa4f2740e0c62d250104e5534b96f139bc885f72b4c2451626648e48df869e32"].unchecked_into(),
			// 5D7WdUD43k9DdXHEvZ9C7tvLhLYfEps2YuFGub1ksGmL3stg
			hex!["2e602c9f8a420256ea5df013de7e3ca03e11fbf96b388a121eef5b71ab364549"].unchecked_into(),
			),
			(
			// 5Dw3zSej9x9WM6xwT8DyfTx9D8ZXMChYEGzjxbkZH9J9FziB
			hex!["52a247e87aabdfe24665162d36438585b77d9f161010ea540794ff281cc7c438"].into(),
			// 5EZNoejHqn4xq7kScZ2UseSCnYs3YCotPG7n4Bt8D9qrB83S
			hex!["6e5672e493165a128cb50e50507a8db4b2df802abda57aeb7d2bd4310feb0e32"].into(),
			// 5H4MZ78DmeJZ7snVL7fF2sVvhvwmACuFhPo2e6VseG52SaDZ
			hex!["dce8d26b64983ae22ef637db258ef2b40c3217a5b7dc8ec1c04e8c6bec225c0b"].unchecked_into(),
			// 5FcK1A37XruBMfLS21pTcHRe3dZxYeHiZhKnXqrgBRQj3LHH
			hex!["9ccf9efbcb511425c93dc044308ee2150b37666884f9b529c98e121cee725264"].unchecked_into(),
			// 5FEoWFfLRg9Qkp46x7aBVxWkYoenG4matHAPUfdtjBZkxdjJ
			hex!["8c682ad5194505608b4628ebd5407aa483452f1568746f0da78a6bea58f9e54c"].unchecked_into(),
			),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5CDSekkWt3RG9MVda6LawudKNSNsyCapjpC18QqVRAdhV8kE
		"06aa05ce13315a2f7ac9e63c362c9408c3f6c6b7a7185414e669b48c5c65bbcd"
	]
	.into();
	//c<AccountId>, <Balnance>
	let mut endowed_accounts: Vec<(sp_runtime::AccountId32, Balance)> = vec![];
	endowed_accounts.push((hex!["0d3fe51599e66fe706caeb2a886698f095112288f6f72d4b773b67785bc76516"].into(), INITIAL_COLLATOR_STAKING)); //validator
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
