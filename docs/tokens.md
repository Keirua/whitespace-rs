{- Input to the whitespace VM.
   For convenience, three input characters 
       A => space, B => tab, C => either of CR/LF

Numbers are binary (A=0, B=1, C=terminator)
Strings are sequences of binary characters, terminated by C.

We have:

* Stack instructions (Preceded by A)
     Push (Integer)    A
     Dup           CA
     Swap          CB
     Discard       CC

* Arithmetic (Preceded by BA)
     Plus          AA
     Minus         AB
     Times         AC
     Divide        BA
     Modulo        BB

* Heap access (Preceded by BB)
     Store         A
     Retrieve      B

* Control     (Preceded by C)
     Label String  AA
     Call Label    AB
     Jump Label    AC
     If Zero Label BA
     If Neg Label  BB
     Return        BC
     End           CC

* IO instructions (Preceded by BC)
     OutputChar    AA
     OutputNum     AB
     ReadChar      BA
     ReadNum       BB

-}
