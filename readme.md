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
3. Compile to Orange Pi (armv7 muslhf)
```sh
./run.py prod release # or ./run.py prod debug if you wan to compile faster
```
4. A file called `release.zip` will be located in `dist/`. You can move the file in your development machine using `scp` in gateway shell, for example `scp valianmasdani@192.168.1.22:~/codes/rust/twiot-gateway/dist/release.zip` 

### Configuration
1. Config WiFi and static IP via `nmtui`. Enable `i2c0` and `i2c1` in `armbian-config`
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
# /etc/nginx/sites-available/default. Maybe a good thing to backup the default file first

...
location / {
  proxy_pass http://localhost:8080;

  # the rest of "location /" body is empty
}
...
```

4. Write systemd daemon for twiot-gateway
```sh
# /etc/systemd/system/twiot-gateway.service

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
```sh
systemctl enable twiot-gateway
systemctl start twiot-gateway 
```

6. And you're done. You can now