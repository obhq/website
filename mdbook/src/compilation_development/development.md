# Development
---
### Recommended Code Editor
Before proceeding, make sure the build preset you are using is `*-debug`. We recommend [Visual Studio Code](https://code.visualstudio.com/Download) as a code editor with the following extensions:

- CMake Tools ([Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cmake-tools) | [Open VSX](https://open-vsx.org/extension/ms-vscode/cmake-tools))
- clangd ([Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=llvm-vs-code-extensions.vscode-clangd) | [Open VSX](https://open-vsx.org/extension/llvm-vs-code-extensions/vscode-clangd))
- rust-analyzer ([Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) | [Open VSX](https://open-vsx.org/extension/rust-lang/rust-analyzer))
- CodeLLDB ([Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) | [Open VSX](https://open-vsx.org/extension/vadimcn/vscode-lldb))

Then open this folder with VS Code. It will ask which CMake preset to use and you need to choose the same one that you were using when building. Everything should work out of the box (e.g. code completion, debugging, etc).

### macOS debugging issues

If you can't launch or debug Obliteration from VS Code, try [this](https://github.com/vadimcn/codelldb/discussions/456#discussioncomment-874122) solution.

### Get a homebrew application for testing

If you don't have a PS4 application for testing you can download PS Scene Quiz for free [here](https://pkg-zone.com/details/LAPY10010).

### Rules for Rust sources

- Use unsafe code only when you know what you are doing. When you do try to wrap it in a safe function so other people who are not familiar with unsafe code can have a safe life.
- Don't chain method calls without an intermediate variable if the result code is hard to follow. We encourage code readability as a pleasure when writing so try to make it easy to read and understand for other people.
- Do not blindly cast an integer. Make sure the value can fit in a destination type. We don't have any plans to support non-64-bit systems so the pointer size and its related types like `usize` are always 64-bits.
- Beware of deadlock and memory leak. Rust can protect us from most mistakes, except those two. Deadlock can be happen easily in Rust because Rust requires us to wrap the data we want to protect with a mutex and get a mutable reference through it. The problem with this is it become natural for you to lock the mutex to operate on the inner data, which can easily cause a deadlock if you are not aware when there are another locks being active. The only cases for memory leak you need to aware is when working with `Arc`. Just make sure you don't create a reference cycle that will never be dropped.

### Rules for C++ sources

Just follow how Qt is written (e.g. coding style, etc.). Always prefers Qt classes over `std` when possible so you don't need to handle exceptions. Do not use the Qt `ui` file to design the UI because it will break on a high-DPI screen.

### Starting point

The application consists of 2 binaries:

#### Frontend

This is what users will see when they launch Obliteration. Its entry point is inside `src/main.cpp`.

#### Kernel

This is where emulation takes place. Its entry point is inside `src/kernel/src/main.rs`.

### Debugging the kernel

Create `.kernel-debug` in the root of the repository. The contents of this file is YAML and the kernel will deserialize it to the `Args` struct in `src/kernel/src/main.rs` when passing the `--debug` flag to the kernel. See `Args` struct for available options.

We already provide a launch configuration for VS Code so all you need to do is choose `Debug - Kernel` as the configuration and start debugging.

### Code contribution

If you want to make some contributions but don't know what to work on you can look for `TODO` comment or `todo!` macro invocation in the source code. You can also take a look at unassigned issues.

### Additional informations

[PS4 Developer Wiki](https://www.psdevwiki.com/ps4) has a lot of useful information about the PS4 internal. We also have a PS4 reverse engineering [project](https://github.com/obhq/reverse-engineering).