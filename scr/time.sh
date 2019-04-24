#!/bin/sh

clear
../target/release/SLICK -t $1 $2 > .temp.csv
echo
gnuplot -p -c time.gp .temp.csv
rm fit.log .temp.csv
echo 
