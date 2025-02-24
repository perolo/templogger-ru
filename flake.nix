{
  description = "Rust flake";
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux"; # your version
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        name = "Rust Flake";
        packages = with pkgs; [ 
        # add this module, to enable cross-compilation.
          #crossSystem = {
            # target platform
            #system = "thumbv6m-none-eabi";
          #};
          rustup
          file
          elfutils
          elf2uf2-rs
        ]; # whatever you need
        shellHook = ''
          export PS1='\n\[\033[1;34m\](Rust):\w]\$\[\033[0m\]'
          echo "Welcome to the $name development shell!"
          echo "All necessary libraries and tools are installed."
          #rustup target add thumbv6m-none-eabi
        '';
      };
    };
}