Here we list the detail information about the bugs found by HEALER.
HEALER found **218** vulnerabilities, including **33** previously-unknown vulnerabilities.
Syzkaller and Moonshine did *not* find any previously-unknown vulnerabilities during our experiments. For
previously-found vulnerabilities, Syzkaller found 135 while Moonshine found 162, 112 of
which have been found by both tools.


## Previously-unknown bugs 

Table 5 in paper lists the new bugs exclusively detected by HEALER. All of them have been
confirmed and fixed. 
Each directory here contains the complete crash information and reproducer program for the corresponding vulnerability.


## Maintainer Response

The [maintainer-response](./maintainer-response) directory contains the *responses* and *confirmations* from kernel subsystem maintainers.

As we demonstrate below, we have actively communicate with the kernel maintainers, following figures are the conformance of the maintainers with some author information be hidden.

<img src=./maintainer-response/merge/merge1.png width="900"/>

<img src=./maintainer-response/merge/merge2.png width="900"/>

<!-- <img src=./maintainer-response/merge/merge4.png width="900"/> -->

<img src=./maintainer-response/merge/merge5.png width="900"/>

<!-- <img src=./maintainer-response/merge/merge6.png width="900"/> -->

<img src=./maintainer-response/merge/merge7.png width="900"/>

<img src=./maintainer-response/merge/merge3.png width="900"/>

<img src=./maintainer-response/merge/merge9.png width="900"/>

<img src=./maintainer-response/merge/merge10.png width="900"/>

<img src=./maintainer-response/merge/merge8.png width="900"/>


Also, we worked actively with the maintainer to fix bugs that HEALER found, following are some of the correponding patches we contribute to the Linux community.

<img src=./maintainer-response/merge/patch1.png width="900"/>

<img src=./maintainer-response/merge/patch2.png width="900"/>

<img src=./maintainer-response/merge/patch3.png width="900"/>

<img src=./maintainer-response/merge/patch4.png width="900"/>

<img src=./maintainer-response/merge/patch5.png width="900"/>