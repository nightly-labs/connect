# connect base
cd ./apps/base
pnpm build

# connect networks
cd ../solana
pnpm build
cd ../sui
pnpm build

# ui packages
cd ../../packages/qr-codes
pnpm build
cd ../modal
pnpm build
