from spdcalc import *

meta = get_all_crystal_meta()

print("All built-in crystal ids")
print("=========================")
for v in meta:
    print("-", v["id"])
