# Kernel source directory
KERNEL_SRC := /lib/modules/$(shell uname -r)/build

# Module name
MODULE := simple_rust_driver

# Rust compiler and flags
RUSTC := rustc
RUSTFLAGS := --crate-type=cdylib

# Target
obj-m := $(MODULE).o

# All rule: Build the kernel module
all:
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules

# Clean rule: Clean up built files
clean:
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) clean

# Load the module
load:
	sudo insmod $(MODULE).ko

# Unload the module
unload:
	sudo rmmod $(MODULE)

# Create a device node (update <major_number> after loading the module)
devnode:
	sudo mknod /dev/$(MODULE) c <major_number> 0

.PHONY: all clean load unload devnode
