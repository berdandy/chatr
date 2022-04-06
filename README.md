# chatr

Transform GW2 Template Codes into something else.

Example:

> `chatr DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=`

OR

> `chatr "[&DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=]"`

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
- [x] auto trim `[&` and `]`

### Medium term

- [x] make binary release (windows)
- [ ] make binary release (mac)
- [ ] build a gui for casual use

### Long term

- [ ] add support for equipment templates

# Dev Setup (on fresh machine)

## Windows 11 (winget):

- [Install windows build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Rustup will give more details, but c++ win10 sdk required.
- [Install Rustup](https://www.rust-lang.org/tools/install)
- And then...
```
winget install --id Git.Git
winget install --id GitHub.cli
gh repo clone accessibilitywars/chatr
cd chatr; cargo build
```

## mac

- [install brew](https://brew.sh/)
- And then...
```
brew install git gh
gh repo clone accessibilitywars/chatr
cd chatr; cargo build
```

## linux

- install git, gh, rust (TODO: specific commands)
- And then...
```
gh repo clone accessibilitywars/chatr
cd chatr; cargo build
```
