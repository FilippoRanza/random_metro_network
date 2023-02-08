#! /usr/bin/python

import sys
import json

import numpy as np
from matplotlib import pyplot as plt
from matplotlib import colors


def load_file(file_path):
    with open(file_path) as file:
        data = json.load(file)

    points = np.array(data["points"])
    lines = data["lines"]
    return points, lines


def plot_points(points):
    xs = points[:, 0]
    ys = points[:, 1]
    plt.scatter(xs, ys, color="k")

def plot_line(x, y, color):
    plt.plot(x, y, color=color)

def get_line_coords(point_x, point_y, line):
    line = np.array(line)
    x = point_x[line]
    y = point_y[line]
    return x, y

def plot_lines(points, lines):
    point_x = points[:, 0]
    point_y = points[:, 1]
    color_iter = iter(colors.TABLEAU_COLORS.values())
    mapped = map(lambda x: get_line_coords(point_x, point_y, x), lines)
    for x, y in mapped:
        color = next(color_iter)
        plot_line(x, y, color)

def get_input_file():
    try:
        output = sys.argv[1]
    except:
        print(
            sys.argv[0],
            "must provide an input network file json from random_metro_netork",
        )
        exit(1)
    return output


def main():
    input_file = get_input_file()
    points, lines = load_file(input_file)
    plot_points(points)
    plot_lines(points, lines)
    plt.axis("off")
    plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    main()
