import (
  let rev = "6f8fa05acf5e6f6b4926631de2a9a8b8b544f30d"; in
  builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz"
)
