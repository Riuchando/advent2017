
f = open("../../../day1.txt")

def part1(x):
  y = x[1:]+x[0]
  print sum([ int(pair[0]) for pair in zip(x,y)  if pair[0]==pair[1]])

def part2(x):
  z = x[len(x)/2:] + x[:(len(x))/2]
  # print x,z
  print sum([ int(pair[0]) for pair in zip(x,z)  if pair[0]==pair[1]])


file =f.read().strip()
part1(file)
part2(file)

# part2("1212")
# part2("123425")
# part2("123123")
# part2("12131415")