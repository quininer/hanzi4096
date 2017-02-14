# 汉字4096

This library encodes bytes into 12-bit 汉字 (although it actually uses 13-bits).

### Exmaple

```
assert_eq!(
	hanzi4096::encode("Hello 汉字!".as_bytes()),
	"贰娃迤交杀萝尻淳"
);

assert_eq!(
    hanzi4096::decode("桃之夭夭灼灼其华之子于归宜其室家").unwrap(),
    [51, 151, 3, 125, 208, 7, 84, 67, 53, 227, 115, 29, 57, 240, 3, 23, 144, 14, 253, 52, 62, 160, 38, 131]
);
```
