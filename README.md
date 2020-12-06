# ferrisflow

## Supported Protocols
- Netflow v5
- Netflow v9

## Supported Publisher
- JSON
- CSV

## Usage

### help
```sh
> cargo run -- --help
ferrisflow 0.1.0

USAGE:
    ferrisflow [FLAGS] [OPTIONS]

FLAGS:
    -c, --csv
        --help           Prints help information
    -h, --header-none
    -j, --json
        --netflow-v5
        --netflow-v9
        --print
    -V, --version        Prints version information

OPTIONS:
    -p, --port <port>     [default: 2055]
```

### Netflow v5 Collector

```
> cargo run -- -p 2055 --netflow-v5 --json
```

### Netflow v9 Collector

```
> cargo run -- -p 2055 --netflow-v9 --json
```

### Netflow v5, v9 Collector

```
> cargo run -- -p 2055 --netflow-v5 --netflow-v9 --json
```

## Custom Publisher

Publisher trait

```
pub trait Publisher: Send + Display {
    fn box_clone(&self) -> Box<dyn Publisher>;
    fn publish(&self, flowmessages: &Vec<FlowMessage>) -> Result<()>;
}
```