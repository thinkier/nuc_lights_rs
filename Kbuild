obj-m := nuc_lights.o
nuc_lights-objs := nuc_lights.rust.o

CARGO ?= cargo

$(src)/target/x86_64-linux-kernel/debug/libnuc_lights.a: $(src)/Cargo.toml $(wildcard $(src)/src/*.rs)
	cd $(src); env -u MAKE -u MAKEFLAGS $(CARGO) build -Z build-std=core,alloc --target=x86_64-linux-kernel

%.rust.o: target/x86_64-linux-kernel/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<
