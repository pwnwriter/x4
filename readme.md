Possible configuration values:



### Configuration Options

The configuration JSON supports the following key values for server definitions:

| Key         | Description                                                 | Example Value                  |
|-------------|-------------------------------------------------------------|--------------------------------|
| `description` | description of the server.                        | "using a private key for SSH authentication and default 22 port" |
| `name`      | Name identifier for the server.                   | "ec2"                          |
| `host`      | The hostname or IP address of the server.                  | "fawn.pwnwriter.xyz"          |
| `user`      | The username for SSH authentication.                        | "fawn"                        |
| `password`  | The method of authentication. Can be an environment variable (prefix with `env:`) or a command (prefix with `cmd:`). | "env:wolf_pass" or "cmd:pass uni/server/wolf" |
| `commands`  | An array of commands to be executed after SSH connection.   | ["ls -l", "cat /etc/hostname"] |
