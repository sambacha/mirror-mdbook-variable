
## MdBook  Variables preprocessor

A simple preprocessor for mdbook, that look for variable in double brackets and replace with some value that come from the book.toml

### Example

SimpleFile.md:
```
## something

a contentent with a variable {{name}} 
```

book.toml
```toml
#... all the basic detail first and then:

[preprocessor.variables.variables]
name= "my wonderful name"
```

It does support looking for variable values in the environment variables behind a flag

Toml configured variable right now take precedence on environment variables.
```toml

[preprocessor.variables]
use_env = true

[preprocessor.variables.variables]
other_variabled_not_in_env= "value"
```


The implementation got a lot of inspiration and code from the mdbook links preprocessor an mdbook-plantuml.






