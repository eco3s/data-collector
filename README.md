# Data Collector

## Purpose

This tool does two things:

1. download all invasive species data list from [Information of Korean Alien Species](https://kias.nie.re.kr/home/main/main.do) website.

2. parse them into a fully-structured, granularly filtered format that fits well for both JSON and SQL.

## How to use

First, clone this repository and `cd` into it.

```sh
$ git clone https://github.com/eco3s/data-collector
$ cd data-collector
```

make sure that you have `cargo` and `rustc` (follow this [guide](#rustup)) then run the following command to build project.

```sh
$ cargo build --release
```

Or you can run it directly

```sh
$ cargo run --release
```

above command will automatically build the project if necessary, then run compiled downloader, which will download all raw json species data in `downloads` folder.

## Contributing

### Before Commit

1. install [pre-commit](https://pre-commit.com) on your machine.

    If not, try following

    ```sh
    $ pip install pre-commit
    ```

    To ensure `pre-commit` is successfully installed, try following

    ```sh
    $ pre-commit -V
    ```

    Which will show installed version.

    Then, setup rest things by below instructions

    ```sh
    $ pre-commit autoupdate
    $ pre-commit install
    ```

    Then try running the `pre-commit` hooks by following command

    ```sh
    $ pre-commit run -a
    ```

    if these hooks run successfully, you are able to commit. But the above will show an error, keep reading.

2. <a id="rustup">make sure that you have installed [rustup](https://rustup.rs) and configured channel as nightly.</a>

    we must need nightly `rustfmt` toolchain to support unstable format options listed in [rustfmt.toml](./rustfmt.toml).

    if not, try following

    ```sh
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    $ rustup default nightly
    ```

    To ensure `rustfmt` is successfully installed, try following

    ```sh
    rustfmt -V
    ```

    which will show that installed version is nightly.

## License

[![GitHub](https://img.shields.io/github/license/eco3s/data-collector?color=2e8555&style=for-the-badge)](https://github.com/eco3s/data-collector/blob/main/LICENSE)
