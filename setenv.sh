#! /bin/sh
#
# setenv.sh
# Copyright (C) 2023 rzavalet <rzavalet@noemail.com>
#
# Distributed under terms of the MIT license.
#


# From https://stackoverflow.com/questions/40833078/how-do-i-specify-the-linker-path-in-rust
export RUSTFLAGS="-L ${HOME}/opt/sdl2/lib"
