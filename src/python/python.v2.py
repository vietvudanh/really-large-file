#!/usr/bin/env python3
import sys
import multiprocessing
from collections import defaultdict

file_path = sys.argv[1]

print(f"processing file:: {file_path}")

count = 0
fnames_dict = defaultdict(int)
date_dict = defaultdict(int)
names_index = []
with open(file_path, "r") as file:
    for line in file:
        count += 1

        # date: 201801180300185828
        _, _, _, _, date, _, _, name, *rest = line.split('|')
        date = date[:6]
        date_dict[date] += 1

        if count in (433, 43244):
            names_index.append(name)

        if ', ' in name:
            fname, lname, *rest = name.split(', ')
            fnames_dict[fname] += 1

max_val = -1
max_fname = None
for k, v in fnames_dict.items():
    if v > max_val:
        max_val = v
        max_fname = k

print(f"task 1:: {count}")
print(f"task 2:: {names_index}")
print("task 3")
print("\n".join(f"  {k}:{v}" for k, v in date_dict.items()))
print(f"task 4:: {max_fname}; {fnames_dict[max_fname]}")
