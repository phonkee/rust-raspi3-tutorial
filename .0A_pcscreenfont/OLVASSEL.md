Oktatóanyag 0A - PC Screen Font
===============================

Képeket kirakni poénos, de mindenképpen szükség van többre, karakterek megjelenítésére is. Alapvetően
a fontok nem mások, mint képek minden karakterhez (glyphek). Ehhez az oktatóanyaghoz azt a PC Screen Font
formátumot választottam, amit a Linux Console csomag is használ.

Lfb.h, lfb.c
------------

`lfb_init()` beállítja a felbontást, mélységet, színcsatornákat és visszaadja a framebuffer címét.

`lfb_print(x,y,s)` megjelenít egy szöveget a képernyőn.

Font.psf
--------

A font fájl. Bármelyik használható a /usr/share/kbd/consolefonts mappából. Unicode táblákat nem támogatja.
A karakterek glypehnek való megfeleltetése ezen táblázat által (a meglévő egy-az-egyhez megfeleltetés helyett)
házi feladat, Rád van bízva. Ezt a fontot az eredeti IBM PC VGA ROM-jában található 8x16 fontkészletből generáltam,
és 127 glyphet tartalmaz.

Makefile
--------

Egy új object-et adtam hozzá, ami a psf-ből generálódik. Jó példa arra, hogyan kell bináris fájlt behúzni és
hivatkozni C forrásból. A következő parancsot használtam a cimke nevének kiderítésére:

```sh
$ aarch64-elf-readelf -s font.o
        ... kimenet törölve az átláthatóság miatt ...
     2: 0000000000000820     0 NOTYPE  GLOBAL DEFAULT    1 _binary_font_psf_end
     3: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT    1 _binary_font_psf_start
     4: 0000000000000820     0 NOTYPE  GLOBAL DEFAULT  ABS _binary_font_psf_size
```

Main
----

Nagyon egyszerű. Beállítjuk a felbontást és megjelenítjük a szöveget.
