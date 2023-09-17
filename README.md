# Turing Machine Simulator - TMS

An Alan Turing machine simulator written in Rust as part of my personal learning journey to master the Rust programming language.

<!--

## Screenshots

![App Screenshot]()

-->

## Getting Started

Follow these instructions to get the Turing Machine simulator up and running on your system.

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) - The Rust programming language.

### Installation

1. Clone the repository:

    ```bash
    $ git clone https://github.com/LanaMirko04/tms
    $ cd tms
    ```

2. Build the project:

    ```bash
    $ cargo build
    ```

3. Run the program:

    ```bash
    $ cargo run
    ```

## Usage

**Important Note:** The command-line interface (CLI) for this Turing Machine simulator has not been implemented yet. To use the simulator, you will need to manually modify the path to the configuration file within the source code.

To load your configuration file, follow this steps:

1. Open `src/main.rs` file in the project directory.
2. Locate the following line of code:

    ```rust
    let cfg_path = "/Users/mirko/Projects/tms/examples/example.cfg";
    ```

3. Modify the `cfg_path` variable and save the changes.
4. Run the program:

    ```bash
    $ cargo run
    ```

## License

TMS is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.


## Acknowledgements

 - [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/)
 - [The Cargo Book](https://doc.rust-lang.org/stable/cargo/)

