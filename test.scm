(block)@test(mod_item name: (identifier)@namespace)
(scoped_identifier(scoped_identifier path: (identifier) @rust_path) (#set! conceal ""))((field_identifier) @constant (#lua-match? @constant "^[A-Z]"))
"=" @something
("=") @something
(helloworld
  "hello"
  (mynode)
  "world")
["(" ")" "[" "]" "{" "}"]  @punctuation.bracket
