# Plan for implementing L-system plant generation
* Implement L-system processing
    * use production rules to generate a string from a given axiom
    * parametrized number of iterations
    * allow user-defined axioms and sets of production rules
    * https://dn790008.ca.archive.org/0/items/the-algorithmic-beauty-of-plants/The%20Algorithmic%20Beauty%20of%20Plants.pdf
    * alphabet `F, +, -,  &, ^, \, /, [, ]`
* Implement turtle graphics interpreter
  * go from L-system string to a vector of transformation matrices for the base model
  * interpret symbols as turtle commands
* Load base plant model from OBJ file
  * simple - draw n times with different transform matrix
  * optimization use instanced rendering