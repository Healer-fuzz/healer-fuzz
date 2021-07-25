# Supplementary Experiments

This directory contains the result of 48-hours coverage comparision experiment (Figure 1-3) and 
the preliminary results of enhancing HEALER with distilled seeds of Moonshine (Figure 4). 

## Coverage Comparison Experiment in 48 hours 

Additionally, we performed **48-hour-long** experiments for all kernels.
We monitor the fuzzing process of Linux kernel versions 5.11, 5.4 and 4.19 
while comparing its performance to those of Syzkaller and Moonshine.
We sample each fuzzer’s performance statistics each minute in the 48-hour. 

<img src=./merged.PNG width="900"/>

As we can infer from the figure, all tested tools show significant growth in the first 8 hours then gradually slow down. At around 24 hours into the experiment, the relative performance of the tools can be established. Overall, HEALER’s performance gains persisted, achieving 15% to 28% more coverage than Syzkaller and Moonshine.

## HEALER + Moonshine Seed

In addition, we also performed 24-hour experiments to collect coverage data from HEALER + Moonshine Seeds (HEALER+).
We sample fuzzer’s performance statistics each minute in the 24-hour. 
In this supplementary experiment when enhanced with Moonshine’s distilled seeds, HEALER’s coverage improved by **5%** on average.

<img src=./HEALER+Moonshine_seed.png width="300"/>

From the above figure, we can see that HEALER has a better performance after augmentation with Moonshine's distillation seeds. In detail, HEALER has a sharper growth rate in the initial 4h and increases the coverage of HEALER by **5%** on average during the 24h experiment. 
HEALER benefits from the enriched initial seeds, which accelerate subsequent iterations of the relation table. 