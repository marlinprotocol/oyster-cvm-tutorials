{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    fenix,
    naersk,
  }: let
    systemBuilder = systemConfig: rec {
      echo-server = import ./echo-server/default.nix {
        inherit nixpkgs systemConfig fenix naersk;
      };
    };
  in {
    formatter = {
      "x86_64-linux" = nixpkgs.legacyPackages."x86_64-linux".alejandra;
      "aarch64-linux" = nixpkgs.legacyPackages."aarch64-linux".alejandra;
    };
    packages = {
      "x86_64-linux" = systemBuilder {
        system = "x86_64-linux";
        rust_target = "x86_64-unknown-linux-musl";
        eif_arch = "x86_64";
        static = true;
      };
      "aarch64-linux" = systemBuilder {
        system = "aarch64-linux";
        rust_target = "aarch64-unknown-linux-musl";
        eif_arch = "aarch64";
        static = true;
      };
    };
  };
}
