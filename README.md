# SPA-JSON Parser and Serializer for Rust

## Overview

This repository contains a Rust implementation of a SPA-JSON parser and serializer. SPA-JSON is a JSON-like format used in the WirePlumber project, which is a modular session/policy manager for PipeWire. This library provides a way to parse SPA-JSON formatted strings into Rust data structures and convert Rust data structures into SPA-JSON formatted strings, which can be used in configuration files and inter-process communication within the WirePlumber ecosystem.

## Features

- Parsing of SPA-JSON formatted strings into Rust data structures
- Serialization of Rust data structures to SPA-JSON format
- Support for basic data types (integers, floats, strings, booleans, null)
- Handling of complex structures (arrays, objects)
- Customizable indentation for pretty-printing (serialization)
- Error handling for parsing and serialization failures

## Installation

To use this SPA-JSON library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
spa-json = "0.1.0"
```


## Usage

Here's a basic example of how to use the SPA-JSON parser and serializer:

```rust
use spa_json::{parse, to_string};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    value: i32,
    enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parsing
    let json_str = r#"{
        name = "example"
        value = 42
        enabled = true
    }"#;
    
    let config: Config = parse(json_str)?;
    println!("Parsed config: {:?}", config);

    // Serializing
    let config = Config {
        name: "example".to_string(),
        value: 42,
        enabled: true,
    };

    let json = to_string(&config)?;
    println!("Serialized JSON:\n{}", json);

    Ok(())
}
```

This will output:

```
Parsed config: Config { name: "example", value: 42, enabled: true }
Serialized JSON:
{
  name = "example"
  value = 42
  enabled = true
}
```

## API Reference

### Parsing

#### `parse<T>(s: &str) -> Result<T, Error>`

Parses a SPA-JSON formatted string into a Rust data structure.

- `s`: The SPA-JSON string to parse.
- Returns: A `Result` containing either the parsed data structure or an `Error`.

### Serialization

#### `to_string<T>(value: &T) -> Result<String, Error>`

Serializes a value to a SPA-JSON formatted string.

- `value`: The value to serialize. It must implement the `serde::Serialize` trait.
- Returns: A `Result` containing either the serialized string or an `Error`.

## Configuration File Support

This SPA-JSON serializer is particularly useful for working with WirePlumber configuration files. WirePlumber uses a modular configuration system where settings can be defined in multiple files and fragments. The SPA-JSON format allows for a more flexible and readable configuration syntax compared to standard JSON.

Some key features of WirePlumber configuration that this serializer supports:

1. **Fragments**: Configuration can be split across multiple files, allowing for easier management and overriding of settings.
2. **Dynamic Options**: Certain configuration options (called "settings") can be modified at runtime.
3. **Static Options**: Other configuration options are static and require a restart of WirePlumber to take effect.
4. **Rules**: Complex matching rules can be defined using SPA-JSON syntax, which this serializer can generate from Rust structures.

## Contributing

Contributions to this project are welcome! Here are some ways you can contribute:

1. Report bugs or request features by opening an issue.
2. Improve documentation or add examples.
3. Submit pull requests with bug fixes or new features.

Please ensure that your code follows the existing style and includes appropriate tests.

## License

This project is licensed under [INSERT LICENSE HERE]. See the LICENSE file for details.

## Acknowledgements

This project is inspired by and designed to work with the WirePlumber project. Special thanks to the WirePlumber community for their work on the SPA-JSON format and configuration system.
