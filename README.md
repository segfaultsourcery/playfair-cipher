# The Playfair Cipher

This implementation tries to follow the rules laid out in this [Wikipedia article][wiki].

[wiki]: https://en.wikipedia.org/wiki/Playfair_cipher

To test it, I used it to decode the cipher from National Treasure 2 (yes, I watched it recently):

```
ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV
```

When deciphered with the key `death`, you get the following result:

```
LA BO UL AY EL AD YW IL LX LE AD TO CI BO LA TE MP LE SO FG OL DX
```

Rearranged for readability, and without X:

```
LABOULAYE LADY WILL LEAD TO CIBOLA TEMPLES OF GOLD
```

I'd say it works!


## Try it yourself:

### Decode

```bash
playfair --ignore-char j decipher --key death \
    "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV"
```

```
LA BO UL AY EL AD YW IL LX LE AD TO CI BO LA TE MP LE SO FG OL DX
```

### Encode

```bash
playfair --ignore-char j encipher --key death \
    "LABOULAYE LADY WILL LEAD TO CIBOLA TEMPLES OF GOLD"
```

```
ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV
```

### Trouble?

Use the `-d` or `--debug` flag.

Here's a command using the `--debug` flag, and the output.

```bash
playfair --debug --ignore-char j decipher --key death \
    "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV"
```

```log
Aug 01 10:43:10.116 DEBUG playfair: Opt {
    debug: true,
    ignore_char: 'j',
    method: Decipher {
        key: "death",
        ciphertext: "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV",
    },
}
Aug 01 10:43:10.116 DEBUG playfair: Playfair key:
| d e a t h |
| b c f g i |
| k l m n o |
| p q r s u |
| v w x y z |

Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('m', 'e') -> ('l', 'a')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('i', 'k') -> ('b', 'o')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('q', 'o') -> ('u', 'l')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('t', 'x') -> ('a', 'y')
Aug 01 10:43:10.116 DEBUG playfair: VerticalLine :: ('c', 'q') -> ('e', 'l')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('t', 'e') -> ('a', 'd')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('z', 'x') -> ('y', 'w')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('c', 'o') -> ('i', 'l')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('m', 'w') -> ('l', 'x')
Aug 01 10:43:10.116 DEBUG playfair: VerticalLine :: ('q', 'c') -> ('l', 'e')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('t', 'e') -> ('a', 'd')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('h', 'n') -> ('t', 'o')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('f', 'b') -> ('c', 'i')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('i', 'k') -> ('b', 'o')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('m', 'e') -> ('l', 'a')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('h', 'a') -> ('t', 'e')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('k', 'r') -> ('m', 'p')
Aug 01 10:43:10.116 DEBUG playfair: VerticalLine :: ('q', 'c') -> ('l', 'e')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('u', 'n') -> ('s', 'o')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('g', 'i') -> ('f', 'g')
Aug 01 10:43:10.116 DEBUG playfair: HorizontalLine :: ('k', 'm') -> ('o', 'l')
Aug 01 10:43:10.116 DEBUG playfair: Rectangle :: ('a', 'v') -> ('d', 'x')
LA BO UL AY EL AD YW IL LX LE AD TO CI BO LA TE MP LE SO FG OL DX
```