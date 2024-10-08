use codec::{
    Decode,
    Encode,
    MaxEncodedLen,
};
use frame_support::{
    dispatch::RawOrigin,
    traits::fungibles::{
        approvals::{
            Inspect as AllowanceInspect,
            Mutate as AllowanceMutate,
        },
        Inspect,
        InspectMetadata,
        Transfer
        
    },
};
use pallet_assets::{
    self
};
use pallet_contracts::chain_extension::{
    ChainExtension,
    Environment,
    Ext,
    InitState,
    RetVal,
    SysConfig,
};
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{
    traits::{
        Saturating,
        StaticLookup,
        Zero,
    },
    DispatchError,
};

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22BalanceOfInput<AssetId, AccountId> {
    asset_id: AssetId,
    owner: AccountId,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22AllowanceInput<AssetId, AccountId> {
    asset_id: AssetId,
    owner: AccountId,
    spender: AccountId,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22TransferInput<AssetId, AccountId, Balance> {
    asset_id: AssetId,
    to: AccountId,
    value: Balance,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22TransferFromInput<AssetId, AccountId, Balance> {
    asset_id: AssetId,
    from: AccountId,
    to: AccountId,
    value: Balance,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22ApproveInput<AssetId, AccountId, Balance> {
    asset_id: AssetId,
    spender: AccountId,
    value: Balance,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22BurnInput<AssetId, AccountId, Balance> {
    asset_id: AssetId,
    from: AccountId,
    value: Balance,
}

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct Psp22MintInput<AssetId, AccountId, Balance> {
    asset_id: AssetId,
    to: AccountId,
    value: Balance,
}

#[derive(Default)]
pub struct Psp22Extension;

fn convert_err(err_msg: &'static str) -> impl FnOnce(DispatchError) -> DispatchError {
    move |_err| {     
        DispatchError::Other(err_msg)
    }
}

/// We're using enums for function IDs because contrary to raw u16 it enables
/// exhaustive matching, which results in cleaner code.
enum FuncId {
    Metadata(Metadata),
    Query(Query),
    Transfer,
    TransferFrom,
    Approve,
    IncreaseAllowance,
    DecreaseAllowance,
    Burn,
    Mint,
}

#[derive(Debug)]
enum Metadata {
    Name,
    Symbol,
    Decimals,
}

#[derive(Debug)]
enum Query {
    TotalSupply,
    BalanceOf,
    Allowance,
}

impl TryFrom<u16> for FuncId {
    type Error = DispatchError;

    fn try_from(func_id: u16) -> Result<Self, Self::Error> {
        let id = match func_id {
            // Note: We use the first two bytes of PSP22 interface selectors as function IDs,
            // While we can use anything here, it makes sense from a convention perspective.
            0x3d26 => Self::Metadata(Metadata::Name),
            0x3420 => Self::Metadata(Metadata::Symbol),
            0x7271 => Self::Metadata(Metadata::Decimals),
            0x162d => Self::Query(Query::TotalSupply),
            0x6568 => Self::Query(Query::BalanceOf),
            0x4d47 => Self::Query(Query::Allowance),
            0xdb20 => Self::Transfer,
            0x54b3 => Self::TransferFrom,
            0xb20f => Self::Approve,
            0x96d6 => Self::IncreaseAllowance,
            0xfecb => Self::DecreaseAllowance,
            0x9e55 => Self::Burn,
            0x6bba => Self::Mint,
            _ => {
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        };

        Ok(id)
    }
}

fn metadata<T, E>(
    func_id: Metadata,
    env: Environment<E, InitState>,
) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let asset_id = env.read_as()?;
    let result = match func_id {
        Metadata::Name => {
            <pallet_assets::Pallet<T> as InspectMetadata<T::AccountId>>::name(&asset_id)
                .encode()
        }
        Metadata::Symbol => {
            <pallet_assets::Pallet<T> as InspectMetadata<T::AccountId>>::symbol(&asset_id)
                .encode()
        }
        Metadata::Decimals => {
            <pallet_assets::Pallet<T> as InspectMetadata<T::AccountId>>::decimals(
                &asset_id,
            )
            .encode()
        }
    };    
    env.write(&result, false, None)
        .map_err(convert_err("ChainExtension failed to call PSP22Metadata"))
}

fn query<T, E>(
    func_id: Query,
    env: Environment<E, InitState>,
) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let result = match func_id {
        Query::TotalSupply => {
            let asset_id = env.read_as()?;
            <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::total_issuance(asset_id)
        }
        Query::BalanceOf => {
            let input: Psp22BalanceOfInput<T::AssetId, T::AccountId> = env.read_as()?;
            <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::balance(
                input.asset_id,
                &input.owner,
            )
        }
        Query::Allowance => {
            let input: Psp22AllowanceInput<T::AssetId, T::AccountId> = env.read_as()?;
            <pallet_assets::Pallet<T> as AllowanceInspect<T::AccountId>>::allowance(
                input.asset_id,
                &input.owner,
                &input.spender,
            )
        }
    }
    .encode();
    
    env.write(&result, false, None)
        .map_err(convert_err("ChainExtension failed to call PSP22 query"))
}

fn transfer<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();

    let input: Psp22TransferInput<T::AssetId, T::AccountId, T::Balance> =
        env.read_as()?;
    let sender = env.ext().caller();

    <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
        input.asset_id,
        sender,
        &input.to,
        input.value,
        true,
    )
    .map_err(convert_err("ChainExtension failed to call transfer"))?;
    

    Ok(())
}

fn transfer_from<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();    
   

    let input: Psp22TransferFromInput<T::AssetId, T::AccountId, T::Balance> =
        env.read_as()?;
    let spender = env.ext().caller();

    let result =
        <pallet_assets::Pallet<T> as AllowanceMutate<T::AccountId>>::transfer_from(
            input.asset_id,
            &input.from,
            spender,
            &input.to,
            input.value,
        );
   
    result.map_err(convert_err("ChainExtension failed to call transfer_from"))
}

fn approve<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
      

    let input: Psp22ApproveInput<T::AssetId, T::AccountId, T::Balance> = env.read_as()?;
    let owner = env.ext().caller();

    let result = <pallet_assets::Pallet<T> as AllowanceMutate<T::AccountId>>::approve(
        input.asset_id,
        owner,
        &input.spender,
        input.value,
    );
   
    result.map_err(convert_err("ChainExtension failed to call approve"))
}

fn decrease_allowance<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let input: Psp22ApproveInput<T::AssetId, T::AccountId, T::Balance> = env.read_as()?;
    if input.value.is_zero() {
        return Ok(())
    }
   

    let owner = env.ext().caller();
    let mut allowance =
        <pallet_assets::Pallet<T> as AllowanceInspect<T::AccountId>>::allowance(
            input.asset_id,
            owner,
            &input.spender,
        );
    <pallet_assets::Pallet<T>>::cancel_approval(
        RawOrigin::Signed(owner.clone()).into(),
        input.asset_id.into(),
        T::Lookup::unlookup(input.spender.clone()),
    )
    .map_err(convert_err(
        "ChainExtension failed to call decrease_allowance",
    ))?;
    allowance.saturating_reduce(input.value);
    if allowance.is_zero() {       
        return Ok(())
    }
    <pallet_assets::Pallet<T> as AllowanceMutate<T::AccountId>>::approve(
        input.asset_id,
        owner,
        &input.spender,
        allowance,
    )
    .map_err(convert_err(
        "ChainExtension failed to call decrease_allowance",
    ))?;
   

    Ok(())
}
fn burn<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let input: Psp22BurnInput<T::AssetId, T::AccountId, T::Balance> = env.read_as()?;
    let caller = env.ext().caller();

    <pallet_assets::Pallet<T>>::burn(
        RawOrigin::Signed(caller.clone()).into(),
        input.asset_id.into(),
        T::Lookup::unlookup(input.from.clone()),
        input.value,
    ).map_err(convert_err("ChainExtension failed to call burn"))?;

    Ok(())
}

fn mint<T, E>(env: Environment<E, InitState>) -> Result<(), DispatchError>
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
    E: Ext<T = T>,
{
    let mut env = env.buf_in_buf_out();
    let input: Psp22MintInput<T::AssetId, T::AccountId, T::Balance> = env.read_as()?;
    let caller = env.ext().caller();

    <pallet_assets::Pallet<T>>::mint(
        RawOrigin::Signed(caller.clone()).into(),
        input.asset_id.into(),
        T::Lookup::unlookup(input.to.clone()),
        input.value,
    ).map_err(convert_err("ChainExtension failed to call mint"))?;

    Ok(())
}

impl<T> ChainExtension<T> for Psp22Extension
where
    T: pallet_assets::Config + pallet_contracts::Config,
    <T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
{
    fn call<E: Ext>(
        &mut self,
        env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        E: Ext<T = T>,
        <E::T as SysConfig>::AccountId:
            UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let func_id = FuncId::try_from(env.func_id())?;
        match func_id {
            FuncId::Metadata(func_id) => metadata::<T, E>(func_id, env)?,
            FuncId::Query(func_id) => query::<T, E>(func_id, env)?,
            FuncId::Transfer => transfer::<T, E>(env)?,
            FuncId::TransferFrom => transfer_from::<T, E>(env)?,
            // This is a bit of a shortcut. It was made because the documentation
            // for Mutate::approve does not specify the result of subsequent calls.
            FuncId::Approve | FuncId::IncreaseAllowance => approve::<T, E>(env)?,
            FuncId::DecreaseAllowance => decrease_allowance(env)?,
            FuncId::Burn => burn(env)?,
            FuncId::Mint => mint(env)?,
            
        }

        Ok(RetVal::Converging(0))
    }
}