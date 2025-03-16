# 🦪 Oysters

_Oysters_ is a simple LRU key/value store which operates off an HTTP endpoint. Oysters allows you to persistantly store data by dumping it to an SQLite file.

## Usage

You can build Oysters using the following command:

```bash
just build
```

This will compile two binaries: `oysters` and `oysters-cli`.

### `oysters` binary

The `oysters` binary will start the server at port `5072` by default. This port can be changed in the config file:

```toml
# ./.config/config.toml
port = 5072
```

### `oysters-cli` binary

The `oysters-cli` binary allows you to interface with the server through a CLI.

For example, here's a simple demonstration of inserting a key, fetching the stored value, and then removing the key:

```bash
> oysters-cli insert a 1
> oysters-cli get a
1
> oysters-cli remove a
```

It can also perform more complex operations, such as filtering values by their key:

```bash
> oysters-cli insert a:a 1
> oysters-cli insert a:b 2
> oysters-cli insert b:a 3
> oysters-cli insert a:b:c 4
> oysters-cli insert a:b:d 5
> oysters-cli filter a:b:*
a:b:c = 4
a:b:d = 5
> oysters-cli dump # dump the in-memory map into an SQLite file, allowing us to store data persistantly
> oysters-cli scan # scan the in-memory map for keys accessed less than 7 days ago, allowing us to delete stale data
```
