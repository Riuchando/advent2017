f = open("../../../day2.txt")

def divisible(x):
  for index,y in enumerate(x):
    for z in x[index+1:]:
      if y%z == 0:
	return y/z
      if z%y == 0:
        return z/y

def part1(x):
  checksum = 0
  for row in x:
    low = min([int(item) for item in row])
    high = max([int(item) for item in row])
    checksum = checksum + (high - low)
  return checksum

def part2(x):
  checksum = 0
  for row in x:
    introw =[int(i) for i in row]
    checksum = checksum + divisible(introw)
  return checksum

file =[row.strip().split("\t") for row in f.readlines()]

print(part1(file))
test=[[5, 9, 2, 8],
[9, 4, 7, 3],
[3, 8, 6, 5]]
#print sum([divisible(i) for i in test])
print(part2(file))
