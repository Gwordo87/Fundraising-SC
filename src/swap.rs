elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub type SwapTokensFixedInputResultType<BigUint> = EsdtTokenPayment<BigUint>;

#[elrond_wasm::proxy]
pub trait SwapContract {

    #[payable("EGLD")]
    #[endpoint(wrapEgld)]
    fn wrap_egld(&self);

    #[payable("*")]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
        &self,
        token_out: TokenIdentifier,
        amount_out_min: BigUint,
    ) -> SwapTokensFixedInputResultType<Self::Api>;
}