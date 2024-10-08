import csv
import math  # pyright: ignore[reportUnusedImport]
import random

fun = input("part1: f(x) = ")
with open("data/pt1_main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)

    # just believe me, this will create the function here
    exec("def f(x): return " + fun)

    for elem in (  # pyright: ignore[reportUnknownVariableType]
        [
            x / 100,
            f(x / 100),  # pyright: ignore[reportUndefinedVariable]
        ]
        for x in range(100000)
    ):
        the_writer.writerow(elem)  # pyright: ignore[reportUnknownArgumentType]

fun = input("part2: f(x) = ")
with open("data/pt2_main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)

    # just believe me, this will create the function here
    exec("def f(x): return " + fun)

    vals = []
    for x in range(100000):
        xval = x / 100 + random.randrange(-5, 5) / 10 *x/100
        fval = f(x / 100) + random.randrange(-5, 5) / 10*f(x/100)  # pyright: ignore[reportUndefinedVariable]
        vals.append([
            xval,
            fval,
            int(f(xval) > fval),  # pyright: ignore[reportUndefinedVariable, reportUnknownArgumentType] # fmt: skip
        ])

    for elem in (vals):
        the_writer.writerow(elem)  # pyright: ignore[reportUnknownArgumentType]
