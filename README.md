# Linmodpak

**Lin**ux **mod**ule **pack**age helper is a package helper to remove
unnecessary kernel modules on embedded or minimalistic setup, where
there is no expectations to put an existing disk into a different
hardware and expect it to boot.

## Use Case
Create a very tiny Linux kernel setup from the mainline Linux kerne,
where only necessary modules are present on the media, but achieveing
this without intervening into an existing kernel maintenance framework
or modifying upstream packaging.

## Principle
The Linmodpak works by installing a mainline kernel with all supported
modules and then remove those, that are not needed.

## Limitations
The Limodpak is only a helper, which is used to track used modules and
or install/remove them on demand. This means that the Linux module
state on the machine does not depend on the mainline kernel update
mechanisms.
