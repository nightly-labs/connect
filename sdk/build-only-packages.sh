# connect base
cd ./apps/base
pnpm build
# ui packages
cd ../../packages/qr-codes
pnpm build
cd ../modal
pnpm build

# connect networks
cd ../solana
pnpm build
cd ../sui
pnpm build

# selector packages
cd ../selector-base
pnpm build
cd ../selector-solana
pnpm build
cd ../selector-sui
pnpm build
