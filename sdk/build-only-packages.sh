# connect base
cd ./apps/base
pnpm build

# connect networks
cd ../../packages/solana
pnpm build
cd ../sui
pnpm build
cd ../aptos
pnpm build
cd ../polkadot
pnpm build
# ui packages
cd ../qr-codes
pnpm build
cd ../modal
pnpm build

# selector packages
cd ../selector-base
pnpm build
cd ../selector-solana
pnpm build
cd ../selector-sui
pnpm build
cd ../selector-polkadot
pnpm build
cd ../selector-aptos
pnpm build