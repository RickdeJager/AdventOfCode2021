f = 0
d = 0
with open("input.txt", "r") as inp_fd:
    for line in inp_fd.readlines():
        di, am = line.strip().split()
        am = int(am)
        if di == "forward":
            f += am
        if di == "up":
            d -= am
        if di == "down":
            d += am
    print(d*f)
