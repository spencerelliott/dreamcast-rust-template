KOS_ROMDISK_DIR = ../romdisk
include $(KOS_BASE)/Makefile.rules

PROJECT_NAME=dc_rust

SOURCES := $(shell find "../src/" -name '*.rs') $(shell if [ -d "../kos-rust-bindings" ]; then echo "../kos-rust-bindings/wrapper/wrapper.c"; fi)
RS_OBJS = $(SOURCES:.rs=.o)
OBJS = $(RS_OBJS:.c=.o)

all: rm-elf copy-bindings $(PROJECT_NAME).elf

%.o: %.rs
	kos-cc -frust-incomplete-and-experimental-compiler-do-not-use $(CFLAGS) -c $< -o $@

copy-bindings:
	python3 ../kos-rust-bindings/generate.py -d ../kos-rust-bindings/src -o ../src

clean:
	rm -f $(PROJECT_NAME).elf $(OBJS)

rm-elf:
	rm -f $(PROJECT_NAME).elf
	rm -f romdisk.img

$(PROJECT_NAME).elf: $(OBJS)
	kos-cc -o $(PROJECT_NAME).elf $(OBJS)
	rm -f $(OBJS)
	rm -f *.rox

run: $(PROJECT_NAME)
	$(KOS_LOADER) $(PROJECT_NAME).elf

dist:
	rm -f $(OBJS) romdisk.*
	$(KOS_STRIP) $(PROJECT_NAME).elf
