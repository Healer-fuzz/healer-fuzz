# Section 1 

## “Deeper Information”
Q: "does not contain any deeper information" What does "deeper information" mean?

A: Sorry for the confusion. The deeper information here means implicit semantic information of syscalls. For instance, the fact that the  execution of one syscall can influence another syscall’s execution path is a kind of deeper information, since it relies on hidden kernel states that cannot be explicitly expressed by a syscall’s interface. Influence relation between syscall *fcntl$ADDSEAL* and syscall *mmap* as shown in Figure 2 is an example.
## “Item in Choice-table”
Q: each item in the choice-table cannot convey the influence relation" We don't know what is an item in a choice table, and don't really know what an influence relation is, so this is hard to fully understand.

A: Each item of the choice-table records the probability of a syscall being invoked before another. The table helps to generate more meaningful syscall sequences by prioritizing related calls. For example, if *read* usually occurs after *open*, then we generate the sequence of [open, read] with more weights. See the third paragraph in Section 1; a detailed explanation is also available in Section 3.
## “Iteratively”
Q: "which is refined iteratively and will be" What is meant by iteratively? Iteratively as what happens?

A: *iteratively* means HEALER constructs the relation table step by step during the fuzzing campaign. More specifically, HEALER performs online learning to refine the learned relations. However, new relations cannot be learned without test cases demonstrating interesting behaviors. Therefore, HEALER interleaves the learning and generation algorithms. It first constructs the initial relations by static analysis, then generates new syscall sequences by using the existing relations. Among all generated sequences, the interesting ones are discovered according to coverage. HEALER then learns new relations from the high-quality sequences. The generation and learning steps are invoked by turns -- thus “iteratively”.

## “Deeper Logic”
Q: "the kernel’s deeper logic" Deeper than what? In general, it is not clear what "deep" means.

A: Conventional syscall fuzzers only try to bypass shallow checks such as structural validation and semantic checks. However, since there is limited investigation towards generating correct syscall sequence, the execution of syscall sequence is likely to be stopped by deeper check logics, including the kernel-state dependent semantic checks.

Apart from using coverage to guide the generation part as conventional fuzzers, HEALER reveals the semantics of system calls by using learned relations. Therefore, it can trigger deeper logic instead of the checks focusing on shallow input conformity.

## “Essential Influence Relation”
Q: What is the essential influence relation.

A: Sorry for the typo. Here “essential” means important. It has the same meaning as “influence relation”. We will polish the paper accordingly.
## “Interesting Test Case”
Q: "whenever an interesting test case is discovered" What is an "interesting test case"? Is this the same point that was previously made about new coverage?

A: Yes, an interesting test case is a test case that covers new edges/blocks. In a fuzzing campaign, the exploration of program states is based on feedback of program behaviors, where coverage is one of the most used metrics. For example, if a program executes a previously-unknown code region (basic block) or demonstrates a new control-flow transfer (edge), then the fuzzer stores the input as an interesting test case for further analysis and mutation. Fuzzer can obtain and derive more information of program behaviors by analyzing the saved input, e.g., learning relation between syscalls, as well as generating new test cases that have higher probability to trigger new execution paths by mutating test cases that cover new edges/blocks. Therefore, this kind of test case is called “interesting test case” 
# Section 2 
## Seed Distillation of Moonshine
Q: Could explain what seed distillation is.

A: Seed distillation is a mechanism for initial seed generation proposed by Moonshine. It uses static analysis to extract read-write dependence between syscalls. It then collects sequences of executed system calls during the execution of existing test cases. Finally, Moonshine distills the collected syscall sequences with obtained dependencies. Syscalls that do not have read-write dependencies with other calls in the sequence will be removed, which improve the quality of collected sequences. Call sequences distilled by the above process can be used as initial seeds for kernel Fuzzer.

Different from Moonshine, HEALER learns the influence relations between syscalls with dynamic learning and guides the generation and mutation with learned relations during the whole fuzzing campaign.

## Purpose of Including QEMU

Q: "combining AFL and QEMU’s full-system simulation." What is the purpose of including Qemu?

A: The QEMU full system simulation is used to run the kernel because the kernel runs in privileged mode and manages the hardwares. Since an OS kernel interacts directly with the hardware, it is impossible to fuzz OS kernels as normal programs. Therefore, we simulate the hardware with QEMU and communicate with it via shared memory and virtualized network interface cards.

## Subject of “Guide”
Q: "kernel to consume and guides" It is not clear what is the subject of "guides". Maybe it is "driver", but that is quite far away in the rather complicated sentence.

A: The original sentence, subject of ‘kernel to consume and guide’ is the driver. We are sorry for the long and complicated original sentence (“Then, a fuzzing driver is utilized to decode the inputs generated by AFL for the kernel to consume and guide the fuzzing process.”). The sentence should be polished as:

“ To fuzz a kernel with AFL, the fuzzer injects an agent process to the guest virtual machine. The agent process communicates with the outside fuzzer, receives the message sent from the fuzzer, decodes them into a sequence of syscalls, and issues the syscalls to the kernel accordingly. Furthermore, it also performs bookkeeping functions to drive the fuzzing loop.”

We will further polish the paper by splitting the sentences for clarity.

## “Coverage-guided Kernel Fuzzers”

Q: "Furthermore, some coverage-guided kernel fuzzers" So the previously specifically mentioned systems are not coverage guided?

A: Most fuzzers mentioned before "Furthermore, some coverage-guided kernel fuzzers" are coverage-guided, too. The use of “coverage-guided” is aimed at emphasizing the co-exist of generation and mutation of the mentioned fuzzers inside the sentence. Different from the aforementioned fuzzers using coverage to guide byte-level mutations, the fuzzers mentioned in this sentence generate a meaningful sequence of syscalls like what HEALER does.

## “Syzlang”
Q: Syzlang supports system call specializations, which instantiates -> Syzlang supports system call specializations, which instantiate It is not really clear what is the importance of this information in the context of this paper.

A: We mention Syzlang in this context because of the need for an explanation for the number of specialized syscalls. For generation-based fuzzer, the quality of the generation specification is crucial for the overall fuzzing performance. For the ease of specification building, Syzkaller introduces a domain-specific language, Syzlang. Since HEALER directly reuses Syzkalle’s specification, it is necessary to describe the usage of Syzlang in the paper. To explain why the number of specialized syscalls is more than the number of original syscalls in Linux, we give a brief introduction for syscall specialization, a language feature in Syzlang. 

## “Find More Kernel Bugs”
Q: "its ability to find more kernel bugs" More than what?

A: The fuzzers being compared against Syzkaller are hobby projects from individual security researchers and academic works, such as kAFL, TriforceAFL, and Moonshine. Reasons behind Syzkaller’s more bug findings is Google’s continuous investment, including maintaining the syscall specification and running on server clusters. Backed by a large corporation, it is not surprising that Syzkaller reports more bugs to the upstream kernel community.
# Section 3
## “Influence Relation” and “Kernel State”
Q: "kernel states, which often hide deep and hard-to-find kernel vulnerabilities," How does a state hide a vulnerability?

A: Sorry for the confusion. Kernel state refers to the data maintained by the kernel. Influence relations between syscalls refer to whether the execution of one syscall can affect the execution path of another syscall. Influence relations are external indicators of shared internal kernel state between calls.

For instance, a successful *open* call will add an entry to the list of the process’s opened files (shared kernel state). The entry may be accessed by subsequent calls, such as *read*, *write*, or *fcntl*.  *fcntl* may change the state of the opened file thus influencing the following calls *read* and *write*.

## Interaction Between Syscalls from Different Modules
Q: Is it guaranteed that if two functions are defined in different modules that there is no interaction between them? I think that some modules may have dependencies on other modules?

A: Yes, syscalls from different modules may still have an influence on each other. For example, the network stack may halt the execution of a thread due to blocking IO. In this example, the network module influences syscalls related to the scheduler.

## Reserved
Q: "The reason that 𝐶 𝑖 is reserved in the" What does "reserved" mean here?

A: “reserved” here means not being deleted by the minimization algorithm. The original call sequences generated by fuzzer may contain redundant calls due to the randomness. Minimization algorithm will remove calls in a sequence that do not contribute to the new coverage and only keep the necessary calls. For instance, syscall *getpid* will be removed from call sequence [open, getpid, write], and syscall *open* will be reserved.
# Section 4.1

## Comprehensiveness
Q: "aware of the relationships between system calls without sacrificing comprehensiveness." It is not clear what is the problem being addressed. Where is the risk of sacrificing comprehensiveness?

A: HEALER utilizes relations to prioritize input generation of related syscalls. However, wrong prioritization can reduce the search space of a fuzzer, thus reducing the ability to detect the set of bugs. Therefore, the proposed algorithm contains two parts to ensure the same search space. First, the static analysis part bootstraps the algorithm fastly, while the dynamic analysis refines the generated relations with high precision.
## “Global State”
Q: Is the kernel's internal state necessarily global state?

A: No. Most modern kernels follow the hybrid design, where the performance-sensitive part inherits the “monolithic kernel” design, and the other part can be placed into user space with IPCs. Therefore, a kernel’s state can be divided into many small submodules. For example, the Linux kernel’s device driver can be dynamically loaded to the kernel space, and its internal state becomes a self-contained part of the whole global kernel state.

## Influence Relation at Beginning of Fuzzing
Q: "At the beginning of the fuzzing process, the fuzzer may not be aware of all system call influence relations" Why would it be aware of any influence relations?

A: HEALER can be aware of influence relations at the beginning of a fuzzing campaign with the help of static analysis. Specifically, the relation learning of HEALER contains two parts. First, before fuzzing starts, the static analysis of HEALER can learn the relations by performing static analysis on predefined syscall specifications from Syzkaller. Although the relations learned by static analysis are incomplete and may contain wrong entries, they still provide valuable initial fuzzing directions to bootstrap fuzzing.

## The Referred Target of “its”

Q: "minimized system call sequences and its feedback information" What does "its" refer to?

A: “its” refers to “system call sequences”. We are sorry that the original sentence, “The dynamic routine of the algorithm derives the influence relations by executing and analyzing the minimized system call sequences and its feedback information, which finds deeper influence relations.”, is unclear. Put it more clearly, we would like to rewrite the sentence as follows to avoid confusion:

“ The dynamic routine of the algorithm can find influence relations not expressible by static syscall specifications. To do so, the algorithm first minimizes the syscall sequences, then executes the minimized sequence to obtain the coverage feedback for individual syscalls, and finally analyze the corresponding feedbacks to determine the influence relations.”
## Inheritance and Subtyping
Q: What do inheritance and subtyping mean for C code? Perhaps more has to be explained about Syzlang?

A: Thanks for the note. The context of the explanation of “inheritance” and “subtype” is Syzlang. In Syzlang, resources are assigned with types. For example, the action of opening a KVM device to setup virtualization returns a “KVM” type of resource. Since the KVM device is a subtype of file descriptor, syscalls accepting objects of “fd” type also accept KVM devices. With subtyping of Syzlang, fuzzer can automatically generate generic file descriptor syscalls such as “close” when a KVM device is opened.

## Resource Semantic
Q: "contains resource semantics" What does it mean?

A: Many types of syscalls rely on an existing resource, such as an opened file, established network connection, and created device control channel. By introducing semantics to resources, Syzlang can establish orders between resource-creating syscalls (e.g. open, socket) and resource-using syscalls (e.g. sendto, read). Although semantics can prune many unrelated syscalls, deep semantics hidden in kernel logic cannot be easily expressed. The dynamic analysis part of HEALER can detect richer semantics inexpressible by resource semantics.

## Relation Between “fcntl$ADD_SEALS” and “memfd_create”

Q: “How do we see that fcntl$ADD_SEALS from memfd_create?”

A: We can derive the influence relation between syscall *fcntl$ADD_SEAL and syscall *memfd_create* with static analysis of relation learning. Specifically, the type of the first parameter of “fcntl$ADD_SEALS” is resource type, which is the same as the return type of syscall “memfd_create”. Using the rules of static analysis introduced in Section 4,, we can derive that syscall *memfd_create* can influence the execution path of syscall *fcntl$ADD_SEALS*.

##  “More” Relations or “More” Explicit?

Q: "of more explicit influence relations" Does "more" modify explicit or relations? More than what? It is not clear how the first part of the sentence connects to the second part.

A: “more” modifies “explicit”. If a relation can be derived from static analysis of syscall specification, then it is more explicit than another relation which can only be derived from dynamic analysis of syscall sequences. The difference in explicitness can be explained by the limited expressiveness of Syzlang: Syzlang can express only a small part of relations such as creating a file is the prerequisite for reading it. Therefore, HEALER can obtain a limited set of relations by performing static analysis, supposing the relation is explicitly stated in the Syzlang specification.

##  Data Structure of Coverage
Q: "and the list of new coverage" Coverage is a number, so how can it be a list?

A: Sorry for the confusion. Indeed, the coverage of each basic block or edge is represented as the number of times that the basic block or edge is executed. The paper focuses on the organization of multiple basic blocks or edges, rather than storing the hit count of one basic block or edge. Here, HEALER stores the executed syscall as a sequence of identifiers denoting the triggered basic block or edge.

##  How Reverse Order Help?
Q: "triggered new coverage in reverse order (Lines 3-7). This ensures that the resulting subsequences are independent and non-repetitive." How is taking something in reverse order helpful?

A: Reverse order ensures that the minimization algorithm won’t produce redundant sequences. More specifically, suppose the coverage of syscall sequence [open, fcntl, write] is [cov0, cov1, cov2],  in which both cov1 and cov2 contain new covered branches/edges. With the normal order, [open, fcntl, write] would be minimized as [open fcntl] and [open, fcntl, write]. With reverse order, the result would be only [open, fcntl, write] (see Algorithm 1) and the shorter sequence [open, fcntl] will be skipped.
## What Does "minimized once" Mean? 
Q: "p' is successfully minimized once" What does "minimized once" mean? Minimization should be idempotent. Once something is minimized, minimizing it again should give the same result.

A: MInimization algorithm uses multiple iterations to minimize the call sequence, “minimized once” is the intermediate result of one iteration. Specifically, suppose the coverage of syscall sequence [open, socket, read, listen] is [cov0, cov1, cov2, cov3], in which cov3 and cov2 contain new branches. According to the Algorithm 1, after the first iteration of the minimization algorithm, [open, socket, read, listen] would be [open, socket, read], yielding [socket, listen], the sequence is said to be minimized once. After the next iteration of minimization, [open, socket, read] would be [open, socket], yielding [open, read]. The minimization would be finished in the next iteration since both cov0 and cov1 do not contain new coverage.

## Non-consecutive Calls
Q: "non-consecutive calls’ removals cannot demonstrate the influence of relations." Why not?

A: The reason is the intermediate calls in non-consecutive calls might influence the later calls, which decreases the accuracy of learned relations. For example, consider the syscall sequence [C1, C2, C3]. When removing C1, the coverage changes of C3 could not indicate that C1 has an influence on C3 because the intermediate call C2 could also change the behavior of C3.

## Continuously in What Sense?
Q: "HEALER can continuously refine the relation table," Continuously in what sense?

A:  “continuously” indicates the whole fuzzing process. The dynamic analysis of relation learning is invoked whenever new coverages are discovered. As a result, the new relations could be learned in the whole fuzzing process. In other words, HEALER could use the learned relations to guide the generation and mutation incessantly.

# Section 4.2 
## The additional uses of learned relations
Q: "In HEALER, relations are mainly used to guide the mutation and generation of system call sequences." What else could they be used for? At this point in the paper, it would be better to focus on what you actually do. Other uses of the learned information could be in the conclusion/future work.

A: Except for using relations to guide the generation and mutation, these learned relations could also be used to help experts to correct and enrich the specifications. With these specifications, other fuzzers could better test kernels. One can also use the learned relations to distill call sequences and provide high quality initial seed for fuzzer. We will add these possible uses into the revision.
## Preserved Call Sequences
Q: "These are preserved because of their" Why would they not be preserved?

A: HELER only preserves the system call sequences which cover new branches. Preserving these system call sequences helps to utilize the latest finds and expand the possibility to find new branches, thus improving the efficiency and bug discovery capability of fuzzer. For others, since they do not contribute to coverage, mutating them will cost lots of resources but gain less.

##  Specificity of Generation Process
Q: "At the beginning of the generation process, HEALER mainly considers the producer and consumer of resource types based on the information provided by the Syzlang descriptions." Is what is described here specific to HEALER?

A: Sorry for the confusion. The process is common for most kernel fuzzers such as Syzkaler. They all consider the calls of producer and consumer to combine them and refine new call sequences. Differently, HEALER also guides the generation with the relation learned by dynamic analysis. Based on these relations, HEALER can generate test cases to access the deeper logic of the kernel.


# Section 6
## Meaning of “Practical”
Q: "A practical kernel fuzzer should be able to cover" What does "practical" mean here?

A: The “practical” means the fuzzer to work normally against real Linux kernels for a long time and trigger real vulnerabilities.

## Metrics to evaluate fuzzer
Q: "should be able to cover the execution path of the kernel under test as much as possible efficiently while triggering as many kernel vulnerabilities as possible." Perhaps coverage is not so important? Some parts of the kernel may be very rarely (or even never) executed in practice. So finding more vulnerabilities could be more important than achieving the highest coverage.

A: Yes, the ability to trigger vulnerabilities is an important metric to measure the performance of a kernel fuzzer. In the 10 rounds of the 24-hour experiment, HEALER found 32 bugs, whereas Moonshine, Syzkaller, and HEALER found 20, 17, and 10 bugs, respectively. However, the coverage is still an unneglected metric to evaluate fuzzers. Many fuzzers evaluate platforms (e.g. FuzzBench) all use coverage as the metric. For branch coverage, HEALER achieves 27.75% and  20.89% improvement to Syzkaller and Moonshine, respectively.


## Experiments for RQ3
Q: RQ3 seems much more important than RQ1. It should be more clear that RQ3 also involves comparison to existing tools.

A:  In Section 6.3, we compare the bug discovery capability among HEALER, Syzkaller, and Moonshine. Syzkaller and Moonshine did *not* find any previously-unknown vulnerabilities. For previously-found vulnerabilities, Syzkaller found 135 while Moonshine found 162, 112 of which have been found by both tools.
HEALER found 218 vulnerabilities, including 185 previously-found vulnerabilities and 33 previously-unknown vulnerabilities.
Table 5 lists the new bugs exclusively detected by HEALER. All of them have been confirmed and fixed. See [here](https://github.com/Healer-fuzz/healer-fuzz/tree/main/docs/bugs) for more detailed information.

# Section 6.1
## Research Question 1

Q: Is this section about research question 1?

A: Yes, Section 6.1 compares the performance of HEALER to Syzkaller and Moonshine, which is related to research question 1.

##  Performance
Q: "while comparing its performance" Performance in what sense?

A: Sorry for the confusion. The performance here indicates the coverage and the bug finding capability in 24-hour experiments.

# Section 6.2
## Relation between number of learned relations and Syzlang descriptions 

Q: "The result is reasonable because most of the system calls in the Syzlang description are specialized calls". Does this statement relates how to the previously mentioned numbers?

A: Yes, the Syzlang descriptions contain a total of 3579 system calls, and HEALER uses the specification as-is (revision [0085e0](https://github.com/google/syzkaller/tree/0655e081f42239d4eca4345ef7293307085f78f5/sys/linux). On these system calls, HEALER finds about 6320 relations on average in Linux-5.11 (see [here](https://github.com/Healer-fuzz/healer-fuzz/blob/main/docs/relations) for details).

## Relation between the length and the complexity

Q: "Longer sequences imply that there are more complex combinations". Not sure that longer is necessarily more complex.

A: Since only syscalls that contribute to new coverage will be saved after minimization, the longer sequence might contain more types and combinations of syscalls, which indicates more complicated sequences. 

## Numbers in Figure 6

Q: Figure 6: There are numbers at the top of the bars (that are very hard to read). What do they represent?

A: The number at the top of each bar means the percentage of call sequence with the corresponding length in the corpus. For instance, the percentage of sequences longer than 3 in HEALER, Syzkaller, and Moonshine is 46%, 21%, and 25%, respectively. This indicates that HEALER can produce more sophisticated test cases, further justifying HEALER’s performance gains.
## Patches

Q: "where the corresponding patches were added and" What are the corresponding patches? Patches that fix the problem, or that introduced the problem?

A: The “corresponding patches” means the patches that fix the bugs. Table 5 lists the new bugs exclusively detected by HEALER, and all of them have been confirmed and fixed by the Linux kernel maintainers. See [here](https://github.com/Healer-fuzz/healer-fuzz/tree/main/docs/bugs) for the previously-unknown bug reports.

# Section 7
## The call sequences do not generated by Syzkaller

Q: Why would Syzkaller not be able to generate the call sequence used in this section?

A: Syzkaller is hard to generate the call sequences because the certain syscall combination is rare and complicated. With the relation learned by HEALER, it could capture the dependency between these syscalls to generate the sequence and further trigger the vulnerability.
