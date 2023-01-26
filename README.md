#  base-x-fast

This is a python binding for the Rust fork of https://github.com/cryptocoinjs/base-x implemented in https://github.com/OrKoN/base-x-rs

WARNING: This module is NOT RFC3548 compliant, it cannot be used for base16 (hex), base32, or base64 encoding in a standards compliant manner.

# Usage

import base_x_fast

MY_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

base_x_fast.encode(MY_ALPHABET,[2,3,4,5])

> 'CAwQF'

base_x_fast.decode(MY_ALPHABET,"CAwQF")

> [2, 3, 4, 5]