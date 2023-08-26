let
  sysPkg = import <nixpkgs> { };
  releasedPkgs = sysPkg.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "26e8da26b4089966dce23f36741dd2be35fbeedc"; # master as of 2023-08-255
    sha256 = "sha256-zwjRPOC8ES0yQ684j+EC+Ggw6gXar4WiZF7Zt9ufRtk=";
  };
  pkgs = import releasedPkgs {};
  stdenv = pkgs.stdenv;
  extraInputs = sysPkg.lib.optionals stdenv.isDarwin (with sysPkg.darwin.apple_sdk.frameworks; [
    Cocoa
    CoreServices]);

in stdenv.mkDerivation {
  name = "env";
  buildInputs = [ pkgs.gnumake
                  pkgs.wget
                  pkgs.cargo
                  pkgs.rustc
                  pkgs.rustfmt
                  pkgs.rust-analyzer
                  pkgs.libiconv
                ] ++ extraInputs;
  shellHook = ''

  '';

}
