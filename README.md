# IODD Engine

A Rust toolkit for parsing [IO-Link](https://io-link.com) Device Descriptions (IODD 1.1), decoding/encoding process data, and interacting with IFM IoTCore masters.

IODD Engine turns raw hexadecimal sensor data into human-readable values with names, units, and descriptions -- and can encode values back for writing to devices.

## Features

- **IODD XML Parsing** -- Parse IODD 1.1 XML files and ZIP archives into strongly-typed Rust structs
- **Process Data Decoding** -- Convert hex PDIn data into named data points with values, units, and descriptions
- **Process Data Encoding** -- Encode human-readable values back to hex for writing to devices
- **IODD Catalog** -- Query the official [IODD Finder API](https://ioddfinder.io-link.com) to download device descriptions automatically
- **Menu Navigation** -- Browse Observer, Maintenance, and Specialist menus with conditional resolution
- **Multi-Language Support** -- 10 languages (EN, DE, ES, FR, IT, JA, KO, PT, RU, ZH)
- **IFM IoTCore Client** -- Read/write parameters and process data from IFM IO-Link masters over HTTP
- **WebAssembly Ready** -- Core crates target `wasm32-unknown-unknown`

## Workspace Structure

| Crate | Description |
|---|---|
| `ioddengine` | Core library -- parser, engine, catalog, encoders/decoders |
| `ioddcli` | Command-line interface with 10 subcommands |
| `iotcoreconnect` | HTTP client for IFM IoTCore IO-Link masters |

## Quick Start

### Prerequisites

- Rust (2021 edition)
- Download the [IODD Standard Definitions](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip) and place the spec files into `data/specs/`
- For live device interaction: a network-accessible IFM IoTCore master

### Build

```bash
cargo build --all
```

### Run Tests

```bash
cargo test --verbose
```

### Run Benchmarks

```bash
cargo bench
```

## CLI Usage

```bash
cargo run -p ioddcli -- <subcommand> [args]
```

### Read live process data from a device

```bash
ioddcli eval --ip 192.168.56.89 --port 1
```

### Parse an IODD XML or ZIP file

```bash
ioddcli parse --file path/to/device.xml
ioddcli parsezip --file path/to/device.zip
```

### Decode raw hex data offline

```bash
ioddcli parseraw --vendorid 310 --deviceid 872 --productname SD1540 \
  --hexdata 0000000000000000FE00079EFE0000000300
```

### List and inspect variables

```bash
# List all read-write variables
ioddcli list-variable --vendorid 310 --deviceid 706 --productname TN2405 --accessrights rw

# Show a specific variable
ioddcli variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS
```

### Read/write device parameters

```bash
# Read a parameter
ioddcli read-parameter --ip 192.168.56.89 --port 2 --index 551 --subindex 0

# Write a parameter
ioddcli set-parameter --ip 192.168.56.89 --port 2 --index 552 --subindex 0 --hexdata 4400
```

### Encode values to hex

```bash
ioddcli encode-variable --vendorid 310 --deviceid 706 --productname TN2405 \
  --id V_uni --param "0"

ioddcli encode-variable --vendorid 310 --deviceid 706 --productname TN2405 \
  --id V_diS --param "true, false, 4"
```

### Browse device menus

```bash
ioddcli get-menu --vendorid 310 --deviceid 706 --productname TN2405 \
  --role observer --menu parameter
```

## Library Usage

```rust
use ioddengine::{catalog::Catalog, parser::Parser, engine::Engine};

let catalog = Catalog::new_with_db(None);
let (drivername, files) = catalog
    .queryfordriver(device_id, product_name.to_owned(), vendor_id)
    .await;

let parser = Parser::new(drivername, files);
let engine = Engine::new(&parser.iodevice, "en");

let datapoints = engine.parse(hex_data).expect("Failed to parse process data");

for dp in datapoints {
    println!(
        "Name: {}, Desc: {}, Value: {}{}",
        dp.name,
        dp.description,
        dp.value,
        if !dp.unit.is_empty() {
            format!(", Unit: {}", dp.unit)
        } else {
            String::new()
        }
    );
}
```

**Example output** (SD1540 flow sensor):
```
HexData: 0000000000000000FE00079EFE0000000300
Name: Totalisator, Desc: Durchflussmenge, Value: 0.0, Unit: m³
Name: Durchfluss, Desc: Aktueller Durchfluss, Value: 0.0, Unit: m³/h
Name: Temperatur, Desc: Aktuelle Temperatur, Value: 19.5, Unit: °C
Name: Druck, Desc: Aktueller Druck, Value: 0.0, Unit: bar
Name: OUT2, Desc: Aktueller Zustand des digitalen Signals [OUT2], Value: "OFF"
Name: OUT1, Desc: Aktueller Zustand des digitalen Signals [OUT1], Value: "OFF"
```

## Environment Variables

| Variable | Description | Default |
|---|---|---|
| `LOCALE` | Language for text resolution | `en` |
| `RUST_LOG` | Log verbosity (`debug`, `info`, `warn`, `error`) | -- |

## Supported Data Types

| Type | Decode | Encode |
|---|---|---|
| `RecordT` | Yes | Yes |
| `UIntegerT` | Yes | Yes |
| `IntegerT` | Yes | Yes |
| `Float32T` | Yes | Yes |
| `BooleanT` | Yes | Yes |
| `StringT` | -- | Yes |

## Roadmap

- [ ] Read device variables before printing PDIn (e.g. Fahrenheit vs Celsius setting)
- [ ] CRC32 verification on IODD files
- [ ] Top-level decode support for standalone data types (IntegerT, StringT, OctetStringT, TimeT, TimeSpanT, ArrayT)
- [ ] Additional encoder data types

## License

MIT -- see [LICENSE](LICENSE) for details.
