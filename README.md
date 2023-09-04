# chatr

Transform GW2 Template Codes into something else.

Example:

> `chatr DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=`

OR

> `chatr "[&DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=]"`

Output:

```
`[&DQMGKiYaHT4oAQAAkwEAAI4BAAAiAQAAiQEAAAAAAAAAAAAAAAAAAAAAAAA=]`

---

<div
  data-armory-embed='skills'
  data-armory-ids='5857,5927,5912,5836,5868'
>
</div>
<div
  data-armory-embed='specializations'
  data-armory-ids='6,38,29'
  data-armory-6-traits='525,1892,505'
  data-armory-38-traits='1930,2006,510'
  data-armory-29-traits='509,470,1854'
>
</div>
```

Suitable for copy-pasting into a build webpage, or using with vim `read!`

# Dev Setup (on fresh machine)

The following instructions are only required if you wish to build chatr from source. If you're just downloading the release, this is not required. Chatr just needs to be your PATH or current directory (for windows)

## Windows 10/11 (winget):

- Winget is available pre-installed on Windows 11.
- [Install windows build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Rustup will give more details, but c++ win10 sdk required.
- [Install Rustup](https://www.rust-lang.org/tools/install)
- And then...
```
winget install --id Git.Git
winget install --id GitHub.cli
gh repo clone berdandy/chatr
cd chatr; cargo build
```

_I like winget for a solid pre-installed package manager on Windows 11. See [here for more info on earlier versions](https://docs.microsoft.com/en-us/windows/package-manager/winget/). Or simply install Git & GitHub CLI [using a different method](https://github.com/cli/cli#windows)_

## mac

- [install brew](https://brew.sh/)
- And then...
```
brew install git gh
gh repo clone berdandy/chatr
cd chatr; cargo build
```

## linux

- install git, gh, rust (TODO: specific commands)
- And then...
```
gh repo clone berdandy/chatr
cd chatr; cargo build
```
