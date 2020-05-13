# License Board Files

The License Board files can be found at
`ps2data/image/ff12/test_battle/in/binaryfile` inside the VBF. The
boards are named board_n.bin where n is a number 1-12 corresponding to
the job.

| File         | Job             |
|:-------------|:----------------|
| board_1.bin  | White Mage      |
| board_2.bin  | Uhlan           |
| board_3.bin  | Machinist       |
| board_4.bin  | Red Battlemage  |
| board_5.bin  | Knight          |
| board_6.bin  | Monk            |
| board_7.bin  | Time Battlemage |
| board_8.bin  | Foebreaker      |
| board_8.bin  | Archer          |
| board_10.bin | Black Mage      |
| board_11.bin | Bushi           |
| board_12.bin | Shikari         |

# Data Structure

The board file contains an 8-byte long magic header:

`0x6c, 0x69, 0x63, 0x64, 0x18, 0x00, 0x18, 0x00`

Immediately following that is the license board data. It is an array of
576 u16/ushort values (or a 2D array, 24*24 matrix), each a license ID.
The license ID reference can be found in
[`license_ids.tsv`](license_ids.tsv). Each ID goes left-to-right across
the board, then top-to-bottom. Any empty cell should be filled with
`0xFFFF`.
