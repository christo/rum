#!/usr/bin/env python

# strip ascii prelude from omz decrypt output dump

import sys

end_of_header = "UM program follows colon:"
input_data = sys.stdin.buffer.read()

try:
    pos = input_data.find(end_of_header.encode("ascii"))
    if pos == -1:
        raise ValueError("Prelude not found")
    sys.stdout.buffer.write(input_data[pos + len(end_of_header):])
except ValueError as e:
    print(e, file=sys.stderr)
    sys.exit(1)
except BrokenPipeError:
    sys.exit(0)

