# 汉字4096

This library encodes bytes into 12-bit 汉字 (although it actually uses 13-bits).

### Exmaple

```rust
// encode
assert_eq!(
	hanzi4096::encode("Hello 汉字!".as_bytes()),
	"贰娃迤交杀萝尻淳荥"
);

// decode
assert_eq!(
    hanzi4096::decode("桃之夭夭灼灼其华之子于归宜其室家").unwrap(),
    [51, 151, 3, 125, 208, 7, 84, 67, 53, 227, 115, 29, 57, 240, 3, 23, 144, 14, 253, 52, 62, 160, 38, 131]
);

// decode, ignore invalid char
assert_eq!(
	hanzi4096::decode_ignore("
        南有乔木 不可休息
        汉有游女 不可求思
        汉之广矣 不可泳思
        江之永矣 不可方思
	"),
	vec![141, 85, 24, 195, 97, 5, 90, 48, 13, 201, 65, 123, 54, 81, 24, 205, 42, 4, 90, 48, 13, 177, 178, 93, 54, 145, 3, 52, 224, 57, 90, 48, 13, 232, 180, 93, 25, 146, 3, 67, 225, 57, 90, 48, 13, 162, 176, 93]
);
```
