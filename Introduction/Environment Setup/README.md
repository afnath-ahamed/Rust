# Environment Setup

## Step 1 — Install Microsoft C++ Build Tools

### Why is this required?

On Windows, Rust's recommended MSVC toolchain needs Microsoft's linker to build executable programs.

### Installation steps

1. Open the [Microsoft C++ Build Tools download page](https://visualstudio.microsoft.com/downloads/).
2. Download and run the installer.
3. Select **Desktop development with C++**.
4. Ensure the **MSVC C++ build tools** and **Windows SDK** components are selected.
5. Click **Install**.

> [!NOTE]
> You do not need the full Visual Studio IDE. The standalone build tools are sufficient. Rustup can also help install these prerequisites when they are missing.

### Visual Studio Installer selection

The installer should have the **Desktop development with C++** workload selected, including the MSVC build tools and a Windows SDK.

![Visual Studio Installer with the Desktop development with C++ workload selected](./visual-studio-build-tools.png)

---

## Step 2 — Install Rust using Rustup

### Why Rustup?

Rustup installs and manages the Rust compiler, Cargo, and Rust toolchain updates.

### Installation steps

1. Visit the [official Rustup installer](https://rustup.rs/).
2. Download `rustup-init.exe` for Windows, usually the 64-bit version.
3. Run the installer.
4. Choose the default installation option.

### Tools installed by Rustup

| Tool | Purpose |
| --- | --- |
| `rustup` | Manages Rust versions and toolchains |
| `rustc` | Compiles Rust code |
| `cargo` | Builds projects and manages packages |
| `rustfmt` | Formats Rust code |
| `clippy` | Detects common code issues |

For standard Windows development, use the stable MSVC toolchain.

### Select the default installation

When the installer displays the available options, press **Enter** to choose option `1` and continue with the standard installation.

```text
The Cargo home directory is located at:

  C:\Users\<username>\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\<username>\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the PATH registry key at HKEY_CURRENT_USER\Environment.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
```
