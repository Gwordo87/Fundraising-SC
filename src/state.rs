elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Project<M: ManagedTypeApi> {
    pub project_id: u32,
    pub project_unique_id: ManagedBuffer<M>,
    pub project_name: ManagedBuffer<M>,
    pub project_category: ManagedBuffer<M>,
    pub project_photo: ManagedBuffer<M>,
    pub project_description: ManagedBuffer<M>,
    pub project_description1: ManagedBuffer<M>,
    pub project_description2: ManagedBuffer<M>,
    pub project_owner_name: ManagedBuffer<M>,
    pub project_owner_address: ManagedAddress<M>,
    pub project_goal: BigUint<M>,
    pub project_deadline: u64,
    pub project_participation_numbers: u32,
    pub project_collected_amount: BigUint<M>,
    pub project_status: bool,
    pub project_create_datetime: u64,
    pub project_withdrawn_amount: BigUint<M>,
    pub project_remains_amount: BigUint<M>,
    pub project_verified: bool,
    pub project_last_tx_datetime: u64,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct ProjectTransactions<M: ManagedTypeApi> {
    pub transaction_id: u32,
    pub datetime: u64,
    pub action: u32,
    pub name: ManagedBuffer<M>,
    pub address: ManagedAddress<M>,
    pub amount_in_usdc: BigUint<M>,
    pub withdrawn_fee_for_burn: BigUint<M>,
    pub withdrawn_fee_for_nft: BigUint<M>,
    pub withdrawn_fee_for_vital: BigUint<M>,
    pub withdrawn_fee_for_project: BigUint<M>,
    pub transaction_hash: ManagedBuffer<M>,
    pub identify_hide: bool,
    pub comment: ManagedBuffer<M>,
}