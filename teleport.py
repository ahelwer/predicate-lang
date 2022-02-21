import z3  # type: ignore

class Entities:
  USERNAME: z3.SeqRef = z3.String("username")
  RESOURCE_NAME: z3.SeqRef = z3.String("resource_name")

  _RESOURCE_ATTRIBUTE_KEYS: z3.FuncDeclRef = z3.Function("resource_attribute_keys", z3.StringSort(), z3.BoolSort())
  _RESOURCE_ATTRIBUTE_VALUES: z3.FuncDeclRef = z3.Function("resource_attribute_values", z3.StringSort(), z3.StringSort())
  def resource_attribute(key: z3.SeqRef, *values: list[z3.SeqRef]) -> z3.BoolRef:
    return z3.And(
      Entities._RESOURCE_ATTRIBUTE_KEYS(key),
      z3.Or([
        Entities._RESOURCE_ATTRIBUTE_VALUES(key) == value for value in values
      ])
    )

  def resource_attribute_str(key: str, *values: list[str]) -> z3.BoolRef:
    return Entities.resource_attribute(
      z3.StringVal(key),
      *[z3.StringVal(value) for value in values]
    )
  
  def resource_attribute_any(key: z3.SeqRef) -> z3.BoolRef:
    return Entities._RESOURCE_ATTRIBUTE_KEYS(key)

  def resource_attribute_any_str(key: str) -> z3.BoolRef:
    return Entities.resource_attribute_any(z3.StringVal(key))

  _USER_ATTRIBUTE_KEYS: z3.FuncDeclRef = z3.Function("user_attribute_keys", z3.StringSort(), z3.BoolSort())
  _USER_ATTRIBUTE_VALUES: z3.FuncDeclRef = z3.Function("user_attribute_values", z3.StringSort(), z3.StringSort(), z3.BoolSort())
  def user_attribute(key: z3.SeqRef, *values: list[z3.SeqRef]) -> z3.BoolRef:
    return z3.And(
      Entities._USER_ATTRIBUTE_KEYS(key),
      z3.Or([
        Entities._USER_ATTRIBUTE_VALUES(key, value) for value in values
      ])
    )
  
  def user_attribute_str(key: str, *values: list[str]) -> z3.BoolRef:
    return Entities.user_attribute(
      z3.StringVal(key),
      *[z3.StringVal(value) for value in values]
    )
  
  _USER_ROLE_MEMBERSHIP: z3.FuncDeclRef = z3.Function("user_group_membership", z3.StringSort(), z3.BoolSort())
  def role_set(roles: dict[str, z3.BoolRef]) -> z3.BoolRef:
    return z3.Or([
      z3.If(
        Entities._USER_ROLE_MEMBERSHIP(z3.StringVal(role_name)),
        role_expr,
        z3.BoolVal(False)
      ) for role_name, role_expr in roles.items()
    ])
