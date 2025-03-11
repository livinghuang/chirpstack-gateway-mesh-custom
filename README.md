# ChirpStack Gateway Mesh Custom

This project is a fork of [ChirpStack Gateway Mesh](https://github.com/chirpstack/chirpstack-gateway-mesh), modified to enhance its functionality. The custom implementation forces the gateway to act as a **mesh border node** while also supporting the **transmission of periodic heartbeat packets** to maintain network stability and monitoring.

## Documentation and Binaries

For general documentation and pre-compiled binaries, please refer to the [ChirpStack Gateway Mesh Documentation](https://www.chirpstack.io/docs/chirpstack-gateway-mesh/).

## Custom Modifications

This version introduces the following enhancements:
- **Mesh Border Enforcement**: Configures the gateway as a fixed border node within the mesh network.
- **Heartbeat Packet Transmission**: Periodically sends heartbeat packets to help monitor network health and ensure connectivity.

## Building from Source

### Requirements

Building this project requires:

- [Nix](https://nixos.org/download.html) (recommended)
- [Docker](https://www.docker.com/)

#### Nix

Nix is used to set up the development environment for local development and compilation. Alternatively, dependencies can be installed manually by referring to `shell.nix`.

#### Docker

Docker is used by [cross-rs](https://github.com/cross-rs/cross) for cross-compiling and also for some of the `make` commands.

### Starting the Development Shell

Run the following command to start the development shell:

```bash
nix-shell
```

### Running Tests

To run the test suite, execute:

```bash
make test
```

### Compiling Binaries

To build the ChirpStack Gateway Mesh binaries and packages, use:

```bash
# Compile binaries only
make build

# Compile binaries and build distributable packages
make dist
```

## License

This project is distributed under the MIT license. See [LICENSE](https://github.com/chirpstack/chirpstack-gateway-mesh/blob/master/LICENSE) for details.
