"""Runs a small script to get the master node token"""

import sys

with open("/tmp/master-node.txt", "r") as node:
    token = node.readlines()[1]
    token = token.strip()

if token is not None:
    print(token)
else:
    print("Could not get master_node token")
    sys.exit(1)