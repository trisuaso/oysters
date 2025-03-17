cargo install oysters oysters_cli
sudo mkdir /etc/oysters

cd /etc/oysters
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/.config/config.example.toml
sudo mkdir .config
sudo mv config.example.toml .config/config.toml

cd /etc/systemd/system
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-dump.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-dump.timer
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-scan.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-scan.timer

sudo ln -s ~/.cargo/bin/oysters /usr/bin/oysters
sudo ln -s ~/.cargo/bin/oysters-cli /usr/bin/oysters-cli

sudo systemctl daemon-reload

sudo systemctl enable --now oysters
echo "Oysters service enabled"

sudo systemctl enable oysters-scan.timer
echo "Oysters scan timer enabled"

sudo systemctl enable oysters-dump.timer
echo "Oysters dump timer enabled"
