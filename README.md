# i - A Minimalist, High-Performance Programming Language

![i Language Logo](logo.svg)

i is a Zen-inspired programming language that achieves massive functionality within a 56MB footprint through aggressive optimization and zero-cost abstractions.

## Features

- **Minimalist Syntax**: Clean, simple syntax inspired by Zen philosophy
- **High Performance**: Built in Rust with aggressive optimizations
- **Small Footprint**: Complete runtime in just 56MB
- **File Type Integration**: Automatic `.i` file association for seamless development
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Quick Start (Windows)

### Automatic Installation (Recommended)

When you clone the repository, the setup will automatically start:

```bash
git clone https://github.com/qwert1231231/.i
cd .i
```

**A popup window will appear asking:**
> *"Would you like to install i language and add .i file support to your system?"*

- **Click "Yes"** - Automatically installs everything
- **Click "No"** - Skip installation (you can run `build_and_install.bat` later)

The automatic installer will:
- ✅ Add `i` to your system PATH
- ✅ Associate `.i` files with the i interpreter  
- ✅ Enable double-click execution of `.i` files
- ✅ Add right-click context menu options
- ✅ Set up proper file type recognition

### Manual Installation

If you skipped the automatic setup or need to reinstall:

1. **Run** the `install_with_uac.bat` script (triggers UAC prompt)
2. **Or run** `build_and_install.bat` as Administrator
3. **Done!** You can now create and run `.i` files immediately

### UAC Integration

The installer now includes proper Windows UAC integration:

- **Automatic UAC Prompt**: "Do you want to allow this app to make changes to your device?"
- **Administrator Privileges**: Required for system file associations
- **Security Compliance**: Follows Windows security best practices
- **User-Friendly**: Clear permission requests with explanations

### What Gets Installed

The installer automatically:
- ✅ Adds `i` to your system PATH
- ✅ Associates `.i` files with the i interpreter
- ✅ Sets up double-click execution for `.i` files
- ✅ Adds "New i Language File" to your right-click context menu
- ✅ Creates registry entries for proper file type recognition

### Using i Language

After installation, you can:

#### Create a new .i file
- Right-click in any folder → "New i Language File"
- Or create a file with `.i` extension manually

#### Write your first program
```lisp
(echo "Hello, World!")
(add 1 2 3)
```

#### Run your program
- **Double-click** the `.i` file
- **Right-click** → "Run with i"
- **Command line**: `i yourfile.i`

## Manual Installation

### Prerequisites
- Rust (latest stable version)
- Git

### Build from Source

```bash
git clone https://github.com/yourusername/i-lang.git
cd i-lang
cargo build --release
```

### Windows Setup

Run the installer as Administrator:
```batch
install.bat
```

### macOS/Linux Setup

Add to your `.bashrc` or `.zshrc`:
```bash
export PATH="$PATH:/path/to/i-lang/target/release"
```

Create a file association (optional):
```bash
# Add to ~/.local/share/applications/i-lang.desktop
[Desktop Entry]
Name=i Language
Exec=/path/to/i-lang/target/release/i %F
Terminal=true
Type=Application
MimeType=text/x-i-lang;
```

## Language Syntax

### Core Concepts

The i language follows Zen principles with these core values:
- `zen` = 0 (the state of emptiness)
- `flow` = true (the state of being)
- `void` = nil (the state of nothingness)

### Basic Operations

```lisp
;; Arithmetic
(add 1 2 3)        ; => 6
(sub 10 3)         ; => 7
(mul 2 3 4)        ; => 24
(div 20 4)         ; => 5

;; Output
(echo "Hello")     ; Prints: Hello

;; Conditionals
(if (flow) "yes" "no")  ; => "yes"
```

### File Extensions

i language supports two file extensions:
- `.i` - Primary source files
- `.rag` - Alternative extension (for compatibility)

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### Project Structure

```
i-lang/
├── src/
│   ├── main.rs          # Main interpreter
│   ├── macros.rs        # Macro definitions
│   └── stdlib.rs        # Standard library
├── examples/            # Example programs
├── logo.svg            # Language logo
├── install.bat          # Windows installer
├── uninstall.bat        # Windows uninstaller
├── build_and_install.bat # Build and install script
└── README.md           # This file
```

## Uninstallation

### Windows

Run as Administrator:
```batch
uninstall.bat
```

Then manually remove the i directory from your system PATH.

### macOS/Linux

Remove the path from your shell configuration file and delete the project directory.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Philosophy

The i language embraces minimalism and Zen philosophy:
- **Simplicity**: Complex problems solved with simple solutions
- **Efficiency**: Maximum functionality with minimum resources
- **Clarity**: Code that reads like thoughts
- **Flow**: Programming as a meditative practice

> "In the beginning, there was void. Then came i." - i Language Philosophy
