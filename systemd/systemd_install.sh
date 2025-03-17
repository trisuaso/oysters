cargo install oysters oysters_cli
sudo mkdir /etc/oysters

cd /etc/systemd/system
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-dump.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-dump.timer
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-scan.service
sudo wget https://raw.githubusercontent.com/trisuaso/oysters/refs/heads/master/systemd/oysters-scan.timer

sudo systemctl daemon-reload
sudo systemctl enable --now oysters
echo "Oysters service enabled"
