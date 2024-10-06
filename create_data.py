import csv
import math # pyright: ignore[reportUnusedImport]

with open("data/main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)
    fun = input("f(x) = ")
    # just believe me, this will create the function here
    exec("def f(x): return " + fun)

    for elem in ([x / 100, f(x / 100)] for x in range(100000)): # pyright: ignore[reportUndefinedVariable, reportUnknownVariableType]
        the_writer.writerow(elem) # pyright: ignore[reportUnknownArgumentType]
