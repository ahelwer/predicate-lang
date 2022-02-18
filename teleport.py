import z3  # type: ignore

class Entities:
  USERNAME: z3.SeqRef = z3.String("username")
  RESOURCE_NAME: z3.SeqRef = z3.String("resource_name")
