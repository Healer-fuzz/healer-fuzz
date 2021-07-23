
# Healer

HEALER is a kernel fuzzer that improves fuzzing effectiveness by utilizing system call relation learning. 
HEALER learns the influence relations between system calls by dynamically analyzing the minimized  test cases. 
Then, HEALER utilizes the learned relations to guide input generation and mutation, which improves the quality 
of test cases and the effectiveness of fuzzing. 

## Docs

- [All the learned relations](./docs/relations)
- [All the previously-unknown bugs](./docs/bugs)
- [Test cases we used to verify the learned relations](./docs/corpus/README.md)
- [Supplementary experiments](./docs/graph)
- [Glossary and clarification of terminology](./docs/glossary_and_clarification_of_terminology.md)