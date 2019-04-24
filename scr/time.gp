#!/usr/bin/gnuplot --persist

f(x) = a*x + b
fit f(x) ARG1 u 1:2 via a, b
title_f(a,b) = sprintf('%.2fx + %.2f', a, b)
plot ARG1 u 1:2 title "Time (ns)" with points pointtype 0, f(x) t title_f(a,b)
