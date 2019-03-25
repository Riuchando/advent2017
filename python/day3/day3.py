import numpy as np


def earlySpiralStop(num):
    x = 1
    while x**2 < num:
        x += 2
    return x


def part1(x):
    ss = earlySpiralStop(x)
    ring = ss**2 - (ss-2)**2
    minVal = np.min([np.abs(((ss-2)**2 + ring/8 + ring/4*y) - x)
                     for y in range(4)])
    return (ss-1)/2 + minVal


def part2(x, size):
    y = [[0 for i in range(size)] for j in range(size)]
    xindex = size/2
    yindex = size/2
    ringLen = 1
    spiralIter = 0
    index = 0
    direction = 0  # 0 = r , 1 = up , 2 = l , 3 = d
    while True:
        y[xindex][yindex] = y[xindex-1][yindex-1] + y[xindex][yindex-1] + y[xindex-1][yindex]+y[xindex +
                                                                                                1][yindex]+y[xindex][yindex+1]+y[xindex+1][yindex+1]+y[xindex-1][yindex+1]+y[xindex+1][yindex-1]
        if y[xindex][yindex] == 0:
            y[xindex][yindex] = 1
            index -= 1
        index += 1
        if index == ringLen:
            index = 0
            spiralIter += 1
            direction = (direction + 1) % 4
        if spiralIter == 2:
            spiralIter = 0
            ringLen += 1

        # print y[xindex][xindex],direction, xindex, yindex
        # print np.matrix(y)
        if(y[xindex][yindex] > x):
            return y[xindex][yindex]
        if direction == 0:
            xindex += 1
        elif direction == 1:
            yindex -= 1
        elif direction == 2:
            xindex -= 1
        elif direction == 3:
            yindex += 1


print(part1(312051))
print(part2(312051, 20))
