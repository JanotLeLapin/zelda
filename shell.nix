{ rust-bin
, rust-analyzer
, nodejs
, nodePackages
, mkShell
}: mkShell {
  packages = [
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) nodejs
    rust-analyzer nodePackages.typescript-language-server nodePackages.svelte-language-server
  ];
}
