# TWIoT Gateway

Steps to compile:
1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Add `.env` file containing
```
DATABASE_URL=db.sqlite3
```
3. Compile
```
cargo build
```

Cross compile for Orange Pi Zero:
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
cross build --target armv7-unknown-linux-gnueabihf
```
4. Files will be located in `target/`
