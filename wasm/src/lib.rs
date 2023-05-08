// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           83
// Async Callback (empty):               1
// Total number of exported functions:  85

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    dapp_sc
    (
        createProject
        modifyProject
        participateProject
        withdrawFunds
        transferProjectOwnership
        inactivateProjectByProjectOwner
        getCashBackAmount
        getProject
        getProjectByUniqueId
        getProjects
        getOwnerProjects
        getProjectTransactions
        addProjectUniqueIds
        inactivateProject
        activateProject
        setTreasuryBurnAddress
        setTreasuryNftStakingAddress
        setTreasuryVitalStakingAddress
        setTreasuryProjectAddress
        setWrapContractAddress
        setSwapContractAddress
        setProjectFundsFirstFeeAmount
        setProjectFundsSecondFeeAmount
        setProjectVerified
        setProjectVerifiedByUniqueId
        setVitalTokenIdentifier
        setCashbackPercentage
        setSwapTokenAddress
        setUsdcTokenRate
        setVitalTokenRate
        getFundTokenIdentifier
        getLastProjectId
        getProjectIds
        getProjectUniqueIds
        getProjectUniqueId
        getProjectName
        getProjectCategory
        getProjectPhoto
        getProjectDescription
        getProjectOwnerName
        getProjectOwnerAddress
        getProjectGoal
        getProjectDeadline
        getProjectStatus
        getProjectCreateDatetime
        getProjectCollectedAmount
        getProjectWithdrawnAmount
        getProjectVerified
        getParticipationAddresses
        getLastTransactionId
        getTransactionIds
        getTransactionDateTime
        getTransactionActionType
        getTransactionName
        getTransactionAddress
        getTransactionAmount
        getTransactionHash
        getTransactionIdentifyHide
        getTransactionWithdrawnFeeForBurn
        getTransactionWithdrawnFeeForNft
        getTransactionWithdrawnFeeForVital
        getTransactionWithdrawnFeeForProject
        getTransactionComment
        getProjectFundsFirstFeeAmount
        getProjectFundsSecondFeeAmount
        getProjectFundsFirstFee
        getProjectFundsSecondFee
        getProjectFundsThirdFee
        getWrapContractAddress
        getSwapContractAddress
        getWrapTokenIdentifier
        getTreasuryBurnAddress
        getTreasuryNftStakingAddress
        getTreasuryVitalStakingAddress
        getTreasuryProjectAddress
        getVitalTokenIdentifier
        getCashbackPercentage
        getSwapTokenIdentifiers
        getSwapTokenAddresses
        getUsdcTokenRate
        getVitalTokenRate
        getProjectDescription1
        getProjectDescription2
    )
}

elrond_wasm_node::wasm_empty_callback! {}