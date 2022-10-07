echo "Changing directory to rust"
cd rust

echo "Checking and building with release profile"
cargo check --verbose
cargo build --release --verbose

echo "Testing with release profile"
cargo test --release --verbose

echo "Building module from crate as web target at pkg"
wasm-pack build --target web

echo "Clearing all files inside svelte/src/wasm-game-of-life"
rm -rf ../svelte/src/wasm-game-of-life/*

echo "Copying wasm-game-of-life from rust/pkg to svelte/src/wasm-game-of-life"
cp -r pkg/* ../svelte/src/wasm-game-of-life

echo "Changing directory to svelte"
cd ../svelte

echo "Installing dependencies"
npm i

echo "Building svelte"
npm run build

while getopts ":p" opt; do
  case $opt in
    p)
      echo "-p was triggered!" >&2
      echo "Previewing svelte"
      npm run preview
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      ;;
  esac
done