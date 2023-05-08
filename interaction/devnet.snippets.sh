PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/devnet.pem"

FUND_TOKEN_ID="USDC-8d4068"
FUND_TOKEN_ID_HEX="0x$(echo -n ${FUND_TOKEN_ID} | xxd -p -u | tr -d '\n')"
FUND_TOKEN_ID_ONLY_HEX="$(echo -n ${FUND_TOKEN_ID} | xxd -p -u | tr -d '\n')"

WRAP_TOKEN_ID="WEGLD-d7c6bb"
WRAP_TOKEN_ID_HEX="0x$(echo -n ${WRAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"
WRAP_TOKEN_ID_ONLY_HEX="$(echo -n ${WRAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"

FIRST_FEE_AMOUNT=2000000000                 # $2000 USDC
SECOND_FEE_AMOUNT=100000000000               # $100,000 USDC

CALLER_ADDRESS="erd149axj8feledcw7zck5f3ecwrncgd0gemcr9q69yxqlk0zvnl5zvs065jqu"
CALLER_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${CALLER_ADDRESS})"

WRAP_ADDRESS="erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh"
WRAP_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${WRAP_ADDRESS})"

SWAP_ADDRESS="erd1qqqqqqqqqqqqqpgqq67uv84ma3cekpa55l4l68ajzhq8qm3u0n4s20ecvx"
SWAP_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${SWAP_ADDRESS})"

# commission fee
BURN_ADDRESS="erd148cska3cdzseyyydq98v6vs626p86p99ztwnk7t7x6ddw9jge6zsuzvw5f"
BURN_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${BURN_ADDRESS})"

NFT_STAKING_ADDRESS="erd1q2jkcccfxhf7ulf9q5up9yu5dn59nv2uyuq78hg6cqcsrjhnqyvsq0y8su"
NFT_STAKING_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${NFT_STAKING_ADDRESS})"

VITAL_STAKING_ADDRESS="erd184622f2erl23fnulhzdemcaar7s0ve6yudektstl7xdxgsal6etqtu68dr"
VITAL_STAKING_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${VITAL_STAKING_ADDRESS})"

PROJECT_ADDRESS="erd1yfd0tu464av0n7dtvcflttwqudes4anv3mpac8kkeyz8yafdfphqus4xmr"
PROJECT_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${PROJECT_ADDRESS})"

PROJECT_LIVE_TIME=3888000

CASHBACK_PERCENTAGE=30      # 0.3%
VITAL_TOKEN_ID="VITAL-058fd5"
VITAL_TOKEN_ID_HEX="0x$(echo -n ${VITAL_TOKEN_ID} | xxd -p -u | tr -d '\n')"
VITAL_TOKEN_ID_ONLY_HEX="$(echo -n ${VITAL_TOKEN_ID} | xxd -p -u | tr -d '\n')"

USDC_TOKEN_RATE=1000000
VITAL_TOKEN_RATE=27777000000000000000000

################################################
ADDRESS=$(mxpy data load --key=address-devnet)
TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
################################################

deploy() {
    mxpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=200000000 \
    --arguments ${FUND_TOKEN_ID_HEX} ${FIRST_FEE_AMOUNT} ${SECOND_FEE_AMOUNT} ${WRAP_ADDRESS_HEX} ${SWAP_ADDRESS_HEX} ${WRAP_TOKEN_ID_HEX} ${BURN_ADDRESS_HEX} ${NFT_STAKING_ADDRESS_HEX} ${VITAL_STAKING_ADDRESS_HEX} ${PROJECT_ADDRESS_HEX} ${CASHBACK_PERCENTAGE} ${VITAL_TOKEN_ID_HEX} ${USDC_TOKEN_RATE} ${VITAL_TOKEN_RATE} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    TRANSACTION=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

UPGRADE_ADDRESS="erd1qqqqqqqqqqqqqpgqjlgtkfh45s63d587z9yw0rrmcvxc5vh35zvs03w6pz"
UPGRADE_ADDRESS_ONLY_HEX="$(mxpy wallet bech32 --decode ${UPGRADE_ADDRESS})"

upgrade() {
    mxpy --verbose contract upgrade ${UPGRADE_ADDRESS_ONLY_HEX} --project=${PROJECT} --recall-nonce --pem=${WALLET} --gas-limit=200000000 --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --arguments ${FUND_TOKEN_ID_HEX} ${FIRST_FEE_AMOUNT} ${SECOND_FEE_AMOUNT} ${WRAP_ADDRESS_HEX} ${SWAP_ADDRESS_HEX} ${WRAP_TOKEN_ID_HEX} ${BURN_ADDRESS_HEX} ${NFT_STAKING_ADDRESS_HEX} ${VITAL_STAKING_ADDRESS_HEX} ${PROJECT_ADDRESS_HEX} ${CASHBACK_PERCENTAGE} ${VITAL_TOKEN_ID_HEX} ${USDC_TOKEN_RATE} ${VITAL_TOKEN_RATE} --metadata-payable --metadata-payable-by-sc
}

PROJECT_NAME="PROJECT"
PROJECT_NAME_HEX="0x$(echo -n ${PROJECT_NAME} | xxd -p -u | tr -d '\n')"
PROJECT_CATEGORY="NEW"
PROJECT_CATEGORY_HEX="0x$(echo -n ${PROJECT_CATEGORY} | xxd -p -u | tr -d '\n')"
PROJECT_OWNER_NAME="ABC"
PROJECT_OWNER_NAME_HEX="0x$(echo -n ${PROJECT_OWNER_NAME} | xxd -p -u | tr -d '\n')"
PROJECT_GOAL=1000000000000000000000
PROJECT_UNIQUE_ID="12345"
PROJECT_UNIQUE_ID_HEX="0x$(echo -n ${PROJECT_UNIQUE_ID} | xxd -p -u | tr -d '\n')"
PROJECT_DEADLINE=2591000

createProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --function="createProject" \
    --arguments ${PROJECT_NAME_HEX} ${PROJECT_CATEGORY_HEX} ${PROJECT_OWNER_NAME_HEX} ${PROJECT_UNIQUE_ID_HEX} 0 0 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

PROJECT_ID=1
PROJECT_NEW_NAME="NEW PROJECT"
PROJECT_NEW_NAME_HEX="0x$(echo -n ${PROJECT_NEW_NAME} | xxd -p -u | tr -d '\n')"
PROJECT_PHOTO="NEW"
PROJECT_PHOTO_HEX="0x$(echo -n ${PROJECT_PHOTO} | xxd -p -u | tr -d '\n')"
PROJECT_DESCRIPTION="ABC"
PROJECT_DESCRIPTION_HEX="0x$(echo -n ${PROJECT_DESCRIPTION} | xxd -p -u | tr -d '\n')"

modifyProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="modifyProject" \
    --arguments ${PROJECT_ID} ${PROJECT_NEW_NAME_HEX} ${PROJECT_PHOTO_HEX} ${PROJECT_DESCRIPTION_HEX} 0 0 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

IDENTIFY_HIDE=true;


PARTICIPATE="participateProject"
PARTICIPATE_ONLY_HEX="$(echo -n ${PARTICIPATE} | xxd -p -u | tr -d '\n')"

participateProject() {
    mxpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=60000000 \
    --data="ESDTTransfer@${WRAP_TOKEN_ID_ONLY_HEX}@016345785d8a0000@${PARTICIPATE_ONLY_HEX}@01@00" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdrawFunds() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --function="withdrawFunds" \
    --arguments ${PROJECT_ID} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# devnet
# EGLD, erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh
# USDC-8d4068, erd1qqqqqqqqqqqqqpgqq67uv84ma3cekpa55l4l68ajzhq8qm3u0n4s20ecvx
# UTK-2eabf7,  erd1qqqqqqqqqqqqqpgqq67uv84ma3cekpa55l4l68ajzhq8qm3u0n4s20ecvx
# ASH-4ce444,  erd1qqqqqqqqqqqqqpgq53wlytsnh0g5hauxsx5fyf40eafzz9w00n4sswvfwq

# mainnet
# EGLD, erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3
# USDC-c76f1f, erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq
# UTK-2f80e9,  erd1qqqqqqqqqqqqqpgq0lzzvt2faev4upyf586tg38s84d7zsaj2jpsglugga

# ITHEUM-df6f26  erd1qqqqqqqqqqqqqpgqpmud7t8uprrxzgu8eq2mtkl08kesflj62jps9j8dyh
# BHAT-c1fde3    erd1qqqqqqqqqqqqqpgqp32ecg03fyxgt2pf2fsxyg4knvhfvtgz2jps6rx6gf
# AERO-458bbf    erd1qqqqqqqqqqqqqpgqzjctu8xrgn8jmfp503tajjvzz2zq60v92jpsslkh5a
# ZPAY-247875    erd1qqqqqqqqqqqqqpgqtjhhs49h0q2ncld64l3thtk7s7yuw47v2jps8emt0v
# MEX-455c57     erd1qqqqqqqqqqqqqpgqa0fsfshnff4n76jhcye6k7uvd7qacsq42jpsp6shh2
# PLATA-9ba6c3   erd1qqqqqqqqqqqqqpgqy8ufy6h4uyxzjsxe0kzfszsjd7myphzz2jpsrf7hkv
# EFFORT-a13513  erd1qqqqqqqqqqqqqpgqtga4sxnc462aeqdpxxajs8k7pr64ltkh2jps3ewwyw
# ISET-84e55e    erd1qqqqqqqqqqqqqpgqh3c7mg0dfe5z0t3468wwp08xz3ex25ah2jpsv4pcdl
# RIDE-7d18e9    erd1qqqqqqqqqqqqqpgqav09xenkuqsdyeyy5evqyhuusvu4gl3t2jpss57g8x
# VITAL-ab7917   erd1qqqqqqqqqqqqqpgq3ahw8fctzfnwgvq2g4hjsqzkkvgl9ksr2jps646dnj

SWAP_TOKEN_ID="ASH-4ce444"
SWAP_TOKEN_ID_HEX="0x$(echo -n ${SWAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"
SWAP_TOKEN_ID_ONLY_HEX="$(echo -n ${SWAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"
SWAP_TOKEN_ADDRESS="erd1qqqqqqqqqqqqqpgq53wlytsnh0g5hauxsx5fyf40eafzz9w00n4sswvfwq"
SWAP_TOKEN_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${SWAP_TOKEN_ADDRESS})"

setSwapTokenAddress() {
    mxpy --verbose contract call ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setSwapTokenAddress" \
    --arguments ${SWAP_TOKEN_ID_HEX} ${SWAP_TOKEN_ADDRESS_HEX} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

NEW_ADDRESS="erd149axj8feledcw7zck5f3ecwrncgd0gemcr9q69yxqlk0zvnl5zvs065jqu"
NEW_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${NEW_ADDRESS})"
transferProjectOwnership() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="transferProjectOwnership" \
    --arguments ${PROJECT_ID} ${NEW_ADDRESS_HEX} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

addProjectUniqueIds() {
    mxpy --verbose contract call ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --function="addProjectUniqueIds" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# view
getProject() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getProject" --arguments ${PROJECT_ID}
}

getProjects() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getProjects"
}

getOwnerProjects() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getOwnerProjects" --arguments ${CALLER_ADDRESS_HEX}
}

getParticipateAddresses() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getParticipateAddresses" --arguments ${PROJECT_ID}
}

getDonatedAmount() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getDonatedAmount" --arguments ${PROJECT_ID} ${CALLER_ADDRESS_HEX}
}

getProjectCollectedAmount() {
    mxpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getProjectCollectedAmount" --arguments ${PROJECT_ID}
}

# owner
inactivateProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="inactivateProject" \
    --arguments ${PROJECT_ID} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

activateProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="activateProject" \
    --arguments ${PROJECT_ID} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

UNIQUE_ID="6d3d14ab"
UNIQUE_ID_HEX="0x$(echo -n ${UNIQUE_ID} | xxd -p -u | tr -d '\n')"
PROJECT_VERIFY=1

setProjectVerifiedByUniqueId() {
    mxpy --verbose contract call ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=60000000 \
    --function="setProjectVerifiedByUniqueId" \
    --arguments ${UNIQUE_ID_HEX} ${PROJECT_VERIFY} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

NEW_LIVE_DATETIME=600

setProjectLiveDatetime() {
    mxpy --verbose contract call ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setProjectLiveDatetime" \
    --arguments ${NEW_LIVE_DATETIME} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

OLD_VITAL_TOKEN_ID="VITAL-bc0917"
OLD_VITAL_TOKEN_ID_HEX="0x$(echo -n ${OLD_VITAL_TOKEN_ID} | xxd -p -u | tr -d '\n')"
OLD_VITAL_TOKEN_ID_ONLY_HEX="$(echo -n ${OLD_VITAL_TOKEN_ID} | xxd -p -u | tr -d '\n')"

withdraw() {
    mxpy --verbose contract call ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --arguments ${OLD_VITAL_TOKEN_ID_HEX} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

NEW_ADDRESS="erd1r0s6ss90ktgtk6eytdskpkndl6ke4y75t0c8fq2mj7zumyncmtrsx9ud2u"
NEW_ADDRESS_ONLY_HEX="$(mxpy wallet bech32 --decode ${NEW_ADDRESS})"

transferOwnerShip() {
    mxpy --verbose tx new --receiver ${UPGRADE_ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --data="ChangeOwnerAddress@${NEW_ADDRESS_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

SWAP_AMOUNT=50000000000000000
swap() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=60000000 \
    --function="swap" \
    --arguments ${SWAP_AMOUNT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --function="createProject" \
    --arguments ${PROJECT_NAME_HEX} ${PROJECT_CATEGORY_HEX} ${PROJECT_OWNER_NAME_HEX} ${PROJECT_UNIQUE_ID_HEX} ${PROJECT_DEADLINE} ${PROJECT_GOAL} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

PROJECT_ID=1
PROJECT_NEW_NAME="NEW PROJECT"
PROJECT_NEW_NAME_HEX="0x$(echo -n ${PROJECT_NEW_NAME} | xxd -p -u | tr -d '\n')"
PROJECT_PHOTO="NEW"
PROJECT_PHOTO_HEX="0x$(echo -n ${PROJECT_PHOTO} | xxd -p -u | tr -d '\n')"
PROJECT_DESCRIPTION="ABC"
PROJECT_DESCRIPTION_HEX="0x$(echo -n ${PROJECT_DESCRIPTION} | xxd -p -u | tr -d '\n')"

modifyProject() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="modifyProject" \
    --arguments ${PROJECT_ID} ${PROJECT_NEW_NAME_HEX} ${PROJECT_PHOTO_HEX} ${PROJECT_DESCRIPTION_HEX} 0 0 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
