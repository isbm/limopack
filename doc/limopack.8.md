% LIMOPACK(8) Version 0.1

NAME
====

**limopack** - *Li*nux *Mo*dule *Pack*age (Helper)

SYNOPSIS
========

Usage of the limopack as follows: `limopack [OPTIONS]`

DESCRIPTION
===========

The **limopack** is a Linux Kernel modules packaging helper, which
allows to minimise the amount of mainline kernel modules present on
the disk, if they are not needed, without altering the entire
framework for the mainline kernel management and support.

Main demand of this helper is in the embedded world, where one need to
be able to construct a bootable image for a specific device, using a
standard mainline kernel with all modules present. Mainline kernels
are usually coming with many other modules that normally are never
required in an embedded world or never will be required after a system
has been provisioned.

This helper works by altering package manager database (Debian),
allowing physical post-install removal of modules, those are actually
always unused and unloaded, but present on the disk. Altering package
manager is needed to allow next update cycle, letting system bring the
kernel/modules updates as usual.

In a nutshell, **limopack** works as follows:

1. All mainline kernel modules should be required to be installed
2. **limopack** then only marks explicitly specified modules those are
   actually needed (it can be a package pattern etc)
3. The difference is removed from the disk (unused modules)
4. Kernel modules package is then marked as outdated (lower version)

When a new update comes, modules package is then brought back for update.

OPTIONS
-------

-u, --use <use>

: Specify comma-separated list of kernel modules to be processed. For example
: you can specify **--use=module1,module2,module3** etc.

-s, --static

: Use specified modules as static (i.e. stays permanently)

-e, --tree

: Display module dependency tree.

-l, --list

: Display in a sorted flat list format all modules that will
: be used. This includes all dependencies and already marked
: and existing modules.

-p, --pkname <pkname>

: Specify a package name, which needs to be un-registered
: from the package manager database in order to be visible to the system as
: non-existing, so the system can bring it again for an update or installation.

-i, --install

: Mark specified modules as needed for the system.

-r, --remove

: Remove specified modules as no longer needed for the system,
: so they can be purged from the disk. This operation only marks
: the modules to be removed, but does not actually removes them.

-a, --apply

: Apply the changes, vacuuming all unneded/unregisterd (non-marked)
: kernel modules, those are still exist on a disk, but always unused.
: *NOTE: this option can be only used alone, as it commits the changes*

-d, --debug

: Set debug mode for more verbose output.
-v, --version

: Get current version.
-h, --help

: Print help

FILES
=====

*/usr/bin/limopack*

:   Main runtime binary


EXAMPLES
========

Only to display dependencies to specific kernel modules:

    $ limopack --use=hci_nokia,ltc3815,9pnet_xen,snd-soc-skl-ssp-clk -e


To register `ltc3815` module to be used:

    $ limopack --use=ltc3815 --install

Note, those modules are only added to the list of used modules. To
cleanup all other modules (unused):

    $ limopack --shrink

BUGS
====

See GitHub Issues: https://github.com/isbm/limopack/issues

AUTHOR
======

Bo Maryniuk

COPYRIGHT
---------

(c) 2023, Bo Maryniuk
