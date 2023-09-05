# Rusty Game 🦀🎮

## Overview

Rusty Game is a game development project that serves as a playground for learning and practicing the Entity-Component-System (ECS) architecture using the Bevy game engine in Rust. This project is also an exploration of Test-Driven Development (TDD) in Rust game development.

🔗 **Blog Post**: [TDD in Rust Game Engine Bevy](https://edgardocarreras.com/blog/tdd-in-rust-game-engine-bevy)

## Table of Contents

- [Overview](#overview)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Technologies

### Rust 🦀

- **File**: `Cargo.toml`, `src/lib.rs`, `src/main.rs`
- **Purpose**: Rust is used for the game logic, offering memory safety and high performance. The project uses Rust's package manager, Cargo, to manage dependencies and build the project.

### Bevy 🎮

- **File**: `src/main.rs`, `src/components.rs`, `src/systems.rs`
- **Purpose**: Bevy is a data-driven game engine built in Rust. It's used here to implement the ECS architecture, providing a flexible and efficient framework for game development.

### TDD 🧪

- **File**: Collocated with code modules
- **Purpose**: Test-Driven Development (TDD) is used to ensure the quality and correctness of the game logic. Unit tests are collocated with the code modules, as per Rust standards.

## Getting Started

1. **Clone the Repository**
    ```bash
    git clone https://github.com/donedgardo/rusty_game.git
    ```

2. **Install Rust**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. **Run the Project**
    ```bash
    cargo run 
    ```

## Usage

To play the game, simply run `cargo run` after cloning and setting up the project.

## Contributing

Feel free to contribute to this project. Fork it, create a new branch, commit your changes, and create a pull request.

## License

This project is licensed under the MIT License.

---

Created by [Edgardo Carreras](https://github.com/donedgardo) to explore and demonstrate the capabilities of Bevy and TDD in Rust game development.

