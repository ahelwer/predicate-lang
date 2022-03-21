from teleport import Entities
import z3  # type: ignore


def allows_matrix() -> z3.BoolRef:
  """
  Implement a basic allow matrix with Z3 expressions. Here, each user and
  resource combination has to be explicitly enumerated if it is allowed.
  """
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
  """
  Implement access control predicated on resource attributes.
  """
  return z3.And(
    Entities.resource_attribute_str_eq("foo", "bar", "baz", "boo"),
    Entities.resource_attribute_str_eq("baz", "foobar")
  )

def allow_user_attribs() -> z3.BoolRef:
  """
  Implement access control predicated on user attributes.
  """
  return z3.And(
    Entities.user_attribute_str_eq("foo", "bar"),
    Entities.user_attribute_str_eq("baz", "foobar")
  )

def allow_role_set() -> z3.BoolRef:
  """
  Implement Attribute-Based Access Control similar to to that available
  in Teleport roles.
  """
  return Entities.role_set({
    "access-public" : z3.And(
      Entities.resource_attribute_str_eq("env", "public"),
      Entities.resource_attribute_str_eq_any("owner")
    ),
    "access-private" : z3.And(
      Entities.resource_attribute_str_eq("env", "private"),
      Entities.resource_attribute_eq(z3.StringVal("owner"), Entities.USERNAME)
    )
  })

def local_invariant() -> z3.BoolRef:
  """
  Defines a "local" invariant, to be checked on each authorization
  request. These are constraints placed on the concrete attributes
  of a given user or resource.
  """
  return z3.Not(
    z3.And(
      Entities.USER_GROUP_MEMBERSHIP(z3.StringVal("users")),
      Entities.USER_GROUP_MEMBERSHIP(z3.StringVal("admins"))
    )
  )

def global_invariant() -> z3.BoolRef:
  """
  Defines a "global" invariant, which can be checked against the
  set of abstract rules in the system. In this case, if the above
  roles allow access to a user and their username is *not* listed
  as the resource owner, then the resource must be in the public
  environment and the user must be in the access-public group.
  """
  rules = allow_role_set()
  return z3.Implies(
    z3.And(
      rules,
      z3.Not(
        Entities.resource_attribute_eq(
          z3.StringVal("owner"),
          Entities.USERNAME
        )
      )
    ),
    z3.And(
      Entities.resource_attribute_str_eq("env", "public"),
      Entities.USER_GROUP_MEMBERSHIP(z3.StringVal("access-public"))
    )
  )

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

print("Local Invariant:")
print(local_invariant().sexpr())
print()

print("Global Invariant:")
inv = global_invariant()
print(inv.sexpr())
solver = z3.Solver()
solver.add(z3.Not(inv))
result = solver.check()
if z3.unsat == result:
  print("Global invariant holds")
else:
  print("Global invariant does not hold; counterexample:")
  print(solver.model())
print()
