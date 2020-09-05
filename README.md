<p align="center">
  <img src="./assets/logo.png" width="40%" />
</p>

<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/Version-1.0-red.svg" />
  <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-orange.svg" />
  <img alt="Made with Rust" src="https://img.shields.io/badge/Made%20with-Rust-yellow.svg" />
  <img alt="gh-actions" src="https://github.com/safinsingh/malta/workflows/Rust/badge.svg" />
  <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-Welcome-blue.svg">
  <img alt="awesome" src="https://img.shields.io/badge/Awesome-Yes-purple">
  <br />
</p>

<hr>

## ✨ Binaries

```sh
git clone https://github.com/safinsingh/malta.git
make release
```

## 🔮 In action

![demo](./assets/demo.gif)

## 🔎 How it works

- `helios` is `malta`'s client engine, it finds and scores vulnerabilities specified in a configuration file (see the example helios/conf.yaml).
- After capturing the secret keys generated by `helios gen-keys`, insert them into `{helios/ares}/src/crypto.rs` in order to encrypt/decrypt your configuration. After writing your configuration, you'll need to re-build the binaries with `cargo build --release`.
- Notice that in the configuration, you'll need to specify the URL of a Firebase Realtime Database URL. This is where all scoring data will be appended.
- Once built, you can run `helios encrypt` to encrypt your configuration. You can now safely delete your `config.yaml` and place the encrypted `conf.z` on both your server and client.
- Now, start `ares` and distribute the scoring engine. Typically, this is done via the distribution of an insecure virtual machine with vulnerabilities preloaded.

> If you allow your Firebase connection to be unauthenticated, make sure to set security rules to only allow the IP of your remote server running `ares`

## 📖 Checks

The check schema for helios looks like the following:

```yaml
# Global constants
title: "Safin's OP Round"
remote: "http://localhost:8000/"
db: "https://malta-rs.firebaseio.com"

# Array of vulnerability records
records:
  # All vulnerabilities must have
  # a message, identifier, and points.
  # If the points are negative, it's
  # counted as a penalty
  - message: Removed vulnerability
    identifier: a1b2c3
    points: -4

    # Array of all checks
    checks:
      # Array of REQUIRED sucessful checks,
      # the following MUST be TRUE for the
      # check to pass
      - success:
          - type: FileContains
            file: "/home/safin/Documents/helios/hi.txt"
            contains: "^hello"

      # Array of REQUIRED sucessful checks,
      # the following MUST be FALSE for the
      # check to pass
      - fail:
          - type: FileContains
            file: "/home/safin/Documents/helios/hi2.txt"
            contains: "^hello"
```

`helios` currently supports many checks for both Windows and Unix:

```rust
// Score a file containing a regular expression.
pub struct FileContains {
    file: String,
    contains: String,
}
```

```rust
// Score a command exiting with a certain
// exit code. If a custom code is not specified,
// it defaults to 0.
pub struct CommandExitCode {
    command: String,
    code: Option<i32>,
}
```

```rust
// Score a command's STDOUT matching a
// regular expression.
pub struct CommandOutput {
    command: String,
    contains: String,
}
```

```rust
// Score a file that exists on the system.
pub struct FileExists {
    path: String,
}
```

```rust
// Score a user existing on the system.
// Currently only supports Unix systems.
pub struct UserExists {
    user: String,
}
```

```rust
// Score a group existing on the system.
// Currently only supports Unix systems.
pub struct GroupExists {
    group: String,
}
```

```rust
// Score a user existing in a group.
// Currently only supports Unix systems.
pub struct UserInGroup {
    user: String,
    group: String,
}
```

```rust
// Score the firewall status.
// Currently only supports Unix systems.
pub struct Firewall {}
```

```rust
// Score a systemd service being active.
// Currently only supports Unix systems.
pub struct Service {
    service: String,
}
```

## 👨‍💻 Author

Linkedin: [Safin Singh](https://www.linkedin.com/in/safin-singh-b2630918a/) <br>
GitHub: [safinsingh](https://github.com/safinsingh) <br>
Dribbble: [Safin Singh](https://dribbble.com/safinsingh/) <br>
YouTube: [Safin Singh](https://www.youtube.com/channel/UCvb01sUdAgcPAG1j0SLxAtA)

## 🤝 Contributing

Contributions, PRs, issues and feature requests are welcome! Feel free to check out my [issues page](https://github.com/safinsingh/malta/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you!
Hope you enjoy it!
