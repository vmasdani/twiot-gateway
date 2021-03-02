# TWIoT Gateway
Third-world IoT gateway written in Rust which consists of
  - Smart garden
  - Energy monitor (No progress yet)

### Steps to compile
1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Add `.env` file containing
```
DATABASE_URL=twiot-gateway.sqlite3
```
3. Compile
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
3. Compile to opi armv7 muslhf
```
./run.py prod
```
4. Files will be located in `dist/`

### Configuration
1. Install requirements
```sh
apt install mosquitto nginx
```
2. Set timezone
```sh
timedatectl set-timezone Asia/Jakarta
```
3. Set nginx proxy pass
```sh
# /etc/nginx/sites-available/default

...
location / {
  proxy_pass http://localhost:8080;
}
...
```

4. Write systemd daemon for twiot-gateway
```
# /etc/systemd/system/twiot-gateway

[Unit]
Description=TWIoT gateway
After=network.target

[Service]
Type=Simple
ExecStart=/root/twiot-gateway
WorkingDirectory=/root
User=root

[Install]
WantedBy=multi-user.target
```

5. Enable systemd
```
systemctl enable twiot-gateway
```