#!/bin/bash

set -e
set pipefail

mkdir -p target

#cargo rustc --lib --profile=check -- -Zunpretty=expanded > target/expanded.rs

#sed -iorig -z -f sed_script target/expanded.rs
#perl -pi -0777 -e "s/\[doc =\n\s*/#[doc = /g;s/asm!\(/asm!(\n            /g;s/in\\(reg\\)\n \*/in(reg) /g;s/in\\(kreg\\)\n \+/in\\(kreg\\) /g" target/expanded.rs
perl -p -0777\
     -e "s/#\[doc =\n\s*/#[doc = /g;"\
     -e "s/Intel\\\'s/Intel's/g;"\
     -e "s/#\[doc = \x22(.*)\x22\]/\/\/\/ \1/g;"\
     -e "s/asm!\(/asm!(\n             /g;"\
     -e "s/(in|out|inout)\((reg|kreg|zmm_reg|ymm_reg|xmm_reg)\)\n\s*/\1(\2) /g;"\
     -e "s/options\((.*)\)\);/options(\1)\n        );/g;"\
     -e "s/    \}\n/    }\n\n/g;"\
     target/expanded.rs > target/processed.rs

