#!/usr/bin/env python3
import sys

def num_occ(g):
    return sum(1 if c == '#' else 0 for c in ''.join(g))

def num_neigh(g, x, y):
    n = 0
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            if dx == 0 and dy == 0:
                continue
            nx = dx + x
            ny = dy + y
            if 0 <= ny < len(g) and 0 <= nx < len(g[ny]) and g[ny][nx] == '#':
                n += 1
    return n

def next_cell(g, x, y):
    c = g[y][x]
    if c == '.':
        return c
    n = num_neigh(g, x, y)
    if c == 'L' and n == 0:
        return '#'
    if c == '#' and n >= 4:
        return 'L'
    return c

def next_ferry(g):
    return [
        ''.join(next_cell(g, x, y) for x in range(len(row)))
        for y, row in enumerate(g)
    ]


def fmt_grid(g):
    return '\n'.join(g)


grid = [line.strip().split() for line in open(sys.argv[1])]
n = 0

last = ''
while True:
    n += 1
    grid = next_ferry(grid)
    s = fmt_grid(grid)
    if last == s:
        break
    last = s
    print(f'\n{n} occupied: {num_occ(grid)}\n{s}')
    print('---')

print(f'n: {n}, num_occ: {num_occ(grid)}\n{fmt_grid(grid)}')
