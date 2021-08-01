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
playfair --ignore-char j decipher --key death "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV"
```

```
Jul 31 17:43:09.737  INFO playfair: result = "LA BO UL AY EL AD YW IL LX LE AD TO CI BO LA TE MP LE SO FG OL DX"
```

### Encode

```bash
playfair --ignore-char j encipher --key death "LABOULAYE LADY WILL LEAD TO CIBOLA TEMPLES OF GOLD"
```

```
Jul 31 17:43:09.737  INFO playfair: result = "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV"
```
