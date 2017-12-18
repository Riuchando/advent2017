
def isUneven(arr):
	first =arr[0]
	for rest in arr:
		if rest != arr:
			return false
	return true


def maxIndex(arr):
	idx,ret = 0,0
	for index,item in enumerate(arr):
		if item > ret:
			idx = index
			ret = item
	return idx,ret


def redistribute(arr):
	ret = arr[:]
	index,item = maxIndex(arr)
	redistSize = item / (len(arr)-1)
	if redistSize > 0:
		#print index, item, redistSize, (item % (len(arr) -1)), len(arr)
		for idx, it in enumerate(arr):
			if index != idx:
				ret[idx] = it + redistSize
			else:
				ret[idx] = item % (len(arr) -1)
	else:
		#print index, item
		ret[index] =0
		while item > 0:
			index = (index + 1) % len(arr)
			ret[index] = ret[index] + 1
			item = item - 1
			
	return ret

def part1(x):
	y = redistribute(x)
	z = []
	z.append(x)
	while not (y in z):
		#print y
		z.append(y)
		y = redistribute(y)
	return len(z)

def part2(x):
	y = redistribute(x)
        z = []
        z.append(x)
        while not (y in z):
                #print y
                z.append(y)
                y = redistribute(y)
        return len(z) - z.index(y)


print part1([0,2,7,0])
print part2([0,2,7,0])
print part1([int(x) for x in  open("../../../day6.txt").read().strip().split("\t")])
print part2([int(x) for x in  open("../../../day6.txt").read().strip().split("\t")])
