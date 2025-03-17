# systemd services/timers

Each service/timer requires that you have `oysters-cli` (client binary) at least aliased at `/usr/bin/oysters-cli`.

`oysters.service` requires that `oysters` (server binary) at least be aliased at `/usr/bin/oysters`. It also requires that the `/etc/oysters` directory exists.

All files go in `/etc/systemd/system`.
