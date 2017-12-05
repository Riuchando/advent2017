f = open("../../../day5.txt")


instructions = [int(x.strip()) for x in  f.readlines()]

def part1(inst):
	index = 0
	steps = 0
	while True:
		inst[index] += 1
		#print inst[index], index
		index += inst[index] - 1
		#print index
		steps += 1
		if index >= len(inst):
			return steps


def part2(inst):
        index = 0
        steps = 0
        while True:
		if inst[index] >= 3:
			inst[index] -=1
			index += inst[index]+ 1
		else:
                	inst[index] += 1
                	index += inst[index] - 1
                steps += 1
                if index >= len(inst):
                        return steps 


#arrays are mutable outside of context /shrug
print part1([0,3,0,1,-3])
print part2([0,3,0,1,-3])
#print part1(instructions)
print part2(instructions)
