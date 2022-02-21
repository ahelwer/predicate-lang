from dataclasses import dataclass
from teleport import Entities
import typing
import z3  # type: ignore


def allows_matrix() -> z3.BoolRef:
  return z3.Or(
    z3.And(
      Entities.USERNAME == z3.StringVal("ahelwer"),
      Entities.RESOURCE_NAME == z3.StringVal("rpi")
    ),
    z3.And(
      Entities.USERNAME == z3.StringVal("sasha"),
      Entities.RESOURCE_NAME == z3.StringVal("teleport")
    )
  )

def allow_resource_attribs() -> z3.BoolRef:
  return z3.And(
    Entities.resource_attribute_str("foo", "bar", "baz"),
    Entities.resource_attribute_str("baz", "foobar")
  )

def allow_user_attribs() -> z3.BoolRef:
  return z3.And(
    Entities.user_attribute_str("foo", "bar"),
    Entities.user_attribute_str("baz", "foobar")
  )

def allow_role_set() -> z3.BoolRef:
  return Entities.role_set({
    "access-public" : z3.And(
      Entities.resource_attribute_str("env", "public"),
      Entities.resource_attribute_any_str("owner")
    ),
    "access-private" : z3.And(
      Entities.resource_attribute_str("env", "private"),
      Entities.resource_attribute(z3.StringVal("owner"), Entities.USERNAME)
    )
  })

print("Matrix:")
print(allows_matrix().sexpr())
print()

print("Resource Attributes:")
print(allow_resource_attribs().sexpr())
print()

print("User Attributes:")
print(allow_user_attribs().sexpr())
print()

print("Role Set:")
print(allow_role_set().sexpr())
print()
