{ rust-bin
, rust-analyzer
, nodejs
, mkShell
}: mkShell {
  packages = [
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
    rust-analyzer nodejs
  ];
}
