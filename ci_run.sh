#!/usr/bin/bash 
export COMMON="-DUSE_NO_DEFAULT=TRUE  -DUSE_CLANG=True   " 
export PATH=/home/${USER}/.cargo/bin:/home/${USER}/.local/bin:$PATH
export PICO_SDK=/opt/pico/pico-sdk
export ROOT=$PWD

runbuild()
{
	local cur_dir=$PWD
	local folder_name="build${1}"
	shift
	rm -Rf $cur_dir/$folder_name
	mkdir $cur_dir/$folder_name
	cd $folder_name
	cmake ${COMMON} $@ ..
	make -j 4
	cd $cur_dir 
}

runbuild G3  ""

