import json
import sys


def remove_spacer(inline):
    if inline[0] == '=':
        return False
    else:
        return True


item_list = {'ids': {}}

for line in filter(remove_spacer, [i.strip('",\n') for i in sys.stdin.readlines()]):
    spls = line.split(':')
    item_id = int(spls[0], 16)
    name = str(spls[1])
    item_list['ids'][item_id] = name

print(json.dumps(item_list))
