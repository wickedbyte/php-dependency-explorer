# PHP Dependency Explorer

A command line application with a number of subcommands useful for exploring the vendor dependencies of a PHP project
using Composer. 

## Installation

### Dependencies

If you are not going to use the project Docker image, the following prerequisites are required to run the application.

#### Rust and the Cargo Package Manager
If you don't already have a Rust toolchain installed locally, see the 
[official documentation for the recommended method(s)](https://rust-lang.org/tools/install/) of getting up and running. 
Most likely, you'll want to install `rustup`:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### PHP >=8.4 and the Composer Package Manager

By default, the PHP Dependency Explorer looks for `php` and `composer` using your shell $PATH configuration.
If `composer --version` or `php --version` fails, you want to use a different set of executables or Composer PHAR
file, you can use the `-c` and `-p`  options to set the paths for composer.phar and php executables
when running commands.

### Docker

The recommended way to use the command line application is through the project's Docker image, which is based on the
official Docker Hub Composer image, and ensures that the required dependencies on PHP and Composer are met. It also 
allows exploring the dependencies on legacy projects not yet compatible with PHP 8.4 and/or using older
composer versions.

Build and run the image as the executable, mounting the host directory with the composer.json and composer.lock files
that you want to explore. Pass the desired subcommands and options.

```shell
 docker buildx build --tag=wickedbyte/php-dependency-explorer --target=php-dependency-explorer --pull --load -f Dockerfile .
 docker run --rm --tty --volume=./:/app wickedbyte/php-dependency-explorer list --help
```

### Local Installation

Clone, Compile, and Install:
```shell
    git clone https://github.com/wickedbyte/php-dependency-explorer
    cargo install --path php-dependency-explorer
```

Run the commands:
```shell
    php-dependency-explorer list --help
```

