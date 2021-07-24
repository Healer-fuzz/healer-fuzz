# Supplementary Experiments

We list the supplementary experiments in this directory. 

## Coverage Comparison Experiment in 48 hours

Additionally, we performed **48-hour-long** experiments for all kernels.
We monitor the fuzzing process of Linux kernel versions 5.11, 5.4 and 4.19 
while comparing its performance to those of Syzkaller and Moonshine.
We sample each fuzzer’s performance statistics each minute in the 48-hour. 

<img src=./merged.PNG width="900"/>

As we can infer from the figure, all tested tools show significant growth in the first 8 hours then gradually slow down. At around 24 hours into the experiment, the relative performance of the tools can be established. Overall, HEALER’s performance gains persisted, achieving 15% to 28% more coverage than Syzkaller and Moonshine.

## HEALER + Moonshine Seed

In addition, we also performed 24-hour experiments to collect coverage data from HEALER+Moonshine Seeds.
We sample fuzzer’s performance statistics each minute in the 24-hour. 

![HEALER+Moonshine_seed](./HEALER+Moonshine_seed.png)

HEALER benefits from the enriched initial seeds, which accelerate subsequent 
iterations of the relation table. 
In this supplementary experiment when enhanced with Moonshine’s distilled seeds (*HEALER+*), 
HEALER’s coverage improved by **5%** on average.