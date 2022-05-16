

arcs = []
while True:
    line = input()
    if len(line) == 0:
        break
    tokens = line.split()
    i = int(tokens[0])
    j = int(tokens[1])
    arcs.append((i, j, 1.0))


print(arcs)





