# VSAG Python Binding

[![](https://github.com/jiacai2050/vsag-py/actions/workflows/CI.yml/badge.svg)](https://github.com/jiacai2050/vsag-py/actions/workflows/CI.yml)
[![](https://img.shields.io/pypi/v/vsag.svg)](https://pypi.org/project/vsag)

[VSAG](https://github.com/alipay/vsag) is a vector indexing library used for similarity search.

## Installation

```bash
pip install vsag
```

Built wheels are expected to be compatible with distros using glibc 2.28 or later, [including](https://github.com/pypa/manylinux?tab=readme-ov-file#manylinux_2_28-almalinux-8-based):

- Debian 10+
- Ubuntu 18.10+
- Fedora 29+
- CentOS/RHEL 8+

> [!IMPORTANT]
> Currently we're only testing VSAG on x86-64 Linux.

## Examples

See [hnsw.py](examples/hnsw.py)

## Development

```
python -m venv .env
source .env/bin/activate
pip install maturin
pip install maturin[patchelf]
```

Useful maturin commands:
```
  build        Build the crate into python packages
  publish      Build and publish the crate as python packages to pypi
  develop      Install the crate as module in the current virtualenv
```
