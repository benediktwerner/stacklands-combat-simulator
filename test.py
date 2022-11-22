#!/usr/bin/env python3

speeds = [3.5, 2.5, 1.5, 1, 0.75, 0.5]
chances = [0.5, 0.6, 0.7, 0.8, 0.85, 0.95]
dmgs = list(range(1, 7))


def dps(dmg, chance, speed):
    return int(dmgs[dmg] * chances[chance] * 1 / speeds[speed] * 100)


def best(dmg, chance, speed):
    a = dps(dmg + 1, chance, speed) if dmg < 5 else 0
    b = dps(dmg, chance + 1, speed) if chance < 5 else 0
    c = dps(dmg, chance, speed + 1) if speed < 5 else 0
    m = max(a, b, c)
    return ["", "d"][a == m] + ["", "c"][b == m] + ["", "s"][c == m]


for dmg in range(6):
    print("dmg", dmgs[dmg])
    for chance in range(6):
        print(
            chances[chance],
            "\t",
            "\t".join(
                best(dmg, chance, speed) + "|" + str(dps(dmg, chance, speed))
                for speed in range(6)
            ),
        )
    print()
