This directory lists all the examined test cases that we used to verify the correctness of learned relationes.
Each enrty in this directory contains detailed information about call sequence.

For instance, file [2c0cb3720ecabc52aff1f94b9da2b9cdc509c9b4](./2c0cb3720ecabc52aff1f94b9da2b9cdc509c9b4) contains following call sequence, 
[socket$inet, setsockopt$inet_tcp_int, bind$inet, sendto$inet, setsockopt$sock_int, write$binfmt_elf64].

```
r0 = socket$inet(0x2, 0x4000000000000001, 0x0)
setsockopt$inet_tcp_int(r0, 0x6, 0x80000000000002, &(0x7f0000000300)=0x80, 0x4)
bind$inet(r0, &(0x7f0000000000)={0x2, 0x4e23, @broadcast}, 0x10)
sendto$inet(r0, 0x0, 0x0, 0x200007fd, &(0x7f0000e68000)={0x2, 0x4e23, @local}, 0x10)
setsockopt$sock_int(r0, 0x1, 0x2f, &(0x7f0000000500)=0x9, 0x4)
write$binfmt_elf64(r0, &(0x7f0000000100)=ANY=[], 0x2bcf)
```