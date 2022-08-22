{
  description = "sparsesrv";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/master";

  outputs = { self, nixpkgs }:
    let
      systems =
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

      # Memoize nixpkgs for different platforms for efficiency.
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        });

    in {
      overlays.default = final: prev: {
        sparsesrv = final.callPackage ({ rustPlatform }:

          rustPlatform.buildRustPackage {
            pname = "sparsesrv";
            version = "0.1.0";
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          }) { };
      };

      packages =
        forAllSystems (system: { inherit (nixpkgsFor.${system}) sparsesrv; });
      defaultPackage = forAllSystems (system: self.packages.${system}.sparsesrv);
    };
}
