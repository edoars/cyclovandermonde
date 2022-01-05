#!/usr/bin/env sage

import sys

from sage.all import *


def main(argv):
    if len(argv) != 2:
        print(f"Usage: {argv[0]} [n]")
        print("Output a table of squarefree integers from 2 to n")
        sys.exit(1)

    try:
        n = int(argv[1])
    except ValueError:
        print("Error: [n] must be an integer")
        sys.exit(1)

    print("\n".join(str(x) for x in range(2, n + 1) if is_squarefree(x)))


if __name__ == "__main__":
    main(sys.argv)
