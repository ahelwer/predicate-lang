from dataclasses import dataclass
from teleport import Entities
import typing
import z3  # type: ignore


def allows() -> z3.BoolRef:
  return z3.And(
    Entities.USERNAME == z3.StringVal("ahelwer"),
    Entities.RESOURCE_NAME == z3.StringVal("rpi")
  )

print(allows().sexpr())
