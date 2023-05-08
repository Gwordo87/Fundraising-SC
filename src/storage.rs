elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {

    #[view(getFundTokenIdentifier)]
    #[storage_mapper("fund_token_identifier")]
    fn fund_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLastProjectId)]
    #[storage_mapper("last_project_id")]
    fn last_project_id(&self) -> SingleValueMapper<u32>;

    #[view(getProjectIds)]
    #[storage_mapper("project_ids")]
    fn project_ids(&self) -> SetMapper<u32>;

    #[view(getProjectUniqueIds)]
    #[storage_mapper("project_unique_ids")]
    fn project_unique_ids(&self) -> SetMapper<ManagedBuffer>;

    #[view(getProjectUniqueId)]
    #[storage_mapper("project_unique_id")]
    fn project_unique_id(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectName)]
    #[storage_mapper("project_name")]
    fn project_name(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectCategory)]
    #[storage_mapper("project_category")]
    fn project_category(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectPhoto)]
    #[storage_mapper("project_photo")]
    fn project_photo(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectDescription)]
    #[storage_mapper("project_description")]
    fn project_description(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectOwnerName)]
    #[storage_mapper("project_owner_name")]
    fn project_owner_name(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectOwnerAddress)]
    #[storage_mapper("project_owner_address")]
    fn project_owner_address(&self, project_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[view(getProjectGoal)]
    #[storage_mapper("project_goal")]
    fn project_goal(&self, project_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getProjectDeadline)]
    #[storage_mapper("project_deadline")]
    fn project_deadline(&self, project_id: u32) -> SingleValueMapper<u64>;

    #[view(getProjectStatus)]
    #[storage_mapper("project_status")]
    fn project_status(&self, project_id: u32) -> SingleValueMapper<bool>;

    #[view(getProjectCreateDatetime)]
    #[storage_mapper("project_create_datetime")]
    fn project_create_datetime(&self, project_id: u32) -> SingleValueMapper<u64>;

    #[view(getProjectCollectedAmount)]
    #[storage_mapper("project_collected_amount")]
    fn project_collected_amount(&self, project_id: u32) -> SingleValueMapper<BigUint>;


    /////////////////////////////////////
    #[view(getProjectWithdrawnAmount)]
    #[storage_mapper("project_withdrawn_amount")]
    fn project_withdrawn_amount(&self, project_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getProjectVerified)]
    #[storage_mapper("project_verified")]
    fn project_verified(&self, project_id: u32) -> SingleValueMapper<bool>;

    #[view(getParticipationAddresses)]
    #[storage_mapper("project_participation_addresses")]
    fn project_participation_addresses(&self, project_id: u32) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getLastTransactionId)]
    #[storage_mapper("last_transaction_id")]
    fn last_transaction_id(&self, project_id: u32) -> SingleValueMapper<u32>;

    #[view(getTransactionIds)]
    #[storage_mapper("transaction_ids")]
    fn transaction_ids(&self, project_id: u32) -> SetMapper<u32>;

    #[view(getTransactionDateTime)]
    #[storage_mapper("transaction_date_time")]
    fn transaction_date_time(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<u64>;

    #[view(getTransactionActionType)]
    #[storage_mapper("transaction_action_type")]
    fn transaction_action_type(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<u32>;

    #[view(getTransactionName)]
    #[storage_mapper("transaction_name")]
    fn transaction_name(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getTransactionAddress)]
    #[storage_mapper("transaction_address")]
    fn transaction_address(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[view(getTransactionAmount)]
    #[storage_mapper("transaction_amount")]
    fn transaction_amount(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getTransactionHash)]
    #[storage_mapper("transaction_hash")]
    fn transaction_hash(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getTransactionIdentifyHide)]
    #[storage_mapper("transaction_identify_hide")]
    fn transaction_identify_hide(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<bool>;

    #[view(getTransactionWithdrawnFeeForBurn)]
    #[storage_mapper("transaction_withdrawn_fee_for_burn")]
    fn transaction_withdrawn_fee_for_burn(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getTransactionWithdrawnFeeForNft)]
    #[storage_mapper("transaction_withdrawn_fee_for_nft")]
    fn transaction_withdrawn_fee_for_nft(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getTransactionWithdrawnFeeForVital)]
    #[storage_mapper("transaction_withdrawn_fee_for_vital")]
    fn transaction_withdrawn_fee_for_vital(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getTransactionWithdrawnFeeForProject)]
    #[storage_mapper("transaction_withdrawn_fee_for_project")]
    fn transaction_withdrawn_fee_for_project(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<BigUint>;

    #[view(getTransactionComment)]
    #[storage_mapper("transaction_comment")]
    fn transaction_comment(&self, project_id: u32, transaction_id: u32) -> SingleValueMapper<ManagedBuffer>;
    ///////////////////////////////////////////


    // fee
    #[view(getProjectFundsFirstFeeAmount)]
    #[storage_mapper("project_funds_first_fee_amount")]
    fn project_funds_first_fee_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getProjectFundsSecondFeeAmount)]
    #[storage_mapper("project_funds_second_fee_amount")]
    fn project_funds_second_fee_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getProjectFundsFirstFee)]
    #[storage_mapper("project_funds_first_fee")]
    fn project_funds_first_fee(&self, index: u32) -> SingleValueMapper<u32>;

    #[view(getProjectFundsSecondFee)]
    #[storage_mapper("project_funds_second_fee")]
    fn project_funds_second_fee(&self, index: u32) -> SingleValueMapper<u32>;

    #[view(getProjectFundsThirdFee)]
    #[storage_mapper("project_funds_third_fee")]
    fn project_funds_third_fee(&self, index: u32) -> SingleValueMapper<u32>;

    // proxy
    #[view(getWrapContractAddress)]
    #[storage_mapper("wrap_contract_address")]
    fn wrap_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
    
    #[view(getSwapContractAddress)]
    #[storage_mapper("swap_contract_address")]
    fn swap_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getWrapTokenIdentifier)]
    #[storage_mapper("wrap_token_identifier")]
    fn wrap_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    // treasury
    #[view(getTreasuryBurnAddress)]
    #[storage_mapper("treasury_burn_address")]
    fn treasury_burn_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTreasuryNftStakingAddress)]
    #[storage_mapper("treasury_nft_staking_address")]
    fn treasury_nft_staking_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTreasuryVitalStakingAddress)]
    #[storage_mapper("treasury_vital_staking_address")]
    fn treasury_vital_staking_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTreasuryProjectAddress)]
    #[storage_mapper("treasury_project_address")]
    fn treasury_project_address(&self) -> SingleValueMapper<ManagedAddress>;

    // VITAL price
    #[view(getVitalTokenIdentifier)]
    #[storage_mapper("vital_token_identifier")]
    fn vital_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCashbackPercentage)]
    #[storage_mapper("cashback_percentage")]
    fn cashback_percentage(&self) -> SingleValueMapper<u64>;

    #[view(getSwapTokenIdentifiers)]
    #[storage_mapper("swap_token_identifiers")]
    fn swap_token_identifiers(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[view(getSwapTokenAddresses)]
    #[storage_mapper("swap_token_addresses")]
    fn swap_token_addresses(&self, token_identifier: TokenIdentifier) -> SingleValueMapper<ManagedAddress>;

    //////////////////////////////////////////////
    #[view(getUsdcTokenRate)]
    #[storage_mapper("usdc_token_rate")]
    fn usdc_token_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getVitalTokenRate)]
    #[storage_mapper("vital_token_rate")]
    fn vital_token_rate(&self) -> SingleValueMapper<BigUint>;

    ////////////////////////////////////////////////
    #[view(getProjectDescription1)]
    #[storage_mapper("project_description1")]
    fn project_description1(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

    #[view(getProjectDescription2)]
    #[storage_mapper("project_description2")]
    fn project_description2(&self, project_id: u32) -> SingleValueMapper<ManagedBuffer>;

}