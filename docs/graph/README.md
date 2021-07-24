# Supplementary Experiments

We list the supplementary experiments in this directory. 
We monitor the fuzzing process of Linux kernel versions 5.11, 5.4 and 4.19 while comparing its performance to those of Syzkaller and Moonshine. We sample each fuzzer’s performance statistics each minute in the 48-hour. 
This directory contains the result of 48h coverage comparision experiment.

## Coverage Comparison Experiment in 48 hours 
<img src=./merged.PNG width="900"/>

As we can infer from the figure, all tested tools show significant growth in the first 8 hours then gradually slow down. At around 24 hours into the experiment, the relative performance of the tools can be established. Overall, HEALER’s performance gains persisted, achieving 15% to 28% more coverage than Syzkaller and Moonshine.
<!-- <img src=./linux-v5.11.png width="300"/> <img src=./linux-v5.4.png width="300"/> <img src=./linux-v4.19.png width="300"/> -->

<!-- ![linux-v5.11](./linux-v5.11.png)
![linux-v5.4](./linux-v5.4.png)
![linux-v4.19](./linux-v4.19.png)  
![HEALER+Moonshine_seed](./HEALER+Moonshine_seed.png)
-->

## HEALER + Moonshine Seed

<img src=./HEALER+Moonshine_seed.png width="300"/>

HEALER benefits from a enriched initial seed, which accelerates the subsequent iterations of the relation table. From the above figure, we can see that HEALER has a better performance after augmentation with Moonshine's distillation seeds. In detail, HEALER has a sharper growth rate in the initial 4h and increases the coverage of HEALER by 5% on average during the 24h experiment.
