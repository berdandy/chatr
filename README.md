# chatr

Transform GW2 Template Codes into something else.

Example:

> `chatr DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=` (note that the chat code has `[&` and `]` removed)

Output:

```
Deciphering DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=
<div
  data-armory-embed='specializations'
  data-armory-ids='6,38,29'
  data-armory-6-traits='525,1892,505'
  data-armory-38-traits='1930,2006,510'
  data-armory-29-traits='509,470,1854'
>
</div>
```

Suitable for pasting into a build webpage.

# TODO:

- [x] figure out how to make a REST api call from rust
- [x] figure out how to unpack base64 string to struct from Anet
- [x] translate struct into IDs
- [x] format IDs into armory-embeds output template
- [x] add support for traits and specializations
- [ ] add support for skills
- [ ] add support for ranger pets
- [ ] add support for revenant legendary stances
- [ ] auto trim `[&` and `]`

### Long shot

- [ ] add support for equipment templates
