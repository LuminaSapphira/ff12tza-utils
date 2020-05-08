import json
import sys

data = json.load(sys.stdin)
groups = {}
file_zone = {}
for group in data["Treasure"]:
    groups[group["Name"]] = []
    for item in group["Item"]:
        file = str(item["File"]).split('.')[0]
        file_zone[file] = {'name': item["Name"], 'offset': int(item["Offset"]), 'quantity': int(item["Quantity"])}
        groups[group["Name"]].append(file.split('.')[0])

out_data = {'groups': groups, 'zones': file_zone}

print(json.dumps(out_data))
