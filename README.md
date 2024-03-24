# VM Translator

This project is a part of the **NAND to Tetris** course, which aims to take learners through the process of building a modern computer system from the ground up, starting with elementary logic gates and ending with a fully functional operating system.

## Overview

In this project, we continue our journey from NAND gates to a complete computer system by implementing a compiler for the high-level programming language **Jack**. The compiler translates Jack programs into the assembly language of the Hack platform. 

## Getting Started

To get started with this project, follow these steps:

1. Clone this repository to your local machine.
   ```bash
   git clone git@github.com:WilliamTakeshi/vm-translator-rust.git
   ```

2. Navigate to the project directory.
   ```bash
   cd vm-translator-rust
   ```

3. Now you can transate programs in a intermediate language (`.vm`) to a Hack programming language file (`.asm`).
   ```bash
   cargo run -- -i examples/MemoryAccess/StaticTest/StaticTest.vm --output examples/MemoryAccess/StaticTest/StaticTest.asm
   ```

## Testing

You can test the functionality of the compiler by running the CPUEmulator (downloaded on the [nand-to-tetris website](https://www.nand2tetris.org/software)) and use the `.tst` files on the `/example` directory.

## Resources

- [NAND to Tetris Course](https://www.nand2tetris.org/)
- [Project 7: Jack Compiler](https://www.nand2tetris.org/project07)

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](https://github.com/WilliamTakeshi/vm-translator-rust/blob/main/LICENSE).

---

By [William](https://github.com/WilliamTakeshi)