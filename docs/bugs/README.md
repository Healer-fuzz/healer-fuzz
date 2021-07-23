# Previously-unknown bugs 

This directory lists previously-unknown bugs found by HEALER.
Each directory contains the complete crash information and reproducer program for the corresponding vulnerability.
We do not list all vulnerabilities and related linked in LKML because most patches for vulnerabilities are in the merge phase, and the reporting emails for these vulnerabilities are not yet publicly available due to security concerns.


[use-after-free Read in cdev_de](https://www.lkml.org/lkml/2021/4/4/35)

[memory leak in kvm_io_bus_unregister_de](https://lkml.org/lkml/2021/4/12/1985)

[kvm_vm_ioctl_unregister_coalesced_mmio](https://lkml.org/lkml/2021/4/12/1986)