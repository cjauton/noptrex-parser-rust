# Rust Program Readme

This repository contains a Rust program that performs data reading and processing based on configuration files. The program reads binary data from a file and extracts pulses with various waveform parameters.

## Getting Started

To use this program, follow the steps below:

1. Clone the repository to your local machine.
2. Make sure you have Rust installed. If not, you can install it from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
3. Open a terminal or command prompt and navigate to the cloned repository's directory.

## Usage

To run the program, use the following command:

```bash
cargo run <run_number>
```


Replace `<run_number>` with the desired run number for which you have data and configuration files. The program expects the data file to be located in the `./data` directory and the configuration file in the `./meta` directory. The filenames should follow the format `run<run_number>.bin` for the data file and `run<run_number>.conf` for the configuration file.

## Dependencies

This program depends on the following external crate:

- `configparser` (version 0.6.0): A crate for parsing INI configuration files. For more information, refer to the crate's documentation: [https://docs.rs/configparser/0.6.0/configparser/ini/index.html](https://docs.rs/configparser/0.6.0/configparser/ini/index.html).

The necessary dependencies are managed through Cargo, the Rust package manager, and will be automatically downloaded and built when you run the program.

## Program Structure

The main Rust file (`main.rs`) contains the entry point of the program and defines the data structures used for storing pulse and configuration information.

The program uses two main functions:

- `read_data`: Reads the binary data from the file and returns a vector of `Pulse` structures containing waveform information.
- `read_data2`: A duplicate implementation of the `read_data` function.

The `Config` structure represents the configuration settings loaded from the INI file. It includes methods for reading and processing the configuration parameters.

## Contributing

Contributions to this project are welcome. If you find any issues or have suggestions for improvements, please open an issue or submit a pull request. Please make sure to follow the project's code of conduct.

## License

This program is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as per the terms of the license.

## Acknowledgments

- The `configparser` crate authors and contributors for providing a convenient INI parsing solution for Rust.
- The Rust programming language community for their excellent tools and resources.

If you have any questions or need further assistance, please feel free to contact the project maintainers.
