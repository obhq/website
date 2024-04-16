# Building from source
---
### Windows prerequisites

- [Visual Studio 2022](https://visualstudio.microsoft.com/vs/)
  - `Desktop development with C++` workload is required
- [Rust on the latest stable channel](https://www.rust-lang.org/tools/install)
- [CMake 3.21+](https://cmake.org/download/)
  - Make sure you have `Add CMake to the system PATH` selected when installing
- [Ninja](https://ninja-build.org)
  - Make sure Ninja is added to `PATH`
  - Note, if you're having trouble, you can install `ninja` using [Chocolatey](https://chocolatey.org/install) with `choco install ninja`!

### Linux prerequisites

- GCC 9.4+
- Rust on the latest stable channel
- CMake 3.21+

### macOS prerequisites

- macOS 12+
- Homebrew
- Clang 13+
- Rust on the latest stable channel
- CMake 3.21+

### Install Qt 6

You need to install Qt 6 on your system before you proceed. The minimum version is 6.5. You also need to enable SVG support when installing.

#### Windows-specific requirements

You need `Qt Online Installer` for open-source to install Qt, downloaded from https://www.qt.io. The installer will ask you to sign in with a Qt account, which you can create for free. You need to check `Custom installation` and do not check `Qt for desktop development` that is using the MinGW toolchain. Make sure you have checked the `MSVC 2019 64-bit` component in the `Select Components` page for the version you wish to install and uncheck all of the other components.

Once installation is completed you need to set the `CMAKE_PREFIX_PATH` environment variable to the full path of the installed version (e.g. `C:\Qt\6.5.1\msvc2019_64`). To set an environment variable:

1. Open a run dialog with <kbd>Win</kbd> + <kbd>R</kbd>.
2. Enter `sysdm.cpl` then click `OK`.
3. Go to the `Advanced` tab then click on `Environment Variables...`.
4. Click `New...` to create a new environment variable. Just create for either `User variables` or `System variables`, not both.
5. Restart your terminal or IDE to load the new PATH.

#### Install Qt with Homebrew (macOS only)

```sh
brew install qt@6
```

### Configure build system

```sh
cmake --preset PRESET .
```

The value of `PRESET` will depend on your platform and the build configuration you want to use. The current available presets are:

- windows-release
- windows-debug
- linux-release
- linux-debug
- mac-release
- mac-debug

If all you want is to use the emulator, choose `[YOUR-PLATFORM]-release` for optimized outputs. But if you want to edit the code, choose `*-debug`.

### Build

```sh
cmake --build --preset PRESET
```

You can use `-j` to enable parallel building (e.g. `cmake --build --preset PRESET -j 2`). Each parallel build on Linux consumes a lot of memory so don't use the number of your CPU cores otherwise your system might crash due to out of memory. On Windows and macOS it seems like it is safe to use the number of your CPU cores.
