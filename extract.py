#!/usr/bin/env python3

import os, re, json

outs = []

PREFIX = "Misc_"
PREFIX = "Enemy_"
IMAGE_MAP = {
    "Swordsman": "Swordman",
    "Fisher": "Fisherman",
    "MommaCrab": "MammaCrab",
    "SmallSlime": "Small-Slime",
    "Slime": "Big-Slime",
    "Rat": "Small-Rat",
    "GiantRat": "Big-Rat",
    "Pirate": "AngryPirate",
}


def niceify(name: str) -> str:
    out = name[0]
    for c in name[1:]:
        if c.isupper():
            out += " "
        out += c
    return out


for fname in os.listdir("data"):
    if not fname.startswith(PREFIX):
        continue

    with open("data/" + fname) as f:
        data = f.read()

    name = fname[len(PREFIX) : -len(".prefab")]
    hp = int(re.findall(r"HealthPoints: (\d+)", data)[0])
    speed = float(re.findall(r"AttackTime: ([\d.]+)", data)[0])
    chance = float(re.findall(r"HitChance: (\d+)", data)[0])
    mind = float(re.findall(r"MinDamage: (\d+)", data)[0])
    maxd = float(re.findall(r"MaxDamage: (\d+)", data)[0])

    stun = re.findall("Chance: (\d+)\n    HitType: 2", data)
    image = IMAGE_MAP.get(name, name)
    img_path = f"/mnt/g/extracted/stacklands/Stacklands/ExportedProject/Assets/Texture2D/{image}.png"
    target_path = f"svelte/public/images/{image}.png"
    if os.path.isfile(target_path):
        pass
    elif os.path.isfile(img_path):
        os.system(f"cp {img_path} {target_path}")
    else:
        print("Missing image:", name)

    outs.append(
        {
            "name": niceify(name),
            "hp": hp,
            "attack_speed": int(speed * 10),
            "hit_chance": chance / 100,
            "min_damage": mind,
            "max_damage": maxd,
            "image_name": image,
            "stun_chance": int(stun[0]) / 100 if stun else 0,
        }
    )

with open("data/dump.json", "w") as outf:
    json.dump(outs, outf)
