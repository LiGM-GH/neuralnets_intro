import csv
import math
import random

SAMPLES_COUNT = 100
X_AXIS_LEN = 1000

fun = input("part1: f(x) = ")
with open("data/pt1_main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)

    # just believe me, this will create the function here
    exec("def f(x): return " + fun)

    for elem in (
        [
            x * X_AXIS_LEN / SAMPLES_COUNT,
            f(x * X_AXIS_LEN / SAMPLES_COUNT),
        ]
        for x in range(SAMPLES_COUNT)
    ):
        the_writer.writerow(elem)

fun = input("part2: f(x) = ")
with open("data/pt2_main.csv", "w") as ffile:
    the_writer = csv.writer(ffile)

    # just believe me, this will create the function here
    exec("def f(x): return " + fun)

    vals = []
    for x in range(SAMPLES_COUNT):
        xval = (
            x * X_AXIS_LEN / SAMPLES_COUNT
            + random.randrange(-5, 5) / 10 * x * X_AXIS_LEN / SAMPLES_COUNT
        )
        fval = f(x * X_AXIS_LEN / SAMPLES_COUNT) + random.randrange(-5, 5) / 10 * f(
            x * X_AXIS_LEN / SAMPLES_COUNT
        )
        vals.append(
            [
                xval,
                fval,
                int(f(xval) > fval),
            ]
        )

    for elem in vals:
        the_writer.writerow(elem)
