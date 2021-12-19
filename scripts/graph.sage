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
            reader = csv.reader(fp, delimiter=" ")
            next(reader)
            for row in reader:
                data.append((int(row[0]), int(row[1])))

    except IOError:
        print(f"Could not read file {input}")
        sys.exit(1)
    except (ValueError, IndexError):
        print(f"Invalid format in {input}")
        sys.exit(1)
    except:
        print(f"Unexpected error during table parsing")
        sys.exit(1)

    return data


def partition_data(data):
    max_n = max(x for (x, y) in data)
    max_p = floor(sqrt(max_n))
    len_threshold = floor(len(data) * THRESHOLD)
    p_list = [
        p
        for p in primes(3, max_p)
        if len([c for c in data if c[0] % p == 0]) > len_threshold
    ]
    print(p_list)

    data_primes = [
        c for c in data if is_prime(c[0]) or (c[0] % 2 == 0 and is_prime(c[0] // 2))
    ]
    partitions = {"$p \wedge 2 \cdot p$": data_primes}
    for p in p_list:
        tmp = [
            c for c in data if c[0] % p == 0 and not c in sum(partitions.values(), [])
        ]
        partitions[f"${p}\cdot p$"] = tmp

    data_res = [c for c in data if not c in sum(partitions.values(), [])]
    partitions["others"] = data_res

    return partitions


def main(argv):
    if len(argv) != 3:
        print(f"Usage: {argv[0]} [table] [out]")
        print("Output a graph of Tr(H_n) based on input table")
        sys.exit(1)

    data = read_table(argv[1])
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

    plots.axes_labels(["$n$", "Tr$(H_n)$"])
    plots.set_legend_options(loc="lower right")

    plots.save(argv[2], dpi=300)


if __name__ == "__main__":
    main(sys.argv)
