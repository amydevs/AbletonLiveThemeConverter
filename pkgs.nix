import (
  let rev = "4001a95e1d37442cd413ab3ffdfe5bf6f1522991"; in
  builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz"
)
