with import <nixpkgs> {};
let
  unstable = import <nixos-unstable> { config = { allowUnfree = true; }; };
in {
     testEnv = unstable.stdenv.mkDerivation {
       name = "helloTest";
       buildInputs = [unstable.stdenv unstable.ncurses
                      unstable.cargo
                      unstable.rustc unstable.rustup
                      ];
     };
}
