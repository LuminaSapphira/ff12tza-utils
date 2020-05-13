# Treasure data files

The treasure data can be found within zone-specific data files. These
files contain a variety of data other than treasure so the
treasure-specific section must be located first. In the case of
treasure, exact offsets to each zone's treasure can be found in the data
folder - [`treasure_data.json`](../data/treasure_data.json). Note that due
to JSON restrictions, these offsets are in base 10 / decimal.

After seeking to the treasure data offset, the treasure format is an
array with the size specified in the JSON. Each element is a 24 byte
struct formatted as follows:

| Offset | Desc           | Type |
|:-------|:---------------|:-----|
| 0x00   | Treasure ID    | u32  |
| 0x04   | X Position     | u16  |
| 0x06   | Y Position     | u16  |
| 0x08   | Unknown        | u8   |
| 0x09   | Respawn Slot\* | u8   |
| 0x0a   | Spawn Chance   | u8   |
| 0x0b   | Gil Chance     | u8   |
| 0x0c   | 1st Item ID    | u16  |
| 0x0e   | 2nd Item ID    | u16  |
| 0x10   | (D) 1st Item   | u16  |
| 0x12   | (D) 2nd Item   | u16  |
| 0x14   | Gil Amount     | u16  |
| 0x16   | (D) Gil Amt    | u16  |

\* The Respawn Slot field should be `0xFF` if the chest should respawn.
Otherwise, it should be a byte `0x01` - `0xFE` indicating the
non-respawn slot. Chests may share the same non-respawn slot (i.e.
spells), and all will disappear if one is opened.

\*\* *(D)* indicates Diamond Armlet.

Item IDs can be found in [`item_data.json`](../data/item_data.json)
