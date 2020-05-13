# Magick Metadata

The segment of data within the `battle_pack.bin` contains the sort
order, gil cost, icon, and name of all Magicks.

### How to find
The battle pack file's size is variable, so it is necessary to find a
signature within the binary data to locate the magick metadata. The
signature is as follows:

`0x51, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00`

After locating this signature, the array begins 28(0x1c) bytes from the
**start** of the signature.

### Struct Format

The format is an array of structs, each with format as follows:

| Offset | Desc       | Type |
|:-------|:-----------|:-----|
| 0x0    | Gil Value  | u16  |
| 0x2    | Unknown    | u8   |
| 0x3    | Icon ID    | u8   |
| 0x4    | Name ID    | u16  |
| 0x6    | Sort Index | u8   |
| 0x7-8  | Unknown    | u16? |


### Array Items

Each item in this array is ordered according to it's internal ID.
Mappings between ID and vanilla names can be found in
[base_order.txt](../../src/magick_order/base_order.txt)
