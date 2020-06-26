#build
build:
	@cmake -Wnodev -B build/ -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang . && /usr/bin/ninja -C build
#clean project
clean:
	@rm -rf build/* .dist
	@rm -rf 3rdparty/*
#delete logs
dlog:
	@rm -v **/*.log
#get deps
getd:
	@bash scripts/getdeps.sh
	@bash scripts/builddeps.sh
#kill running chuck vms
killck:
	@sudo killall chuck
#run tokei
toke:
	@tokei
# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
