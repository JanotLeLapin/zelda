{ buildNpmPackage
, }: buildNpmPackage {
  pname = "zelda-web";
  version = "0.1";
  src = ./.;
  npmDepsHash = "sha256-PqcUdLRGt/9ACqiK/VDnAgpFFh5S5qw844apMaW6d+g=";
}
