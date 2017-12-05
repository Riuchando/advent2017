from collections import Counter
import numpy as np 
f = open("../../../day4.txt")


words =[s.strip().split(" ") for s in f.readlines()]

print words
def part1(pass_phrase):
	if len(pass_phrase) == len(set(pass_phrase)):
		return 1
	else:
		return 0

def part2(pass_phrase):
	counts =[dict(Counter(x)) for x in pass_phrase]
	uniqlist = list(np.unique(np.array(counts)))
	if len(counts) == len(uniqlist):
		return 1
	else:
		return 0

print sum([part1(pass_phrase) for pass_phrase in words])
print sum([part2(pass_phrase) for pass_phrase in words])


