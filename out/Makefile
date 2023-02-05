KOS_ROMDISK_DIR = ../romdisk
include $(KOS_BASE)/Makefile.rules

PROJECT_NAME=dc_rust

SOURCES := $(shell find "../src/" -name '*.rs') ../wrapper/wrapper.c
RS_OBJS = $(SOURCES:.rs=.o)
OBJS = $(RS_OBJS:.c=.o)

all: rm-elf $(PROJECT_NAME).elf

%.o: %.rs
	kos-cc -frust-incomplete-and-experimental-compiler-do-not-use $(CFLAGS) -c $< -o $@

clean:
	rm -f $(PROJECT_NAME).elf $(OBJS)

rm-elf:
	rm -f $(PROJECT_NAME).elf
	rm -f romdisk.img

$(PROJECT_NAME).elf: $(OBJS)
	kos-cc -o $(PROJECT_NAME).elf $(OBJS)
	rm -f $(OBJS)

run: $(PROJECT_NAME)
	$(KOS_LOADER) $(PROJECT_NAME).elf

dist:
	rm -f $(OBJS) romdisk.*
	$(KOS_STRIP) $(PROJECT_NAME).elf
