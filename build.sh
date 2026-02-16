sudo dnf install -y clang cmake gcc gcc-c++ make

sudo dnf install -y protobuf protobuf-devel protobuf-compiler

sudo dnf install -y libstdc++-devel

sudo dnf install -y python3-devel

sudo dnf install -y binutils-devel

cargo clean

cargo build --verbose

cargo run --verbose