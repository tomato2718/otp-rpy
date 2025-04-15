# otp-rpy

OTP related function for python written in rust.

## Usage

```py
from time import time

from otp_rpy import generate_hotp, generate_totp, validate_hotp, validate_totp

SECRET = "TESTTESTTESTTEST"

# To create TOTP code
totp_code = generate_totp(SECRET, int(time()))
print(totp_code)

# To validate TOTP code
is_valid = validate_totp(totp_code, secret=SECRET, now=int(time()))
print(is_valid)

# To create HOTP code
hotp_code = generate_hotp(SECRET, 1)
print(hotp_code)

# To validate HOTP code
is_valid = validate_hotp(hotp_code, secret=SECRET, count=2, tolerance=2)
print(is_valid)
```
