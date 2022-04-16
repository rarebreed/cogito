"""Runs a small script to get the main node token"""

import sys

with open("/tmp/main-node.txt", "r") as node:
    token = node.readlines()[1]
    token = token.strip() + " --worker"

if token is not None:
    print(token)
else:
    print("Could not get main_node token")
    sys.exit(1)