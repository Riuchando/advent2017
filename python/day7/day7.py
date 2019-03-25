rows = open("../../../day7.txt")
#parsed = [row.split("->") for row in rows ]


def parseRow(row, dictionary):
    key, value = row.split("->")
    item, num = key.split(" ")[0], int(key.split(" ")[1].replace("(|)", ""))
    roots = value.split(", ")


# z = {}
# for x in rows.readlines():
#     z = parseRow(x, z)
