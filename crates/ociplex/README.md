# ociplex - An OCI multiplexer

The `ociplex` tool is an OCI runtime multiplexer.

The primary usage for `ociplex` is to act as a gateway between OCI-only runtime
engines like `podman` and semi-compliant runtimes like Kata Containers (which,
after version 2.0, only supports the `shimv2` interface).

However, `ociplex` features several back-ends, which makes it possible to target
another OCI runtime, for example to add additional logging or debugging
capability.
