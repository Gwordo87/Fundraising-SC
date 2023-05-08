#![no_std]

elrond_wasm::imports!();

mod storage;
mod state;
mod swap;

use state::Project;
use state::ProjectTransactions;

const TOTAL_PERCENTAGE: u64 = 10_000;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Dapp:
    storage::StorageModule
{   
    #[init]
    fn init(
        &self, 
        fund_token_identifier: TokenIdentifier, 
        project_funds_first_fee_amount: BigUint,
        project_funds_second_fee_amount: BigUint,
        wrap_contract_address: ManagedAddress,
        swap_contract_address: ManagedAddress,
        wrap_token_identifier: TokenIdentifier,
        treasury_burn_address: ManagedAddress,
        treasury_nft_staking_address: ManagedAddress,
        treasury_vital_staking_address: ManagedAddress,
        treasury_project_address: ManagedAddress,
        cashback_percentage: u64,
        vital_token_identifier: TokenIdentifier,
        usdc_token_rate: BigUint,
        vital_token_rate: BigUint,
    ) {
        self.fund_token_identifier().set(fund_token_identifier);

        self.project_funds_first_fee_amount().set(project_funds_first_fee_amount);
        self.project_funds_second_fee_amount().set(project_funds_second_fee_amount);

        // fee : 3%
        self.project_funds_first_fee(1u32).set(60u32);
        self.project_funds_first_fee(2u32).set(60u32);
        self.project_funds_first_fee(3u32).set(60u32);
        self.project_funds_first_fee(4u32).set(120u32);

        // fee : 2.5%
        self.project_funds_second_fee(1u32).set(50u32);
        self.project_funds_second_fee(2u32).set(50u32);
        self.project_funds_second_fee(3u32).set(50u32);
        self.project_funds_second_fee(4u32).set(100u32);

        // fee : 2%
        self.project_funds_third_fee(1u32).set(40u32);
        self.project_funds_third_fee(2u32).set(40u32);
        self.project_funds_third_fee(3u32).set(40u32);
        self.project_funds_third_fee(4u32).set(80u32);

        self.wrap_contract_address().set(wrap_contract_address);
        self.swap_contract_address().set(swap_contract_address);
        self.wrap_token_identifier().set(wrap_token_identifier);
        self.treasury_burn_address().set(treasury_burn_address);
        self.treasury_nft_staking_address().set(treasury_nft_staking_address);
        self.treasury_vital_staking_address().set(treasury_vital_staking_address);
        self.treasury_project_address().set(treasury_project_address);

        self.vital_token_identifier().set(vital_token_identifier);
        self.cashback_percentage().set(cashback_percentage);

        self.usdc_token_rate().set(usdc_token_rate);
        self.vital_token_rate().set(vital_token_rate);
    }

    // endpoint
    #[endpoint(createProject)]
    fn create_project(
        &self, 
        project_name: ManagedBuffer,
        project_category: ManagedBuffer,
        project_owner_name: ManagedBuffer,
        project_unique_id: ManagedBuffer,
        project_deadline: OptionalValue<u64>,
        project_goal: OptionalValue<BigUint>,
    ) {

        require!(
            !self.project_unique_ids().contains(&project_unique_id), 
            "project_unique_id already exist"
        );

        let new_project_id = self.last_project_id().get() + 1;

        self.project_unique_ids().insert(project_unique_id.clone());
        self.project_ids().insert(new_project_id);
        self.last_project_id().set(new_project_id);

        self.project_name(new_project_id).set(project_name);
        self.project_category(new_project_id).set(project_category);
        self.project_owner_name(new_project_id).set(project_owner_name);
        self.project_owner_address(new_project_id).set(self.blockchain().get_caller());
        self.project_unique_id(new_project_id).set(project_unique_id);
        self.project_create_datetime(new_project_id).set(self.blockchain().get_block_timestamp());

        match project_deadline {
            OptionalValue::Some(v) => self.project_deadline(new_project_id).set(v),
            OptionalValue::None => self.project_deadline(new_project_id).set(0)
        }

        match project_goal {
            OptionalValue::Some(v) => self.project_goal(new_project_id).set(v),
            OptionalValue::None => self.project_goal(new_project_id).set(BigUint::zero())
        }

        self.project_status(new_project_id).set(true); // active
    }

    #[endpoint(modifyProject)]
    fn modify_project(
        &self,
        project_id: u32,
        project_name: ManagedBuffer,
        project_photo: ManagedBuffer,
        project_category: ManagedBuffer,
        project_description: ManagedBuffer,
        project_description1: ManagedBuffer,
        project_description2: ManagedBuffer,
        project_deadline: OptionalValue<u64>,
        project_goal: OptionalValue<BigUint>,
    ) {
        self.require_project_id(project_id);
        self.require_project_owner(project_id);
        self.require_project_status(project_id);

        if self.project_name(project_id).get() != project_name {
            self.project_name(project_id).set(project_name);
        }

        if self.project_photo(project_id).get() != project_photo {
            self.project_photo(project_id).set(project_photo);
        }
        
        if self.project_category(project_id).get() != project_category {
            self.project_category(project_id).set(project_category);
        }
        
        if self.project_description(project_id).get() != project_description {
            self.project_description(project_id).set(project_description);
        }

        if self.project_description1(project_id).get() != project_description1 {
            self.project_description1(project_id).set(project_description1);
        }

        if self.project_description2(project_id).get() != project_description2 {
            self.project_description2(project_id).set(project_description2);
        }

        match project_deadline {
            OptionalValue::Some(v) => self.project_deadline(project_id).set(v),
            OptionalValue::None => self.project_deadline(project_id).set(0)
        }

        match project_goal {
            OptionalValue::Some(v) => self.project_goal(project_id).set(v),
            OptionalValue::None => self.project_goal(project_id).set(BigUint::zero())
        }
    }

    #[payable("*")]
    #[endpoint(participateProject)]
    fn participate_project(
        &self, 
        #[payment_token] payment_token_id: EgldOrEsdtTokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
        project_id: u32, 
        identify_hide: bool,
        opt_user_name: OptionalValue<ManagedBuffer>,
        opt_comment: OptionalValue<ManagedBuffer>,
    ) {

        self.require_project_id(project_id);
        self.require_project_status(project_id);
        
        let caller = self.blockchain().get_caller();

        require!(
            payment_amount > BigUint::zero(),
            "the payment must be more than 0"
        );

        let user_name = match opt_user_name {
            OptionalValue::Some(v) => v,
            OptionalValue::None => ManagedBuffer::new()
        };

        let comment = match opt_comment {
            OptionalValue::Some(v) => v,
            OptionalValue::None => ManagedBuffer::new()
        };

        let new_transaction_id = self.last_transaction_id(project_id).get() + 1;

        self.transaction_ids(project_id).insert(new_transaction_id);
        self.last_transaction_id(project_id).set(new_transaction_id);

        self.transaction_hash(project_id, new_transaction_id).set(self.blockchain().get_tx_hash().as_managed_buffer());

        let amount = self.swap_into_fund_token(payment_token_id, payment_amount);

        self.project_collected_amount(project_id).update(|v| *v += & amount);

        self.transaction_date_time(project_id, new_transaction_id).set(self.blockchain().get_block_timestamp());
        self.transaction_action_type(project_id, new_transaction_id).set(0u32);
        self.transaction_name(project_id, new_transaction_id).set(user_name);
        self.transaction_address(project_id, new_transaction_id).set(caller.clone());
        self.transaction_amount(project_id, new_transaction_id).set(amount.clone());
        self.transaction_identify_hide(project_id, new_transaction_id).set(identify_hide);
        self.transaction_comment(project_id, new_transaction_id).set(comment);

        if !self.project_participation_addresses(project_id).contains(&caller) {
            self.project_participation_addresses(project_id).insert(caller);
        }

        // send VITAL 2%
        self.send_cashback(amount);
    }

    #[endpoint(withdrawFunds)]
    fn withdraw_funds(&self, project_id: u32) {
        self.require_project_id(project_id);
        self.require_project_owner(project_id);
        self.require_project_status(project_id);

        let token_id = self.fund_token_identifier().get();
        // let token_id = TokenIdentifier::egld();
        let mut token_amount = self.project_collected_amount(project_id).get() - self.project_withdrawn_amount(project_id).get(); 
        self.project_withdrawn_amount(project_id).update(|v| *v += & token_amount);

        require!(
            token_amount > BigUint::zero(), 
            "balance is zero"
        );      

        require!(
            token_amount <= self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(token_id.clone()), 0), 
            "sc balance error"
        );
        
        // fee
        let mut burn_amount = BigUint::zero();
        let mut nft_staking_amount = BigUint::zero();
        let mut vital_staking_amount = BigUint::zero();
        let mut project_amount = BigUint::zero();
        if token_amount <= self.project_funds_first_fee_amount().get() {
            // 3% = 300
            burn_amount = token_amount.clone() * BigUint::from(self.project_funds_first_fee(1u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            nft_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_first_fee(2u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            vital_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_first_fee(3u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            project_amount = token_amount.clone() * BigUint::from(self.project_funds_first_fee(4u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            
            token_amount -= burn_amount.clone() + nft_staking_amount.clone() + vital_staking_amount.clone() + project_amount.clone();
            // token_amount -= token_amount.clone() * BigUint::from(self.project_funds_first_fee().get()) / BigUint::from(TOTAL_PERCENTAGE);
        } else if token_amount > self.project_funds_first_fee_amount().get() && token_amount < self.project_funds_second_fee_amount().get() {
            // 2.5% = 250
            burn_amount = token_amount.clone() * BigUint::from(self.project_funds_second_fee(1u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            nft_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_second_fee(2u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            vital_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_second_fee(3u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            project_amount = token_amount.clone() * BigUint::from(self.project_funds_second_fee(4u32).get()) / BigUint::from(TOTAL_PERCENTAGE);

            token_amount -= burn_amount.clone() + nft_staking_amount.clone() + vital_staking_amount.clone() + project_amount.clone();
            // token_amount -= token_amount.clone() * BigUint::from(self.project_funds_second_fee().get()) / BigUint::from(TOTAL_PERCENTAGE);
        } else if token_amount >= self.project_funds_second_fee_amount().get() {
            // 2% = 200
            burn_amount = token_amount.clone() * BigUint::from(self.project_funds_third_fee(1u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            nft_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_third_fee(2u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            vital_staking_amount = token_amount.clone() * BigUint::from(self.project_funds_third_fee(3u32).get()) / BigUint::from(TOTAL_PERCENTAGE);
            project_amount = token_amount.clone() * BigUint::from(self.project_funds_third_fee(4u32).get()) / BigUint::from(TOTAL_PERCENTAGE);

            token_amount -= burn_amount.clone() + nft_staking_amount.clone() + vital_staking_amount.clone() + project_amount.clone();
            // token_amount -= token_amount.clone() * BigUint::from(self.project_funds_third_fee().get()) / BigUint::from(TOTAL_PERCENTAGE);
        }

        let new_transaction_id = self.last_transaction_id(project_id).get() + 1;
        self.transaction_ids(project_id).insert(new_transaction_id);
        self.last_transaction_id(project_id).set(new_transaction_id);

        self.transaction_hash(project_id, new_transaction_id).set(self.blockchain().get_tx_hash().as_managed_buffer());
        self.transaction_date_time(project_id, new_transaction_id).set(self.blockchain().get_block_timestamp());
        self.transaction_action_type(project_id, new_transaction_id).set(1u32);
        // self.transaction_name(project_id, new_transaction_id).set(ManagedBuffer::new("Owner"));
        self.transaction_address(project_id, new_transaction_id).set(self.blockchain().get_caller());
        self.transaction_amount(project_id, new_transaction_id).set(token_amount.clone());
        self.transaction_withdrawn_fee_for_burn(project_id, new_transaction_id).set(burn_amount.clone());
        self.transaction_withdrawn_fee_for_nft(project_id, new_transaction_id).set(nft_staking_amount.clone());
        self.transaction_withdrawn_fee_for_vital(project_id, new_transaction_id).set(vital_staking_amount.clone());
        self.transaction_withdrawn_fee_for_project(project_id, new_transaction_id).set(project_amount.clone());

        self.project_status(project_id).set(false); // inactive

        self.send().direct_esdt(&self.treasury_burn_address().get(), &token_id, 0, &burn_amount);
        self.send().direct_esdt(&self.treasury_nft_staking_address().get(), &token_id, 0, &nft_staking_amount);
        self.send().direct_esdt(&self.treasury_vital_staking_address().get(), &token_id, 0, &vital_staking_amount);
        self.send().direct_esdt(&self.treasury_project_address().get(), &token_id, 0, &project_amount);

        self.send().direct_esdt(&self.blockchain().get_caller(), &token_id, 0, &token_amount);
    }

    #[endpoint(transferProjectOwnership)]
    fn transfer_project_ownership(&self, project_id: u32, new_address: ManagedAddress) {
        self.require_project_id(project_id);
        self.require_project_owner(project_id);
        self.require_project_status(project_id);

        self.project_owner_address(project_id).set(new_address);
    }

    #[endpoint(inactivateProjectByProjectOwner)]
    fn inactivate_project_by_project_owner(&self, project_id: u32) {
        self.require_project_id(project_id);
        self.require_project_owner(project_id);
        self.require_project_status(project_id);

        self.project_status(project_id).set(false); // inactive
    }

    // private
    fn require_project_id(&self, project_id: u32) {
        require!(self.project_ids().contains(&project_id), "project_id does not exist");
    }

    fn require_project_owner(&self, project_id: u32) {
        require!(self.project_owner_address(project_id).get() == self.blockchain().get_caller(), "is not project_owner");
    }

    fn require_project_status(&self, project_id: u32) {
        require!(self.project_status(project_id).get(), "inactive the project");
    }

    fn swap_into_fund_token(
        &self, 
        token_id: EgldOrEsdtTokenIdentifier, 
        amount: BigUint
    ) -> BigUint {
        let mut last_payment = EsdtTokenPayment::new(self.wrap_token_identifier().get(), 0, amount.clone());
        if token_id == EgldOrEsdtTokenIdentifier::egld() {
            // wrapping EGLD -> WrappedEGLD
            self.swap_proxy(self.wrap_contract_address().get())
                .wrap_egld()
                .with_egld_transfer(amount)
                .execute_on_dest_context::<()>();
        } else if token_id.clone().unwrap_esdt() == self.fund_token_identifier().get() {
            return amount;
        } else if token_id.clone().unwrap_esdt() != self.wrap_token_identifier().get() {
            let payment_token_id = token_id.unwrap_esdt();
            require!(
                !self.swap_token_addresses(payment_token_id.clone()).is_empty(),
                "swap address is empty"
            );
            // Swap ESDT -> WEGLD
            last_payment = EsdtTokenPayment::new(payment_token_id.clone(), 0, amount);
            last_payment = self.swap_proxy(self.swap_token_addresses(payment_token_id.clone()).get())
                .swap_tokens_fixed_input(
                    self.wrap_token_identifier().get(),
                    BigUint::from(1u32),
                )
                .with_esdt_transfer(last_payment)
                .execute_on_dest_context();
        }

        // WEGLD -> USDC
        last_payment = self.swap_proxy(self.swap_contract_address().get())
            .swap_tokens_fixed_input(
                self.fund_token_identifier().get(),
                BigUint::from(1u32),
            )
            .with_esdt_transfer(last_payment)
            .execute_on_dest_context();
        
        return last_payment.amount;
    }

    fn send_cashback(&self, usdc_amount: BigUint) {
        let cashback_amount_in_vital = self.get_cashback_amount(usdc_amount);
        if cashback_amount_in_vital <= self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(self.vital_token_identifier().get()), 0) {
            self.send().direct_esdt(&self.blockchain().get_caller(), &self.vital_token_identifier().get(), 0, &cashback_amount_in_vital);
        }
        
    }
    
    // view
    #[view(getCashBackAmount)]
    fn get_cashback_amount(&self, usdc_amount: BigUint) -> BigUint {
        usdc_amount * self.vital_token_rate().get() * self.cashback_percentage().get() / self.usdc_token_rate().get() / TOTAL_PERCENTAGE
    }

    #[view(getProject)]
    fn get_project(&self, project_id: u32) -> Project<Self::Api> {

        let participate_count = self.project_participation_addresses(project_id).len() as u32;
        let mut project_last_tx_datetime = self.project_create_datetime(project_id).get();
        if participate_count > 0u32 {
            project_last_tx_datetime = self.transaction_date_time(project_id, self.last_transaction_id(project_id).get()).get();
        }

        let project = Project {
            project_id: project_id,
            project_unique_id: self.project_unique_id(project_id).get(),
            project_name: self.project_name(project_id).get(),
            project_category:self.project_category(project_id).get(),
            project_photo:self.project_photo(project_id).get(),
            project_description:self.project_description(project_id).get(),
            project_description1:self.project_description1(project_id).get(),
            project_description2:self.project_description2(project_id).get(),
            project_owner_name:self.project_owner_name(project_id).get(),
            project_owner_address: self.project_owner_address(project_id).get(),
            project_goal: self.project_goal(project_id).get(),
            project_deadline: self.project_deadline(project_id).get(),
            project_participation_numbers: participate_count,
            project_collected_amount: self.project_collected_amount(project_id).get(),
            project_status: self.project_status(project_id).get(),
            project_create_datetime: self.project_create_datetime(project_id).get(),
            project_withdrawn_amount: self.project_withdrawn_amount(project_id).get(),
            project_remains_amount: self.project_collected_amount(project_id).get() - self.project_withdrawn_amount(project_id).get(),
            project_verified: self.project_verified(project_id).get(),
            project_last_tx_datetime: project_last_tx_datetime,
        };

        project
    }

    #[view(getProjectByUniqueId)]
    fn get_project_by_unique_id(&self, project_unique_id: ManagedBuffer) -> Project<Self::Api> {
        let mut project_id_by_unique_id = 0u32;
        for project_id in self.project_ids().iter() {
            if project_unique_id == self.project_unique_id(project_id).get() {
                project_id_by_unique_id = project_id;
                break;
            }
        }

        require!(
            project_id_by_unique_id > 0, 
            "project_id not exist"
        );

        self.get_project(project_id_by_unique_id)
    }

    #[view(getProjects)]
    fn get_projects(&self) -> MultiValueEncoded<Project<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        for project_id in self.project_ids().iter() {
            if self.project_status(project_id).get() {
                items_vec.push(self.get_project(project_id));
            }
        }

        items_vec
    }

    #[view(getOwnerProjects)]
    fn get_owner_projects(&self, user_address: ManagedAddress) -> MultiValueEncoded<Project<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        for project_id in self.project_ids().iter() {
            let project = self.get_project(project_id);
            if project.project_owner_address == user_address {
                items_vec.push(project);
            }            
        }

        items_vec
    }

    #[view(getProjectTransactions)]
    fn project_transactions(&self, project_id: u32) -> MultiValueEncoded<ProjectTransactions<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        for transaction_id in self.transaction_ids(project_id).iter() {
            let transaction = ProjectTransactions {
                transaction_id: transaction_id,
                datetime: self.transaction_date_time(project_id, transaction_id).get(),
                action: self.transaction_action_type(project_id, transaction_id).get(),
                name: self.transaction_name(project_id, transaction_id).get(),
                address: self.transaction_address(project_id, transaction_id).get(),
                amount_in_usdc: self.transaction_amount(project_id, transaction_id).get(),
                withdrawn_fee_for_burn: self.transaction_withdrawn_fee_for_burn(project_id, transaction_id).get(),
                withdrawn_fee_for_nft: self.transaction_withdrawn_fee_for_nft(project_id, transaction_id).get(),
                withdrawn_fee_for_vital: self.transaction_withdrawn_fee_for_vital(project_id, transaction_id).get(),
                withdrawn_fee_for_project: self.transaction_withdrawn_fee_for_project(project_id, transaction_id).get(),
                transaction_hash: self.transaction_hash(project_id, transaction_id).get(),
                identify_hide: self.transaction_identify_hide(project_id, transaction_id).get(),
                comment: self.transaction_comment(project_id, transaction_id).get(),
            };

            items_vec.push(transaction);
        }

        items_vec
    }

    // owner
    #[only_owner]
    #[endpoint(addProjectUniqueIds)]
    fn add_project_unique_ids(&self) {
        for project_id in self.project_ids().iter() {
            let project = self.get_project(project_id);
            if !self.project_unique_ids().contains(&project.project_unique_id) {
                self.project_unique_ids().insert(project.project_unique_id);
            }
        }
    }

    #[only_owner]
    #[endpoint(inactivateProject)]
    fn inactivate_project(&self, project_id: u32) {
        self.project_status(project_id).set(false); // inactive
    }

    #[only_owner]
    #[endpoint(activateProject)]
    fn activate_project(&self, project_id: u32) {
        self.project_status(project_id).set(true); // active
    }

    #[only_owner]
    #[endpoint(setTreasuryBurnAddress)]
    fn set_treasury_burn_address(&self, treasury_burn_address: ManagedAddress) {
        self.treasury_burn_address().set(treasury_burn_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryNftStakingAddress)]
    fn set_treasury_nft_staking_address(&self, treasury_nft_staking_address: ManagedAddress) {
        self.treasury_nft_staking_address().set(treasury_nft_staking_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryVitalStakingAddress)]
    fn set_treasury_vital_staking_address(&self, treasury_vital_staking_address: ManagedAddress) {
        self.treasury_vital_staking_address().set(treasury_vital_staking_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryProjectAddress)]
    fn set_treasury_project_address(&self, treasury_project_address: ManagedAddress) {
        self.treasury_project_address().set(treasury_project_address);
    }

    #[only_owner]
    #[endpoint(setWrapContractAddress)]
    fn set_wrap_contract_address(&self, wrap_contract_address: ManagedAddress) {
        self.wrap_contract_address().set(wrap_contract_address);
    }

    #[only_owner]
    #[endpoint(setSwapContractAddress)]
    fn set_swap_contract_address(&self, swap_contract_address: ManagedAddress) {
        self.swap_contract_address().set(swap_contract_address);
    }

    // fee
    #[only_owner]
    #[endpoint(setProjectFundsFirstFeeAmount)]
    fn set_project_funds_first_fee_amount(&self, project_funds_first_fee_amount: BigUint) {
        self.project_funds_first_fee_amount().set(project_funds_first_fee_amount);
    }

    #[only_owner]
    #[endpoint(setProjectFundsSecondFeeAmount)]
    fn set_project_funds_second_fee_amount(&self, project_funds_second_fee_amount: BigUint) {
        self.project_funds_second_fee_amount().set(project_funds_second_fee_amount);
    }

    #[only_owner]
    #[endpoint(setProjectVerified)]
    fn set_project_verified(&self, project_id: u32, project_verified: bool) {
        self.project_verified(project_id).set(project_verified);
    }

    #[only_owner]
    #[endpoint(setProjectVerifiedByUniqueId)]
    fn set_project_verified_by_unique_id(&self, project_unique_id: ManagedBuffer, project_verified: bool) {
        for project_id in self.project_ids().iter() {
            if project_unique_id == self.project_unique_id(project_id).get() {
                self.project_verified(project_id).set(project_verified);       
            }
        }
    }

    #[only_owner]
    #[endpoint(setVitalTokenIdentifier)]
    fn set_vital_token_identifier(&self, vital_token_identifier: TokenIdentifier) {
        self.vital_token_identifier().set(vital_token_identifier);
    }

    #[only_owner]
    #[endpoint(setCashbackPercentage)]
    fn set_cashback_percentage(&self, cashback_percentage: u64) {
        self.cashback_percentage().set(cashback_percentage);
    }

    #[only_owner]
    #[endpoint(setSwapTokenAddress)]
    fn set_swap_token_address(&self, swap_token_identifier: TokenIdentifier, swap_token_address: ManagedAddress) {
        if !self.swap_token_identifiers().contains(&swap_token_identifier.clone()) {
            self.swap_token_identifiers().insert(swap_token_identifier.clone());
        }
        self.swap_token_addresses(swap_token_identifier.clone()).set(swap_token_address);
    }

    #[only_owner]
    #[endpoint(setUsdcTokenRate)]
    fn set_usdc_token_rate(&self, usdc_token_rate: BigUint) {
        self.usdc_token_rate().set(usdc_token_rate);
    }

    #[only_owner]
    #[endpoint(setVitalTokenRate)]
    fn set_vital_token_rate(&self, vital_token_rate: BigUint) {
        self.vital_token_rate().set(vital_token_rate);
    }

    // proxy
    #[proxy]
    fn swap_proxy(&self, sc_address: ManagedAddress) -> swap::Proxy<Self::Api>;

}
