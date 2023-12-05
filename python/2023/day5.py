# part 1
seeds, *blocks = open('input/day5.txt').read().split("\n\n")
seeds = list(map(int, seeds.split(":")[1].split()))

for block in blocks:
    ranges = []
    for line in block.splitlines()[1:]:
        ranges.append(list(map(int, line.split())))
    new = []
    for x in seeds:
        for a, b, c in ranges:
            if b <= x < b + c:
                new.append(x - b + a)
                break
        else:
            new.append(x)
    seeds = new
    
# print(min(seeds))


# part 2
seed_ranges, *blocks = open('input/day5.txt').read().split("\n\n")
seed_ranges = list(map(int, seed_ranges.split(":")[1].split()))

seeds = []

for i in range(0, len(seed_ranges), 2):
    seeds.append((seed_ranges[i], seed_ranges[i] + seed_ranges[i + 1]))

for block in blocks:
    ranges = []
    for line in block.splitlines()[1:]:
        ranges.append(list(map(int, line.split())))
    new = []
    while len(seeds) > 0:
        s, e = seeds.pop()
        for a, b, c in ranges:
            os = max(s, b)
            oe = min(e, b + c)
            if os < oe:
                new.append((os - b + a, oe - b + a))
                if os > s:
                    seeds.append((s, os))
                if e > oe:
                    seeds.append((oe, e))
                break
        else:
            new.append((s, e))    
            
    seeds = new
    
print(sorted(seeds))
print(min(seeds)[0])
        
