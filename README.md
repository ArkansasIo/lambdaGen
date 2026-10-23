# lambdaGen

A Rust-based lambda language and research runtime with Lua-inspired object-oriented syntax and sentence/paragraph logic for numerical reasoning.

## Features

- Rust runtime
- Lambda/closure predicates
- Lua-inspired objects and methods
- Even, odd, prime, and composite classification
- Human-readable sentence and paragraph logic examples
- Foundation for a future interpreter/compiler

## Example

```text
object Number
    property value : integer

    function is_even()
        return self.value % 2 == 0
    end

    function is_odd()
        return self.value % 2 ~= 0
    end

    function is_prime()
        if self.value < 2 then
            return false
        end
        return prime(self.value)
    end
end
```

## Status

Pre-alpha research implementation.
