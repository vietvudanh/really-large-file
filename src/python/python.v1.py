#!/usr/bin/env python3
import sys
from collections import defaultdict
import operator

file_path = sys.argv[1]

print(f"processing file:: {file_path}")

count = 0
names_set = set()
fnames_dict = defaultdict(int)
names_arr = []
date_dict = defaultdict(int)
with open(file_path, "r") as file:
    for line in file:
        count += 1

        # date: 201801180300185828
        _, _, _, _, date, _, _, name, *rest = line.split('|')
        date = date[:6]
        date_dict[date] += 1

        if ', ' in name:
            fname, lname, *rest = name.split(', ')
            fnames_dict[fname] += 1
            if name not in names_set:
                names_set.add(name)
                names_arr.append(name)

max_val = -1
max_fname = None
for k, v in fnames_dict.items():
    if v > max_val:
        max_val = v
        max_fname = k

print(f"task 1:: {count}")
print(f"task 2:: {names_arr[432]}; {names_arr[43243]}")
print(f"task 3:: {date_dict}")
print(f"task 4:: {max_fname}; {fnames_dict[max_fname]}")
