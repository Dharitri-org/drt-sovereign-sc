dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait CommonFeeModule {
    fn require_caller_dct_safe(&self) {
        let caller = self.blockchain().get_caller();
        let dct_safe_address = self.dct_safe_address().get();
        require!(
            caller == dct_safe_address,
            "Only DCT Safe may call this SC"
        );
    }

    #[storage_mapper("dctSafeAddress")]
    fn dct_safe_address(&self) -> SingleValueMapper<ManagedAddress>;
}
