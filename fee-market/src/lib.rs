#![no_std]

dharitri_sc::imports!();

pub mod enable_fee;
pub mod fee_common;
pub mod fee_type;
pub mod price_aggregator;
pub mod subtract_fee;

#[dharitri_sc::contract]
pub trait FeeMarket:
    enable_fee::EnableFeeModule
    + fee_common::CommonFeeModule
    + fee_type::FeeTypeModule
    + subtract_fee::SubtractFeeModule
    + price_aggregator::PriceAggregatorModule
    + utils::UtilsModule
    + bls_signature::BlsSignatureModule
{
    #[init]
    fn init(&self, dct_safe_address: ManagedAddress, price_aggregator_address: ManagedAddress) {
        self.require_sc_address(&dct_safe_address);
        self.require_sc_address(&price_aggregator_address);

        self.dct_safe_address().set(dct_safe_address);
        self.price_aggregator_address().set(price_aggregator_address);
        self.fee_enabled().set(true);
    }

    #[endpoint]
    fn upgrade(&self) {}
}
