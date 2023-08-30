# Limopack

1. [What is it?](#intro)
2. [Building](#building)
3. [Use Cases](#uc)
    1. [Determine Current Modules](#current-modules)
    2. [Freezing Modules](#freeze)
    2. [Flush Unnecessary Modules](#flush)
4. [Usage In Packaging](#pkg)
5. [Step-by-step Packaging](#sbs)
    1. [Step 1: The Kernel](#step-1)
    2. [Step 2: The Modules ](#step-2)
    3. [Step 3: Defining Vital Modules](#step-3)
    4. [Step 4: Defining Dynamic Modules](#step-4)
6. [Limitations](#limitations)

## What is it? <a name="intro" />

**Li**nux **mo**dule **pack**age (helper) is a packaging helper for the
situation, where only required Linux kernel modules are needed to be
present on a system.

The `limopack` is here to help you remove unnecessary kernel modules
on an embedded or any minimalistic setup, where there is no expectations
to pull an existing disk with a system and put it into a different hardware,
expecting it to successfully boot. Such behaviour is expected for the
mainline general purpose consumer system, but embedded.

## Building <a name="building" />

`limopack` is written in Rust and is bult in a usual way on a standard
Rust-enabled environemnt, version 1.66 or higher. You would need the following:

- Rust compiler `rustc`
- Cargo
- GNU make 4.3 or higher

To build `limopack` simply checkout this repo, and run inside it:

    make [release]


## Use Cases <a name="uc" />

## Use Case
Create a very tiny Linux kernel setup from the mainline Linux kernel,
where only necessary modules are present on the media, but achieveing
this without intervening into an existing kernel maintenance framework
or modifying upstream packaging.

Below are the main use cases for the `limopack` helper.
### Determine Current Modules <a name="current-modules" />

The `limopack` can (and should) determine which your modules are currently required
to be loaded next time your system boots and all their dependencies. To know this,
simply run module checker:

    limopack --list

This command will return you a list of all loaded modules in your system, denoted
as a relative path to the current kernel, expecting the following prefix for the
complete path:

    /lib/modules/<kernel-version>/

This command will only print-out a list of all currently required modules and their
relative paths.

### Freezing Modules <a name="freeze" />

In order to flush unneeded modules, all required ones needs to be "frozen" or denoted
as those that are needed. There are two kinds of needed modules:

- Static. This is a permanent, immutable list of modules that stays permanently on the system.
- Dynamic, on demand. These modules may be removed as long as they are no longer requested by
any other software component.

All modules are listed in `/lib/modules/<version>/modules.active` file. It has the
following format:

    module_name:<type>

Type can be either `S` for "static" and an a non-zero integer for "dynamic". The integer
denotes how many package references is in the system. With each package installation
this reference is increased by one, and with each package removal/purge it is decreased
by one. If the reference is thus decreased to zero, module is considered no longer needed
and therefore is removed from this list.

#### Add a Module

To add a module to the list, you need to know its _loaded name_ (which is different from
a _file name_), and it is done as follows:

    limopack --use=hci_nokia --install

This will add `hci_nokia` kernel module as a dynamic module, increasing the reference by one.
Repeating this command will _update_ `hci-nokia` dynamic module state, increasting the
reference by one, thus denoting there are two software components installed, requiring this
module to be present on a system.


#### Remove a Module

To remove a module from the list, you need to use the same _loaded module name_ as follows:

    limopack --use=hci_nokia --remove

If you added this module twice, first time it will decrease the reference count by one,
and the second time it will remove the module completely from the list.

#### Add a Static Module

To add `hci-nokia` kernel module as static, you need to add `--static` flag:

    limopack --use=hci_nokia --install --static

Or short version:

    limopack --use=hci_nokia -is

In this case `hci-nokia` module will be permanently added to the system and `limopack`
no longer will be able to remove it. An attempt to its removal will be logged as a warning
that such module is skipped.

#### Add All Necessary Modules

To add all vital modules that are currently loaded in the system, simply omit `--use` flag:

    limopack -i

The `limopack` will extract all current modules, find them on the disk and will register
all of them as static (in this case `--static` makes no influence).

### Flush Unnecessary Modules <a name="flush" />

Once modules are set, one needs to remove unnecessary modules from the system. However
this operation require a package name that needs to be hidden from the system as known,
even though its contents keeps being installed. This is a package, which contains
all the modules of the kernel:

    limopack --pkname=linux-modules-5.19.0-50-generic --apply

This command will do the following:

- Remove any mentioning of a package `linux-modules-5.19.0-50-generic` from the system,
so it will look like such package is not even installed.
- Remove all the modules and their dependencies, those are not mentioned in the active list.

This particular use-case can fit to an embedded image provisioning for "vacuuming" unnecessary
modules, using a package pattern or similar. For instance, installing such package will
install pre-set active static modules and flush all others. Such use-case is often popular
for one-time image provisioning, which is not supposed to be changed afterwards.

## Usage In Packaging <a name="pkg" />

In general, the setup supposed to be as follows:

1. Mainline Linux kernel package, containing no packages
2. A sub-package of that mainline Linux kernel, containing only packages for it

Without this requirement live dynamic tracking and/or updates are impossible.

## Step-by-step Packaging <a name="sbs" />

### Step 1: The Kernel <a name="step-1" />
We would need a kernel package, which contains the kernel itself, its config, all the
`/boot/*` parts etc. However this package should **not** contain any modules.

### Step 2: The Modules <a name="step-2" />
As a second step, we need a kernel _modules_ package, which could be a sub-package of the
kernel package, except it contains everything else but the kernel itself. This package may
contain all possible kernel modules or only essential ones etc.

Additionally, there might be more unlimited number of sub-packages, containing 3rd party
modules or any other additional modules, as long as they are installed to the same root
tree, yielding to `/lib/modules/<linux-version>/kernel` path.

### Step 3: Defining Vital Modules <a name="step-3" />

Now we need to define all vital modules that are still required to be on the system,
no matter what. This should be done in a transient package, which is not really installed
on the system, but only brings some elements and disappears from it.

At this point of time there is only one way of doing it: to have a live system running
with all modules installed and then determine from _live_ system which modules are actually loaded.
This is also current limitation of the `limopack`, unfortunately.

These modules as a list can be saved to `/lib/modules/<linux-version/modules.active` file.
This file then should be installed with this transient package. The same package on `%post-inst`
hook should call `limopack` with applying the changes and self-removal from the package manager
database records.

The transient package is also the main package, so this part could/should be in the Step 1
actually when you install the main kernel package.

### Step 4: Defining Dynamic Modules <a name="step-4" />

This is a separate step, and it is done per any other possible software component, which is
after extra installation of any additional modules, as well as their purge.

In general, when you are packaging whatever your software, which requires some other additional
module, you should do the following:

- Require a package, which contains that module, e.g. `linux-5.19.50-my-modules`. When your
package is installed, and so is that package as well, installed all extra modules somewhere
to `/lib/modules/<linux-version/kernel/mymodules/....` etc.
- In your `%install` section you setup all the `insmod` and other operations, needed so that
your module is automatically loaded after the reboot. The same hook should also call `limopack`
with `--use` and `--install` options. Whether it is `--static` or not, you should decide on your own,
but since you are making a package which is subject to update or uninstall, keep it dynamic reference
instead.
- In the `%post-inst` hook you need to call `limopack` with `--apply` option and `--pkname`
specifying `linux-5.19.50-my-modules` package as it would be never installed prior.

So when you will install your package, the following processes should happen in this order:
- `linux-5.19.50-my-modules` package is installed
- your software package is installed
- required modules from `linux-5.19.50-my-modules` are configured on the system
- those modules are also registered by the `limopack`, marking them with at least one reference
(this your current software component you are packaing in this Step #4)
- at the end `linux-5.19.50-my-modules` is marked as never installed (retaining its content on the disk),
and all other unused modules are purged from the disk

Why would you need to unregister `linux-5.19.50-my-modules` package? Because then the package manager
will be able to repeat the whole cycle, described above.

## Limitations <a name="limitations" />
The `limopack` is only a helper utility and currently works only on Debian family distributions.
It is intended to track required kernel modules and therefore help to install or remove them
on demand. This means that the Linux module state on the machine does not depend on the mainline
kernel update mechanisms and to reference a software component is a burden of that software
component.

Currently there is no way to determine which modules are vital for the system beforehand.
This is only possible to first provision full installation and examine it.

The current design of `limopack` at least as of today has no tracking of any additional data created
on the disk outside of package manager, thus lacks tracking of those files.
