{ rustPlatform
}: rustPlatform.buildRustPackage {
  pname = "zelda-server";
  version = "0.1.0";
  src = ./.;
  cargoLock = { lockFile = ./Cargo.lock; };
}
