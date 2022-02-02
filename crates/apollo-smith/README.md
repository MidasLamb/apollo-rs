## Limitations

- Syntactly correct, but not semanticly correct (example when creating a default value argument you can have something like `value: Int = "test"`)
- Arguments are not fully supported
- DirectiveDef is not supported
- Recursive object type not already supported (example : `myType { inner: myType }`)

Side notes:

Different branch for encodeer
Add a document struct to take Schema, Structs, Operations and so one.
