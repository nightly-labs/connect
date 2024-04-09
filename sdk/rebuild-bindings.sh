echo 'Navigating to project root...'
cd ..

echo 'Deleting existing bindings directories...'
rm -rf server/bindings
rm -rf database/bindings
rm -rf sdk/bindings

echo 'Running cargo tests to recreate bindings...'
cargo test

echo 'Creating new bindings directory in sdk...'
mkdir -p sdk/bindings

echo 'Copying bindings from server and database to sdk...'
cp -r server/bindings/* sdk/bindings/
cp -r database/bindings/* sdk/bindings/

echo 'Generating index.ts for bindings...'
cd sdk/bindings

rm -f index.ts

for file in *.ts; do
    if [ "$file" != "index.ts" ]; then
        echo "export * from './${file%.*}';" >> index.ts
    fi
done

cd ../..

echo 'Operation completed successfully.'