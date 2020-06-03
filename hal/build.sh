#!/usr/bin/env bash
highlight=$(tput setaf 214)
stopcolor=$(tput sgr0)

for mcutype in `ls ../pac/ | grep '.*b$'` ; do
    echo -e "\n--==[ ${highlight}${mcutype}${stopcolor} ] ]==--" 
    cargo build --features ${mcutype:2}
done

