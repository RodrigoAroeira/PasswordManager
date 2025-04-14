# Rust Password Manager (Alpha)

## Critical Security Disclaimer
>
> [!WARNING]  
> **This is an UNSECURED ALPHA VERSION**
>
> Current limitations:
>
> - Passwords stored in plaintext JSON
> - No encryption implemented
> - No master password protection
> - No secure memory management
>
> **Only use with test credentials in development environments**

## Features (Current)

| Feature | Status | Notes |
|---------|--------|-------|
| Basic CLI | ✅ Implemented | `add`, `get`, `list` commands |
| JSON Storage | ✅ Implemented | Unencrypted `passwords.json` |

## Getting Started

### Development Setup

```bash
git clone https://github.com/RodrigoAroeira/PasswordManager.git
cd PasswordManager
cargo run -- --help
```
