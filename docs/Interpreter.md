## 



### Interpreter

> watch table and display function

#### Get Value

* `#`: immediate number:
* `*`: get value of pointer address (usually array)
* `&`: get address

#### IF

> IF x [relop] y GOTO z

1. calculate the operation result about `x [relop] y`
2. if result is `true`, modify the ip to y

#### MOV, ARITH

get value and set 

#### CALL

1. record previous symbol table and offset
2. append (ip, variable, previous symbol table, offset) to call stack
3. modify ip

#### RETURN

1. get the value to return 
2. get the item from call stack
3. recover the ip and offset
4. assign return value to variable

### Computer

#### Memory Operation

> byte encoding, a value need 4 bytes

memory: `self.mem[self.symtable[label][0] // 4]`

```rust
fn load(self, offset: usize) => u32 {
    return self.memory[offset / 4]
}

fn save(self, offset: usize, value: u32) => {
    self.memeory[offset / 4] = value
}
```

#### I/O: READ/WRITE

you should use a closure to set this IO interface.

* READ x: load extern value into variable x
* WRITE x: output the value of variable x 