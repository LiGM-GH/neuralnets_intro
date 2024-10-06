import csv
import math

with open("data/main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)
    fun = input("f(x) = ")
    exec("def f(x): return " + fun)

    for elem in ([x / 100, f(x/100)] for x in range(100000)):
        the_writer.writerow(elem)
