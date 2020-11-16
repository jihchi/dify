[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<br />
<p align="center">
  <a href="https://github.com/jihchi/dify">
    <img src="logo.png" alt="Logo" width="173" height="118">
  </a>
  <h3 align="center">Dify</h3>
  <p align="center">
    Am image visual difference tool
  </p>
</p>

## Getting Started

### Installation

1. Clone the repo

```sh
git clone https://github.com/jihchi/dify.git
```

2. Print the help

```sh
cargo run -- --help
```

## Usage

```
dify --left base.jpg --right comparing.jpg
```

## Benchmarks

```
$ hyperfine -i 'dify -l water-4k.png -r water-4k-2.png' 'dify -l www.cypress.io.png -r www.cypress.io-1.png'

Benchmark #1: dify -l water-4k.png -r water-4k-2.png
  Time (mean ± σ):      2.836 s ±  0.014 s    [User: 2.677 s, System: 0.151 s]
  Range (min … max):    2.820 s …  2.867 s    10 runs

  Warning: Ignoring non-zero exit code.

Benchmark #2: dify -l www.cypress.io.png -r www.cypress.io-1.png
  Time (mean ± σ):      1.974 s ±  0.083 s    [User: 1.819 s, System: 0.145 s]
  Range (min … max):    1.907 s …  2.169 s    10 runs

  Warning: Ignoring non-zero exit code.

Summary
  'dify -l www.cypress.io.png -r www.cypress.io-1.png' ran
    1.44 ± 0.06 times faster than 'dify -l water-4k.png -r water-4k-2.png'
```

Ran on MacBook Pro (13-inch, 2019, Two Thunderbolt 3 ports), macOS Catalina 10.15.7.

## Roadmap

See the [open issues](https://github.com/jihchi/dify/issues) for a list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

- This project is inspired by [dmtrKovalenko/odiff](https://github.com/dmtrKovalenko/odiff)

[contributors-shield]: https://img.shields.io/github/contributors/jihchi/dify.svg?style=flat-square
[contributors-url]: https://github.com/jihchi/dify/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jihchi/dify.svg?style=flat-square
[forks-url]: https://github.com/jihchi/dify/network/members
[stars-shield]: https://img.shields.io/github/stars/jihchi/dify.svg?style=flat-square
[stars-url]: https://github.com/jihchi/dify/stargazers
[issues-shield]: https://img.shields.io/github/issues/jihchi/dify.svg?style=flat-square
[issues-url]: https://github.com/jihchi/dify/issues
[license-shield]: https://img.shields.io/github/license/jihchi/dify.svg?style=flat-square
[license-url]: https://github.com/jihchi/dify/blob/master/LICENSE.md
