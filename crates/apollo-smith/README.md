## Limitations

- Syntactly correct, but not semanticly correct (example when creating a default value argument you can have something like `value: Int = "test"`)
- Arguments are not fully supported
- Recursive object type not already supported (example : `myType { inner: myType }`)
- Check missing part of Document type

Side notes:

Different branch for encodeer
Add a document struct to take Schema, Structs, Operations and so one.

```
 query {
  R0 @x @wx
  ... @x {
  AAEVa6Ql0(J: 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005)
  AA1
}

}
query {
  x0
  R1(R: 281663955271680)
}
fragment B on Int {
  Z0
  R1 @x @PPPPPPD
}

extend schema @N @x {
  query: Int
  mutation: Int
  subscription: Int
}
scalar R @p @wW
type t {
  "J"
  J1: V
  t0(V0: V, R1: Float): [[[V]]]
}
interface wW {
  "g"
  tIQ1: V @V(j: 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005453612399186) @N
  E0: V
}
union wW2 = t
enum V {
  ZgAFZGz2
  l1
  "wo"
  N0
}
union wW2 = t
directive @h on UNION
directive @YNNNNNNNNNNKEA on MUTATION

```

Parser error ============= ERROR@26:27 "expected a Name" @
Parser error ============= ERROR@29:30 "expected R_CURLY, got {" {
