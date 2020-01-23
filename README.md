
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

The implementation got a lot of inspiration and code from the mdbook links preprocessor an mdbook-plantuml.






