# Couldn't Create a Hypervisor
---

This error commonly occurs on machines that don't have virtualization enabled.

The way to fix this issue is to access the UEFI and enable virtualization.
The name depends on what motherboard you use, it could be called AMD-V, SMV, Intel VTT, Intel VT-d, and more! Always look at documentation if you don't know how to enable virtualization!

If the error persists on Windows, go to Windows Features and enable Virtual Machine Platform, then reboot.*
* Note, most Windows 10 and 11 installations come with VMP already installed, and only need the UEFI setting enabled.