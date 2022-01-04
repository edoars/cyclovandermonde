#!/usr/bin/env sage

import csv
import sys

from sage.all import *
from sage.tensor.differential_form_element import d

THRESHOLD = 0.07


def read_table(input):
    data = []

    try:
        with open(input, newline="") as fp:
            reader = csv.reader(fp, delimiter="\t")
            header = next(reader)
            for row in reader:
                data.append((int(row[0]), float(row[1])))

    except IOError:
        print(f"Could not read file {input}")
        sys.exit(1)
    except (ValueError, IndexError):
        print(f"Invalid format in {input}")
        sys.exit(1)
    except:
        print(f"Unexpected error during table parsing")
        sys.exit(1)

    return header, data


def partition_data(data):
    # data = [(x, y) for (x, y) in data if x % 2 != 0]      # remove even input
    # data = [(x, y / euler_phi(x)) for (x, y) in data]     # normalize output
    max_n = max(len(factor(x)) for (x, y) in data)

    partitions = {
        f"${k}$": [(x, y) for (x, y) in data if len(factor(x)) == k]
        for k in range(1, max_n + 1)
    }

    return partitions


def main(argv):
    if len(argv) != 3:
        print(f"Usage: {argv[0]} [table] [out]")
        print("Output a graph of Tr(H_n) based on input table")
        sys.exit(1)

    header, data = read_table(argv[1])
    partitions = partition_data(data)

    plots = sum(
        list_plot_semilogy(
            partitions[key],
            hue=k / len(partitions),
            size=2,
            legend_label=key,
        )
        for k, key in enumerate(partitions)
    )

    plots.axes_labels([f"${header[0]}$", f"${header[1]}$"])
    plots.set_legend_options(title="Smallest prime", loc="upper left")

    plots.save(argv[2], dpi=300, title="$n$ squarefree, $2 < n < 10000$")


if __name__ == "__main__":
    main(sys.argv)
