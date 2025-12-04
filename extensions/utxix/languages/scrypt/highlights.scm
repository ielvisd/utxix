; sCrypt decorators — hot jaguar pink
(decorator) @property

; Bitcoin types — ice blue
(type_identifier) @type

; assert() — blood red
((call_expression
   function: (identifier) @function.builtin
   (#eq? @function.builtin "assert"))) @keyword

; Built-in sCrypt helpers
((identifier) @function.builtin
 (#any-of? @function.builtin "sha256" "ripemd160" "hash160" "checkSig" "checkMultiSig"))
