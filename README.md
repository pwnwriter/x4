<p align="center">
  <img src="https://github.com/user-attachments/assets/db6ea484-be58-4e68-bfcf-e868291867e7" width="200" height="200" alt="server image">
</p>

<p align="center">
  A lightweight implementation of <code>libssh2</code> to execute shell commands on remote servers with rich configuration via <code>json</code>.
</p>


<div align="center">
  <a href="https://github.com/pwnwriter/x4/releases">
    <img src="https://img.shields.io/github/v/release/pwnwriter/x4?style=flat&labelcolor=f38ba8&color=585b70&logo=github&logocolor=white" alt="release version">
  </a>
  
  <a href="https://crates.io/crates/x4/">
    <img src="https://img.shields.io/crates/v/x4?style=flat&labelcolor=b4befe&color=eba0ac&logo=rust&logocolor=white" alt="crates.io version">
  </a>
  
  <a href="https://github.com/pwnwriter/x4/actions?query=workflow%3a%22continuous+deployment%22">
    <img src="https://img.shields.io/github/actions/workflow/status/pwnwriter/x4/nix-build.yml?style=flat&labelcolor=eba0ac&color=74c7ec&label=nix-build&logo=github%20actions&logocolor=white" alt="github actions status">
  </a>
  
  <a href="https://github.com/pwnwriter/x4/blob/main/license">
    <img src="https://img.shields.io/badge/license-mit-white.svg" alt="mit license">
  </a>
  
</div>

<br></br>


<!--toc:start-->
- [Features and todos](#features-and-todos)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usages](#usages)
- [Configuration](#configuration)
  - [Options](#options)
    - [Examples](#examples)
- [Development](#development)
- [Support my work](#support-my-work)
  <!--toc:end-->


## Features and todos
- [x] retrieve the password from environment variables
- [x] retrieve the password by executing a shell command
    
    usages `pass` password manager to get the `user` password and `ssh`'s into the server
    
    https://github.com/user-attachments/assets/2a85e2ab-c762-41e0-855d-fa8e6d15f5e0

- [x] format `stderr` and `stdout` outputs for better readability
- [x] check pipeline configuration

    ![configuration-check](https://github.com/user-attachments/assets/ad24647c-ebbc-42af-8681-865dae15d678)

- [ ] Implement file upload and download functionality
- [ ] Allow connections through a proxy server
- [ ] Implement unit tests for critical features
- [ ] Current configuration is with `json` but with `lua` ? planned yes!! 
- [ ] Better code, docs and more
- [ ] More ways to get the json value fields, like password ?
- [ ] You tell more.


## Requirements 
- **ssh connection**: must have ssh connections using one of the following authentication methods:
  - **password authentication**: using the `username` and `password`.
  - **keypair**: public key authentication.
  - No extra deps will be required for runtime


## Installation
    
  <details> <summary><code>Binary </code></summary>
    &nbsp;
   
   -  **manual**: you can directly download the binary from [**releases**](https://github.com/pwnwriter/x4/releases) of your arch and run it.
   - **one liner**: run this one liner script 

```bash
wget -qo- "$(curl -qfssl "https://api.github.com/repos/pwnwriter/x4/releases/latest" | jq -r '.assets[].browser_download_url' | grep -ei "$(uname -m).*$(uname -s).*musl" | grep -v "\.sha")" | tar -xzf - --strip-components=1 && ./x4 -h
```  
  </details>
  <details> <summary><code>source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/x4 --branch=main
  cd x4
  cargo build --release 
  ```
  then go to `release` dir and `./x4` or move the `binary` to your any `$path` for instant access from anywhere.

</details>


> [!NOTE]  
> This requires a working setup of `rust/cargo` and `binstall`.

<details open> <summary><code>cargo </code></summary>

- using [crates.io](https://crates.io/crates/x4)
    
  
  ```bash
  cargo install x4
  ```

- using [binstall](https://github.com/cargo-bins/cargo-binstall)

  ```bash
  cargo binstall x4
  ```

</details>


<details open> <summary><code>on nix  </code></summary>
&nbsp;
  
- Source build

    ```
    nix run github:pwnwriter/x4
    ```

- With flakes:

    ```
    nix profile install nixpkgs#x4
    ```

- Without flakes:

  ```
   nix-env -ia nixpkgs.x4
  ```

</details>


## Usages

```yaml
Usage: x4 [OPTIONS]

Options:
  -p, --pipeline <PIPELINE>              Path to your pipeline file
      --check-pipeline <CHECK_PIPELINE>  Check the configuration of the specified pipeline
  -h, --help                             Print help
  -V, --version                          Print version

```


## Configuration

### Options

The configuration JSON supports the following key values for server definitions:

| Key          | Description                                                 | Example Value                             |
|--------------|-------------------------------------------------------------|-------------------------------------------|
| `description`| A brief description of the server's purpose or role.      | My hot EC2 instance                        |
| `name`       | A unique identifier for the server.                        | ec2                                       |
| `host`       | The hostname or IP address of the server.                  | fawn.pwnwriter.xyz                        |
| `port`       | The SSH port to connect to the server. Defaults to `22`.   | 22                                        |
| `user`       | The username used for SSH authentication.                  | fawn                                      |
| `password`   | Method of authentication for SSH: use an environment variable (prefix with `env:`) or a command (prefix with `cmd:`). See [Password Retrieval](#password-retrieval) for details. | `env:wolf_pass` or `cmd:pass uni/server/wolf` |
| `commands`   | An array of commands to execute once the SSH connection is established. | `[pnpm run build && pnpm start]`         |

---

#### Password Retrieval

The `password` field allows for secure handling of sensitive information using either environment variables or commands.

- **Using Environment Variables**: 
  To set your password as an environment variable in your shell, run:
  ```bash
  export wolf_pass='my_secure_password'
  ```
  You can then reference this variable in your configuration like so:
  ```json
  "password": "env:wolf_pass"
  ```

- **Using Commands**: 
  If you use the `pass` password manager, you can retrieve your password with a command. For instance:
  ```json
  "password": "cmd:pass uni/server/wolf"
  ```
  This command will execute `pass` to get the password stored under `uni/server/wolf`.


### Examples

- Single server Configuration with `cmd` for `password`

```json
{
  "servers": [
    {
      "description": "using a cmd to get password for SSH authentication and default 22 port",
      "name": "ec2",
      "host": "fawn.pwnwriter.xyz",
      "user": "wolf",
      "password": "cmd:pass personal/server/root",
      "commands": [
        "ping -c 3 google.com",
        "ls"
      ]
    }
  ]
}
```

- More than one server configuration

```json
{
  "servers": [
    {
      "description": "using a cmd to get password for SSH authentication and default 22 port",
      "name": "ec201",
      "host": "fawn.pwnwriter.xyz",
      "user": "wolf",
      "password": "cmd:pass personal/server/root",
      "commands": [
        "mkdir -p from_many_at_wolf",
        "cat /etc/os-release"
      ]
    },
    {
      "description": "using a private key for SSH authentication and default 22 port",
      "name": "ec2",
      "host": "fawn.pwnwriter.xyz",
      "user": "fawn",
      "private_key": "/Users/pwnwriter/.local/share/ssh/wynwood.pem",
      "commands": [
        "mkdir from_many_at_fawn",
        "mkdir from_manyyyy"
      ]
    }
  ]
}
```


## Development

if you are using nix, 

get into the development shell 

`nix develop` or use 

`direnv allow` to enter a shell with all required deps. 

`nix build/run` to build and run the app. 

`nix run github:pwnwriter/x4 -- --help` 

## Support my work

I do open source work in my free time, and I really enjoy it! If any of my applications have helped you in any way, please consider supporting me via Ko-fi. Your support enables me to continue developing and improving my projects.


<a href="https://ko-fi.com/pwnwriter" target="_blank">
    <img src="https://img.shields.io/badge/Ko--fi-Support%20Me%20%F0%9F%92%96-FF5E5B?style=flat-square&logo=ko-fi" alt="Support me on Ko-fi" width="250"/>
</a>


<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">copyright &copy; 2024 <a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz<a> 🍃</a> 
