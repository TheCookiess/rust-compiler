an attempt at making a compiler in rust!

### Currently working on: 
  - [ ] if
    - [x] rework lexer to handle multi-symbol keywords (i.e "==" or "<=")
    - [x] parse boolean comparison 
    - [x]  else & else if parsing
    - [ ] code generation
      - [x] invert 'jump' conditions
      - [ ] unsigned vs signed comparison (diff jump instructions)
      - [x] binary expr conditions
        - conditional expr, either has explicity bool comparison or implicit, expr = lhs: (lhs > 0)
        - cmp reg1, reg2 ; compare arguments
        - set(EQUIVALENCE e.g e, le) al; sets register 'al' (8 bit) to 1,0 depending on cmp flag
        - movzx output_reg, al ; Move Zero Xtend. copies 'al' into 'output_reg' && zero init reg1 bits.  
    - [x] types of scope
      - inherits variables from parent scope (if, else if, else) 
      - doesn't (new function, UNLESS class, inherits 'self')
  - [x] (kinda done) split 'TokenKind': 'Symbol' .. 'LogicalOp' .. 'BinaryOp' .. etc
  - [x] comments
  - [ ] dynamically place variables on stack if they are(nt) used immediately. 
    - use multiple registers to store arguments for a binary expr
    - add stack values at top level (expr, stmt), not term. do ^^
  - [ ] redesign Lexer, Result<T> && self.read_char() returns char?, akin to self.consume()
  - [ ] update grammar to match code. 
  - [ ] variable reassignement (mutability)
  - [ ] data types
    - [ ] bools: al register (8 bit)
    - [ ] u8: al
    - [ ] u16: ax 
    - [ ] u32: eax 
    - [ ] u64: rax
  - [ ] functions


