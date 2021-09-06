
# Healer

**NOTE! This repo is no longer _maintained_, check out the latest progress of Healer [here](https://github.com/SunHao-0/healer) please.**

HEALER is a kernel fuzzer that improves fuzzing effectiveness by utilizing system call relation learning. 
HEALER learns the influence relations between system calls by dynamically analyzing the minimized  test cases. 
Then, HEALER utilizes the learned relations to guide input generation and mutation, which improves the quality 
of test cases and the effectiveness of fuzzing. 

## Docs

- [All the previously-unknown bugs](./docs/bugs)
- [All the learned relations](./docs/relations)
- [Test cases we used to verify the learned relations](./docs/corpus/README.md)
- [Supplementary experiments](./docs/graph)
- [Glossary and clarification of terminology](./docs/glossary_and_clarification_of_terminology.md)
- [Moonshine strong distill seed](./docs)

## Usage

### Build

Here we give step by step guidence to build and run Healer on Linux.

#### Install Rust 

Healer is written in pure rust, except for some ffi libraries. 
Go to official site of rust, download and install rustc and cargo. 
On linux, following command makes life easier:

``` shell 
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> rustc --version # check install
```

#### Builder Healer 

The process of building Healer is pretty straightforward, just executing following command.
*Note* Please ensure the connectivity of your network, as `Cargo` will download all the dependencies automatically.

``` shell 
> cargo build --release 
```

The executable file `healer` should be avaliable under `HEALER/target/release` directory if everything is ok
.
### Fuzz 

#### Quick Start

Execute following command to start Healer quickly.

``` shell
> healer --img path/to/disk-img --ssh-key path/to/ssh-key --target [target name]

```

Now Healer should be running, following log should be printed.

``` 

 ___   ___   ______   ________   __       ______   ______
/__/\ /__/\ /_____/\ /_______/\ /_/\     /_____/\ /_____/\
\::\ \\  \ \\::::_\/_\::: _  \ \\:\ \    \::::_\/_\:::_ \ \
 \::\/_\ .\ \\:\/___/\\::(_)  \ \\:\ \    \:\/___/\\:(_) ) )_
  \:: ___::\ \\::___\/_\:: __  \ \\:\ \____\::___\/_\: __ `\ \
   \: \ \\::\ \\:\____/\\:.\ \  \ \\:\/___/\\:\____/\\ \ `\ \ \
    \__\/ \::\/ \_____\/ \__\/\__\/ \_____\/ \_____\/ \_\/ \_\/

2021-02-28 12:24:25 [INFO] Loading target linux/amd64...
2021-02-28 12:24:25 [INFO] Revision: 0085e0
2021-02-28 12:24:25 [INFO] Syscalls: 3581/3960
2021-02-28 12:24:25 [INFO] Initial relations: 0
2021-02-28 12:24:25 [INFO] Booting 14 linux/amd64 on qemu...
2021-02-28 12:24:49 [INFO] code coverage               : enabled
2021-02-28 12:24:49 [INFO] setuid sandbox              : enabled
2021-02-28 12:24:49 [INFO] namespace sandbox           : enabled
2021-02-28 12:24:49 [INFO] fault injection             : enabled
2021-02-28 12:24:49 [INFO] net packet injection        : enabled
2021-02-28 12:24:49 [INFO] net device setup            : enabled
2021-02-28 12:24:49 [INFO] concurrency sanitizer       : enabled
2021-02-28 12:24:49 [INFO] USB emulation               : enabled
2021-02-28 12:24:49 [INFO] hci packet injection        : enabled
2021-02-28 12:24:49 [INFO] wifi device emulation       : enabled
2021-02-28 12:24:50 [INFO] Boot finished, cost 25s
2021-02-28 12:24:50 [INFO] Let the fuzz begin
2021-02-28 12:24:51 [INFO] Fuzzer-0: detected new relation: (socket$inet_tcp, getsockopt$inet6_mreq)
...
```

#### Configuration 

Healer supports flexible configurations, such as parallel instance number, virtual machine config, tes target, whitelist, etc.
Following is detailed configuration information.

``` shell 
> healer --help 
Healer 1.0.0
Relation learning guilded kernel fuzzer

USAGE:
    healer [FLAGS] [OPTIONS] --img <img> --ssh-key <ssh-key> --target <target>

FLAGS:
        --debug                     Debug mode
        --enable-relation-detect    Enable relation learning
    -h, --help                      Prints help information
        --skip-repro                Skip crash reproducing
    -V, --version                   Prints version information

OPTIONS:
        --disabled-calls <disabled-calls>    File path of disabled calls list
    -i, --img <img>                          Path to disk image
    -j, --jobs <jobs>                        Number of parallel instances [default: 2]
    -k, --kernel-img <kernel-img>            Path to kernel image, e.g. bzImage for linux kernel
    -O, --kernel-obj-dir <kernel-obj-dir>    Directory of object file of target kernel, e.g. vmlinux for linux kernel
    -S, --kernel-src-dir <kernel-src-dir>    Srouce file directory of target kernel
    -o, --out-dir <out-dir>                  Output directory, contains queues, crashes, relations [default: ./output]
        --qemu-mem <qemu-mem>                Size of memory for each qemu in megabyte [default: 2048]
        --qemu-smp <qemu-smp>                Number of cpu cores for each qemu [default: 2]
    -r, --relations <relations>              Specify the relations, [default: 'out_dir/relations']
        --ssh-key <ssh-key>                  Path to ssh key used for login to test machine
        --ssh-user <ssh-user>                User name for login to test machine [default: root]
    -d, --bin-dir <syz-bin-dir>              Binary file directory, syz-executor, syz-symbolize, syz-repro
                                             should be provided [default: ./bin]
    -t, --target <target>                    Fuzzing target in Os/Arch format, e.g. linux/amd64, linux/arm64
        --white-list <white-list>            White list of crash title

```
