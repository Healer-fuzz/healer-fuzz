
# Healer

HEALER is a kernel fuzzer that improves fuzzing effectiveness by utilizing system call relation learning. 
HEALER learns the influence relations between system calls by dynamically analyzing the minimized  test cases. 
Then, HEALER utilizes the learned relations to guide input generation and mutation, which improves the quality 
of test cases and the effectiveness of fuzzing. 
We evaluated its performance on recent versions of the Linux kernel. 
Compared to state-of-the-art kernel fuzzers such as Syzkaller and Moonshine, HEALER improves branch
coverage by 27.75% and 20.89%, while achieving a speedup of 2.24× and 1.85×, respectively. 
In addition, HEALER detected 218 vulnerabilities, 33 of which are previously unknown and
confirmed by the corresponding kernel maintainers.

## 48h Experiements



## Previously-unknown Vulnerabilities
