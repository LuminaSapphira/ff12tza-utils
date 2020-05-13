# ff12tza-utils
Utilities for modifying and reading the data of Final Fantasy XII: The Zodiac Age (PC)


This is a command line utility for executing some of the more basic tasks associated with modding FFXII: TZA.
At the moment there is a utility for reordering the ingame magic list, and for generating treasure/lists and maps.


### Treasure Maps
Generating the lists is pretty much done, however the maps are still in their infancy. I'm still analyzing just how
correct the extracted data is, but the current version outputs mostly correct maps in SVG format. There is no map background,
so it's just the positions and index of treasure relative to each other.

---

### Documentation
I'm working on some documentation for the various file formats found within the data files. It can be found in the
`docs` folder. I hope this will provide some fledgling modder some use as the vast majority of tools currently available
are closed source :( .
