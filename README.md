# Distance Calculator

A simple command-line tool to calculate the distance between two locations using the Nominatim API.

## Description

This project is a Rust-based command-line tool that calculates the distance between two locations. It uses the Nominatim API to fetch the latitude and longitude of the given locations and then calculates the distance using the Haversine formula.

## Usage

### Prerequisites

- Rust installed on your machine

### Installation

1. Clone the repository:

```sh

```

2. Navigate to the project directory:

```sh
cd calc-distance
```

3. Build the project:

```sh
cargo build --release
```

### Running the Tool

To calculate the distance between two locations, use the following command:

```sh
cargo run -- calc --origin "Location1" --destination "Location2"
```

Replace `Location1` and `Location2` with the names of the locations you want to calculate the distance between.

### Example

```sh
cargo run -- calc --origin "Berlin" --destination "Paris"
```

This will output the distance between Berlin and Paris in kilometers.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes.

## License

This project is licensed under the MIT License.