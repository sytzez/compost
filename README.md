# The Compost Programming Language

This is the compiler for my experimental programming language 'Compost'.
The compiler itself is written in Rust.

It doesn't yet compile a Compost program to a binary, but it **does** analyse and execute Compost code and show its output.

## Playground

You can run Compost code from your browser at the [Compost Playground](http://compost-playground.sytzez.com).

## Usage

You need to have the Rust installed to run the Compost compiler or to build it into a binary.

There are some code examples inside the `/examples` folder.

To run a Compost source code file named `examples/functions_and_constants.compost`, run:

```bash
cargo run examples/functions_and_constants.compost
```

## Features

- Functions and constants
- Int and String literals
- Classes and structs
- Full encapsulation of implementation details behind 'traits'
- Automatic trait implementations
- Polymorphism
- Multiple inheritance through automatic trait implementations
- Complex types using `&` and `|`
- Static type checking
- Type coercion
- Matching based on types

## Related Blogs

I've written a number of blogs before and during the implementation of Compost.

- Sketch for a New Programming Language
  - [Part 1](https://sytzez.com/blog/sketch-for-a-new-programming-language/)
  - [Part 2](https://sytzez.com/blog/sketch-for-a-new-programming-language-2/)
- Creating a Compiler for Compost using Rust
  - [Part 1: Lexical Analysis](https://sytzez.com/blog/creating-a-compiler-for-compost-using-rust-part-1-lexical-analysis/)
  - [Part 2: Syntactic Analysis](https://sytzez.com/blog/creating-a-compiler-for-compost-using-rust-part-2-syntactic-analysis/)
  - [Part 3: Semantic Analysis](https://sytzez.com/blog/creating-a-compiler-for-compost-using-rust-part-3-semantic-analysis/)
  - TBC...

## About the Programming Language

Compost is an experimental programming language designed to maximize the composability and reusability of code.

It is a functional, statically typed language. Types are purely based on the traits a value is expected to implement, allowing polymorphism.

The language attempts to solve the problems associated with object oriented inheritance.

See below an overview of its **currently implemented** features. All of the code example work with the current compiler.

### Functions and Constants

Functions and constants are defined using the `lets` keyword. A constant is just a function without any parameters.

```
lets
    MyConstant: Int
        42
        
    MyFunction: (a: Int, b: Int) -> Int
        a + b
        
    Main: Int
        MyFunction(a: MyConstant, b: 10)
        
#> 52
```

The `Main` function specifies the output of your program.

### Classes

A class is defined inside a module using the `class` keyword. The class will have the same name as the module.
Defining a class will automatically define an eponymous constructor function.
It will also define an eponymous type, which is equal to all the traits that are defined on the class (more about this in the next chapter).

To understand why a class needs to be inside a module, see the section on "Class 'Inheritance'" below.

After the `class` keyword, you should define the dependencies of that class which will be accessible inside the classes trait definitions.
The dependencies will be the parameters of the classes constructor function.
There is no way to directly access a class instances dependencies other than through its own trait definitions.

```
mod Point
    class
        x: Int # This is a dependency of the Point class.
        y: Int

lets
    # This constant is of the Point type. It be anything that implements the classes traits.
    MyPoint: Point
        # This is a call to the Point constructor function.
        Point 
            x: 1
            y: 2
    
    # Since the Point type currently doesn't implement any traits, *anything* can be a Point.
    OtherPoint: Point
        10
        
    Main: String
        'See the example below for Point output'

#> See the example below for Point output
```

### Traits and Definitions

Traits are declared inside a module using the `traits` keyword. Each trait declares a name and an output type.
A trait may also declare parameters, just like a function.

Classes may define (implement) traits using the `defs` keyword. These are called definitions.
Definitions can call other functions and may make use of their own classes dependencies.

```
mod Point
    class
        x: Int
        y: Int
    traits
        X: Int # Accessor for x.
        Y: Int
        Opposite: Point # A trait that returns something of the Point type.
    defs
        X: x
        Y: y
        Opposite
            Point
                x: -.X # .X is a shorthand for Self.X.
                y: -.Y
        # The String definition allows the Point to be an output of Main.
        String: .X.String + ', ' + .Y.String

lets
    MyPoint: Point
        Point 
            x: 1
            y: 2

    Main: Point
        MyPoint.Opposite

#> -1, -2
```

Each class definition will automatically declare an eponymous trait, which can be defined on other classes to provide
a way to convert that class into a value that implements former classes type.
For example, defining the `String` trait on a class provides a way to convert that class into a `String`.

The output of the `Main` function *must* define the `String` trait or be instance of the `String` class.

### Complex Types

Complex types can be created by combining traits and modules with `|` and `&`.
The compiler will figure out which traits you can call on such a type.
If you want to specifically use the Trait rather than the Interface of a certain name,
use `@` in front of the name. For example, `String` refers to something that defines the whole `String` module,
while `@String` refers to something that defines the `String` trait to allow it to be transformed into a `String`.

```
mod Name
    class(value: String)
    defs(String: value)

mod Age
    class(value: Int)
    defs(Int: value, String: value.String)

mod Human
    class(name: Name, age: Age)
    defs(Name: name, Age: age)

mod Animal
    class(name: Name, age: Age)
    defs(Name: name, Age: age)

lets
    # Takes anything that implements the Human or Animal module.
    Greeting: (greeted: Human | Animal) -> String
        # We can call the Name trait because it exists on both Human and Animal
        'Hello, ' + greeted.Name.String

    # Takes anything that defines the Name and Age traits on it.
    # We need to use the @ symbol, otherwise this means something that implements both the
    # Name and Age module itself!
    NameAndAge: (subject: @Name & @Age) -> String
        subject.Name.String + ' (' + subject.Age.String + ')'

    Bob: Human
        Human
            name: Name(value: 'Bob')
            age: Age(value: 20)

    Fifi: Animal
        Animal
            name: Name(value: 'Fifi')
            age: Age(value: 3)

    Main: String
        Greeting(greeted: Bob) + '. '
        + Greeting(greeted: Fifi) + '. '
        + NameAndAge(subject: Bob) + '. '
        + NameAndAge(subject: Fifi)

#> Hello, Bob. Hello, Fifi. Bob (20). Fifi (3)
```

### Automatic Definitions

Traits can be declared on a module with no class. 
If a class defines some of those traits, other traits of the module may be automatically defined for that class.

```
mod Point
    class
        x: Int
        y: Int
    traits
        X: Int
        Y: Int
    defs
        X: x
        Y: y
        String: .X.String + ', ' + .Y.String

mod Rectangle
    traits
        # A 'Rectangle' must have definitions for these traits.
        TopLeft: Point
        BottomRight: Point
        Width: Int
        Height: Int
    defs
        # Some automatic definitions based on other traits we have.
        TopLeft
            Point
                x: .BottomRight.X - .Width
                y: .BottomRight.Y - .Height
        BottomRight
            Point
                x: .TopLeft.X + .Width
                y: .TopLeft.Y + .Height
        Width: .BottomRight.X - .TopLeft.X
        Height: .BottomRight.Y - .TopLeft.Y

# A class that implements 'Rectangle', constructed using a point an size.
mod RectangleBySize
    class
        topLeft: Point
        width: Int
        height: Int
    defs
        Rectangle\TopLeft: topLeft
        Rectangle\Width: width
        Rectangle\Height: height
        # BottomRight is automatically defined for this class using the definition on the Rectangle module.

# A class that implements 'Rectangle', constructed using two points.
mod RectangleByPoints
    class
        topLeft: Point
        bottomRight: Point
    defs
        Rectangle\TopLeft: topLeft
        Rectangle\BottomRight: bottomRight
        # Width and Height are automatically defined for this class using the definitions on the Rectangle module.

lets
    # Rectangle is the type of this constant.
    # RectangleBySize implements the Rectangle type because it defines all traits of Rectangle.
    A: Rectangle
        RectangleBySize
            topLeft
                Point
                    x: 10
                    y: 5
            width: 20
            height: 10

    # RectangleByPoints implements the Rectangle type because it defines all traits of Rectangle.
    B: Rectangle
        RectangleByPoints
            topLeft
                Point
                    x: 10
                    y: 5
            bottomRight
                Point
                    x: 15
                    y: 15

    # The following values are calculated using automatic definitions from the Rectangle module.
    Main: String
        'BottomRight of A: ' + A.BottomRight.String
        + '. Width and Height of B: ' + B.Width.String
        + ', ' + B.Height.String

#> BottomRight of A: 30, 15. Width and Height of B: 5, 10
```

### Multiple Inheritance

Compost inheritance works by (automatically) implementing another module's traits. 
You can implement as many traits from different modules as you like, allowing multiple inheritance.
You can also use the `using` keyword to automatically implement all traits from a module that can be automatically implemented.

Because types are based on which traits are implemented, sub-classes can be substituted from the super-class,
allowing full polymorphism.

```
#########################
#                       #
#      Animal      Egg  #
#     /      \      .   #
#    /        \    .    #
#  Mammal   Amphibian   #
#    \        /         #
#     \      /          #
#     Platypus          #
#                       #
#########################

mod Animal
    traits
        Name: String
        SpeciesName: String
        ChildsName: String
    defs
        ChildsName: 'Child of ' + .Name

mod Mammal
    using(Animal\*)
    traits
        RegulateBodyTemperature: String
    defs
        RegulateBodyTemperature: 'Regulating...'

mod Amphibian
    using(Animal\*)
    traits(LayEgg: Egg)

mod Egg
    class(embryo: Amphibian)
    traits(Hatch: Amphibian)
    defs(Hatch: embryo)

mod Platypus
    using
        Mammal\*
        Amphibian\*
    class
        name: String
    defs
        Animal\Name: name
        Animal\SpeciesName: 'Platypus'
        Amphibian\LayEgg
            Egg
                embryo: Platypus(name: .ChildsName)

lets
    FullInformation: (animal: Animal) -> String
        animal.Name + ' (species: ' + animal.SpeciesName + ')'

    MyPlatypus: Platypus
        Platypus(name: 'Perry')

    Main: String
        FullInformation
            animal: MyPlatypus.LayEgg.Hatch

#> Child of Perry (species: Platypus)
```

### Structs

Class dependencies can not have raw types, since those types aren't based on traits.
If we allowed class dependencies to have raw types, those dependencies would lose their flexibility.
Instead, you can define a struct instead of a class using the `struct` keyword.

A struct behaves like a class, but it has fields instead of dependencies.
A structs fields should be of raw types such as `int` or `string`.

Structs can access the fields of other structs of the same type in its definitions.

See for example, the `Int` struct from the standard library:

```
mod Int
    struct
        value: int
    defs
        Op\Add: Int(value: value + rhs.value)
        Op\Sub: Int(value: value - rhs.value)
        Op\Mul: Int(value: value * rhs.value)
        Op\Div: Int(value: value / rhs.value)
        Op\Neg: Int(value: -value)
        String: String(value: value.toString)
```

## Architecture

The compiler currently uses pure Rust without any dependencies other than the standard library.
The compilation process is split up in a few modules:
- Lexical analysis (`lex`) - Reads raw code into tokens.
- Abstract syntax analysis (`ast`) - Reads tokens into an abstract syntax tree.
- Semantic analysis (`sem`) - Resolves abstract syntax tree into semantic objects such as modules, traits and classes.
- Runtime (`runtime`) - Instantiates classes and calculates actual results.

In the future there will be modules to replace the runtime module, which compile the code down into a binary file.

For more details about the implementation of this compiler see my [blog posts](#related-blogs).

## The Future of Compost

There are many features of Compost that I have designed but haven't had the time to implement yet, such as:
- Functions and constants within modules.
- Operator precedence.
- Enum types.
- Array types.
- Control flow keywords such as `if` and `for`.
- Better compiler errors.
- Compiling to binary.
