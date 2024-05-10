#!/bin/bash
num_lectures=120
directory='slidoo'

mkdir -p ${directory}
cd ${directory}

for i in $(eval echo {3..$num_lectures});
do 
    echo "We are currently handling the ${i}. slide"
    curl -O https://telestudio.pro/presentation/kabardian/${i}.pptx
    unoconv -f pdf ${i}.pptx;
    rm ${i}.pptx;
    echo "------"
done

cd ..