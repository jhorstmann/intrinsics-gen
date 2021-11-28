#!/bin/bash

set -e
set pipefail

mkdir -p target

perl -p -0777\
     -e "s/#\[doc =\n\s*/#[doc = /g;"\
     -e "s/Intel\\\'s/Intel's/g;"\
     -e "s/#\[doc = \x22(.*)\x22\]/\/\/\/ \1/g;"\
     -e "s/asm!\(/asm!(\n             /g;"\
     -e "s/(in|out|inout)\((reg|kreg|zmm_reg|ymm_reg|xmm_reg)\)\n\s*/\1(\2) /g;"\
     -e "s/options\((.*)\)\);/options(\1)\n        );/g;"\
     -e "s/    \}\n/    }\n\n/g;"\
     target/expanded.rs > target/processed.rs

