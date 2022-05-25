# The Predicate Language Project

The Predicate language project uses Z3 as the foundation of a new access control system.
The objective of this project is to create an access control system with the following desirable attributes:
 1. Blocking or allowing access based on arbitrary constraints over user traits, resource labels, and environment variables
 2. Defining & checking global security invariants
 3. Querying & introspection of reasoning for allow/deny decisions
 4. Portable & expandable access control logic

Previously, the Z3 theorem prover from Microsoft Research was used [to analyze Teleport roles](https://goteleport.com/blog/z3-rbac/) and enable logical comparisons between them.
This project takes a step beyond that.
Teleport currently uses YAML files to [encode role logic](https://goteleport.com/docs/access-controls/reference/); in the original project, these files were compiled into logical expressions understood by Z3.
Here, Z3 expressions themselves are used to encode arbitrary access control logic and permit analysis.

How does one write a Z3 expression?
Z3 bindings exist for [a number of popular programming languages](https://github.com/Z3Prover/z3#z3-bindings), most notably Python.
The envisaged user experience is as follows:
 1. The Python Z3 bindings are used to create a logical expression over resource labels, user traits, and environment variables which, if evaluated to true, permits a user access to a resource
 2. Z3 is used to check that the logical expression (possibly in conjunction with others) maintains global access control invariants
 3. The logical expression is exported from Z3 into the [SMT-LIBv2](https://smtlib.github.io/jSMTLIB/SMTLIBTutorial.pdf) language, which encodes the logical expression as an easily-parsed S-expression
 4. When a user requests access to a resource, the S-expression is evaluated (currently, using the Lisp-like language [Racket](https://racket-lang.org/)) to determine whether the user should be allowed or blocked from accessing the resource

## Example Use

Consider a basic access control system that enumerates combinations of users and resources: specific users are granted access to specific resources.
This could be written in Python as follows:
```py
from teleport import Entities
import z3

expr =
  z3.Or(
    z3.And(
      Entities.USERNAME == z3.StringVal("jdoe"),
      Entities.RESOURCE_NAME == z3.StringVal("build-server")
    ),
    z3.And(
      Entities.USERNAME == z3.StringVal("admin"),
      Entities.RESOURCE_NAME == z3.StringVal("prod-server")
    )
  )
```
The `expr` variable points to a Z3 expression.
Z3 can analyze this expression in various ways, like comparing it to other expressions or checking to ensure it satisfies global security invariants.
The S-expression form can be generated with:
```py
print(expr.sexpr())
```
which in this case will print out:
```sexpr
(or (and (= username "jdoe") (= resource_name "build-server"))
    (and (= username "admin") (= resource_name "prod-server")))
```
It is easy to evaluate this expression in Racket; whenever a user requests access to a resource, the well-known variables `username` and `resource_name` are given concrete values and the expression will evaluate to either true or false.
If true, the user is permitted to access the resource.
These well-known variables (and various helper functions) are provided by the `teleport` python module.
This module contains the `Entities` class defining the variables, as used above.

## Roles

Roles have two components: their name, and a logical expression.
If a user possesses a role and the corresponding logical expression evaluates to true for a given access request, the user is permitted access.
This is represented using Z3's if-then-else construct, `z3.If`:
```py
z3.If(
  Entities.USER_GROUP_MEMBERSHIP(z3.StringVal(role_name)),
  role_expr,
  z3.BoolVal(False)
)
```
The entire set of roles is combined in a large disjunction with `z3.Or` so if one or more of the `z3.If` statements evaluates to true then the user will be provided access.
`Entities.USER_GROUP_MEMBERSHIP` is an arbitrary Z3 function from strings to bools that is given a concrete value (the set of groups/roles of which the user is a member) when evaluating whether the formula is true or false.

## Constraint Variables

As mentioned above, access control expressions can be constraints over user traits, resource labels, or environmental variables such as the current time.
These are expressed as follows:
```py

```
