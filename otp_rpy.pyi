__all__ = ["generate_hotp", "validate_hotp", "generate_totp", "validate_totp",]

def generate_hotp(secret: str, count: int) -> str: ...
def validate_hotp(
    code: str, *, secret: str, count: int, tolerance: int | None = None
) -> bool: ...
def generate_totp(secret: str, now: int) -> str: ...
def validate_totp(
    code: str, *, secret: str, now: int, tolerance: int | None = None
) -> bool: ...
