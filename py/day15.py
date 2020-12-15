import sys

nums = [int(x) for x in sys.argv[1].split(',')]
rounds = int(sys.argv[2])

num_to_round = {}
last_spoken = 0

for i, num in enumerate(nums):
    if i > 0:
        num_to_round[last_spoken] = i - 1
    last_spoken = num

for i in range(len(nums), rounds):
    r = num_to_round.get(last_spoken)
    num_to_round[last_spoken] = i - 1
    if r == None:
        last_spoken = 0
    else:
        last_spoken = i - r - 1

print(f'After {rounds}, last_spoken={last_spoken}')
