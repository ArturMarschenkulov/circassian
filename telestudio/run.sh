#!/bin/bash

num_lectures=120

mkdir -p slides
cd slides
for i in $(eval echo {0..$num_lectures}); 
    do curl -O https://telestudio.pro/presentation/kabardian/${i}.pptx; 
done
cd ..