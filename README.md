<h1 align="center">
  <a href="https://github.com/jihchi/dify"><img src="logo.png" alt="Dify Logo" /></a>
</h1>

> A pixel-by-pixel image diffs tool

[![Workflows - CI][workflows-ci-shield]][workflows-ci-url]

## Features

| Feature                                                  |     |
| -------------------------------------------------------- | --- |
| `.png`, `.jpg`, `.jpeg`, or `.bmp` format supported.     | ✅  |
| Compares different format, `.png` vs `.jpg` for example. | ✅  |
| Compares different dimensions.                           | ✅  |
| Anti-aliasing support                                    | ✅  |
| Supports macOS, Linux and Windows.                       | ✅  |

## Getting Started

### Installation

#### From binaries

Download the binaries for your platform from [release](https://github.com/jihchi/dify/releases) page.

## Usage

```
dify left.jpg right.jpg
```

## Benchmarks

> Test data is coming from [here](https://github.com/dmtrKovalenko/odiff/tree/main/images)

```
$ hyperfine \
  --warmup 1 \
  --ignore-failure \
  --export-markdown bench-dify.md \
  'dify tiger.jpg tiger-2.jpg -o tiger-diff.png' \
  'dify water-4k.png water-4k-2.png -o water-diff.png' \
  'dify www.cypress.io.png www.cypress.io-2.png -o www.cypress.io-diff.png'
```

| Command                                                                   |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :------------------------------------------------------------------------ | -----------: | -------: | -------: | -----------: |
| `dify tiger.jpg tiger-2.jpg -o tiger-diff.png`                            |   49.3 ± 0.8 |     48.2 |     52.3 |         1.00 |
| `dify water-4k.png water-4k-2.png -o water-diff.png`                      | 2142.6 ± 4.0 |   2137.5 |   2149.2 | 43.46 ± 0.75 |
| `dify www.cypress.io.png www.cypress.io-2.png -o www.cypress.io-diff.png` | 1618.9 ± 3.9 |   1613.4 |   1626.1 | 32.84 ± 0.57 |

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

- This project is inspired by [dmtrKovalenko/odiff](https://github.com/dmtrKovalenko/odiff) and [mapbox/pixelmatch](https://github.com/mapbox/pixelmatch)
- Same as the projects inspired from, implements ideas from the following papers:
  - [Measuring perceived color difference using YIQ NTSC transmission color space in mobile applications](http://www.progmat.uaem.mx:8080/artVol2Num2/Articulo3Vol2Num2.pdf) (2010, Yuriy Kotsarenko, Fernando Ramos)
  - [Anti-aliased pixel and intensity slope detector](https://www.researchgate.net/publication/234126755_Anti-aliased_Pixel_and_Intensity_Slope_Detector) (2009, Vytautas Vyšniauskas)

[workflows-ci-shield]: https://github.com/jihchi/dify/workflows/CI/badge.svg
[workflows-ci-url]: https://github.com/jihchi/dify/actions?query=workflow%3ACI
