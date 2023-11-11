//! Low-level types used throughout the Substrate stencil code.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic,
};
/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

pub mod staking {
    use sp_runtime::Perbill;
    use super::Balance;
	pub const UNIT: Balance = 100_000_000;
    pub const MIN_VALIDATOR_BOND: u128 = 25_000 * UNIT;
    pub const MIN_NOMINATOR_BOND: u128 = 100 * UNIT;
    pub const MAX_NOMINATORS_REWARDED_PER_VALIDATOR: u32 = 1024;
    pub const YEARLY_INFLATION: Balance = 30_000_000 * UNIT;
    pub const VALIDATOR_REWARD: Perbill = Perbill::from_percent(90);

    pub fn era_payout(miliseconds_per_era: u64) -> (Balance, Balance) {
        // Milliseconds per year for the Julian year (365.25 days).
        const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;

        let portion = Perbill::from_rational(miliseconds_per_era, MILLISECONDS_PER_YEAR);
        let total_payout = portion * YEARLY_INFLATION;
        let validators_payout = VALIDATOR_REWARD * total_payout;
        let rest = total_payout - validators_payout;

        (validators_payout, rest)
    }

    /// Macro for making a default implementation of non-self methods from given class.
    ///
    /// As an input it expects list of tuples of form
    ///
    /// `(method_name(arg1: type1, arg2: type2, ...), class_name, return_type)`
    ///
    /// where
    ///   * `method_name`is a wrapee method,
    ///   * `arg1: type1, arg2: type,...`is a list of arguments and will be passed as is, can be empty
    ///   * `class_name`is a class that has non-self `method-name`,ie symbol `class_name::method_name` exists,
    ///   * `return_type` is type returned from `method_name`
    /// Example
    /// ```ignore
    /// wrap_methods!(
    ///     (bond(), SubstrateStakingWeights, Weight),
    ///     (bond_extra(), SubstrateStakingWeights, Weight)
    /// );
    /// ```
    #[macro_export]
    macro_rules! wrap_methods {
        ($(($wrapped_method:ident( $($arg_name:ident: $argument_type:ty), *), $wrapped_class:ty, $return_type:ty)), *) => {
            $(
                fn $wrapped_method($($arg_name: $argument_type), *) -> $return_type {
                    <$wrapped_class>::$wrapped_method($($arg_name), *)
                }
            )*
        };
    }
}