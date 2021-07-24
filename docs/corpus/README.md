This directory lists all the examined test cases that we used to verify the correctness of learned relationes.
Each enrty in this directory contains detailed information about call sequence.

For instance, file *2290e5edd75af6b7fe5b6cbe624fe63db3a3c89c* contains following call sequence, 
[socket$nl_generic, syz_genetlink_get_family_id, sendmsg$SMC_PNETID_ADD].

```
r0 = socket$nl_generic(0x10, 0x3, 0x10)
r1 = syz_genetlink_get_family_id$smc(&(0x7f0000000380)='SMC_PNETID\x00', 0xffffffffffffffff)
sendmsg$SMC_PNETID_ADD(r0, &(0x7f00000004c0)={0x0, 0x0, &(0x7f0000000480)={&(0x7f00000003c0)={0x34, r1, 0x1, 0x0, 0x0, {}, [@SMC_PNETID_NAME={0x9, 0x1, 'syz2\x00'}, @SMC_PNETID_ETHNAME={0x14, 0x2, 'syzkaller0\x00'}]}, 0x34}}, 0x0)
```