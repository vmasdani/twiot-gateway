# TWIoT Gateway
Third-world IoT gateway written in Rust which consists of
  - MQTT router
  - Smart garden (WIP)
  - Energy monitor (Not progress yet)

### Steps to compile
1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Install dependencies
```
# Ubuntu/debian
sudo apt install libsqlite3-dev
```
3. Add `.env` file containing
```
DATABASE_URL=db.sqlite3
```
4. Compile
```
cargo build
```

### Cross compile for Orange Pi Zero
1. Install cross
```
cargo install cross
```
2. Start docker
```
systemctl start docker
```
3. Compile to opi zero target
```
cross build --release --target armv7-unknown-linux-gnueabihf
```
4. Files will be located in `target/`
