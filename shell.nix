let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  buildInputs = (with pkgs; [ cargo rustc rustfmt cargo-edit cargo-flamegraph ]);
}
