# The Compost Programming Language

This is the compiler for my experimental programming language 'Compost'.
The compiler itself is written in Rust.

It currently doesn't actually compile a Compost program to a binary, but it **does** execute Compost code and show its output.

## Playground

You can run Compost code from your browser at the [Compost Playground](http://compost-playground.sytzez.com).

## Usage

You need to have the Rust compiler installed to run the Compost compiler or to build it into a binary.

There are some code examples inside the `/examples` folder.

To run a Compost source code file named `examples/functions_and_constants.compost`, run:

```bash
cargo run --release examples/functions_and_constants.compost
```

## About the Programming Language

Compost is an experimental programming language designed to maximize the composability and reusability of code.

It is a functional, statically typed language. Types are purely based on the traits a value is expected to implement, allowing polymorphism.

The language attempts to solve the problems associated with object oriented inheritance.

For more information see my blogposts
[Sketch for a new programming language](https://sytzez.com/blog/sketch-for-a-new-programming-language/)
and
[Sketch for a new programming language: Part 2](https://sytzez.com/blog/sketch-for-a-new-programming-language-2/)
.

See below an overview of its **currently implemented** featured. All of the code example work with the current compiler.

### Functions and Constants

Functions and constants are defined using the `lets` keyword. A constant is just a function without any parameters.

```
lets
    MyConstant: Int
        Int(value: 42)
        
    MyFunction: (a: Int, b: Int) -> Int
        a + b
        
    Main: Int
        MyFunction(a: MyConstant, b: Int(value: 10))
        
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
        Point # This is a call to the Point constructor function.
            x: Int(value: 1)
            y: Int(value: 2)
    
    # Since the Point type currently doesn't implement any traits, *anything* can be a Point.
    OtherPoint: Point
        Int(value: 10)
        
    Main: String
        String(value: 'See the example below for Point output')

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
        String: .Point\X.String + String(value: ', ') + .Point\Y.String

lets
    MyPoint: Point
        Point 
            x: Int(value: 1)
            y: Int(value: 2)

    Main: Point
        MyPoint.Point\Opposite

#> -1, -2
```

Each class definition will automatically declare an eponymous trait, which can be defined on other classes to provide
a way to convert that class into a value that implements former classes type.
For example, defining the `String` trait on a class provides a way to convert that class into a `String`.

The output of the `Main` function *must* define the `String` trait or be instance of the `String` class.

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
        Point\X: x
        Point\Y: y
        String: .Point\X.String + String(value: ', ') + .Point\Y.String

mod Rectangle
    traits
        # A 'Rectangle' must have definitions for these traits.
        TopLeft: Point
        BottomRight: Point
        Width: Int
        Height: Int
    defs
        # Some automatic definitions based on other traits we have.
        Rectangle\TopLeft
            Point
                x: .Rectangle\BottomRight.Point\X - .Rectangle\Width
                y: .Rectangle\BottomRight.Point\Y - .Rectangle\Height
        Rectangle\BottomRight
            Point
                x: .Rectangle\TopLeft.Point\X + .Rectangle\Width
                y: .Rectangle\TopLeft.Point\Y + .Rectangle\Height
        Rectangle\Width: .Rectangle\BottomRight.Point\X - .Rectangle\TopLeft.Point\X
        Rectangle\Height: .Rectangle\BottomRight.Point\Y - .Rectangle\TopLeft.Point\Y

# A class that implements 'Rectangle', constructed using a point and a size.
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
                    x: Int(value: 10)
                    y: Int(value: 5)
            width: Int(value: 20)
            height: Int(value: 10)

    # RectangleByPoints implements the Rectangle type because it defines all traits of Rectangle.
    B: Rectangle
        RectangleByPoints
            topLeft
                Point
                    x: Int(value: 10)
                    y: Int(value: 5)
            bottomRight
                Point
                    x: Int(value: 15)
                    y: Int(value: 15)

    # The following Constants are calculated using the automatic definitions from the Rectangle module.
    BottomRightOfA: Point
        A.Rectangle\BottomRight

    WidthOfB: Int
        B.Rectangle\Width

    HeightOfB: Int
        B.Rectangle\Height

    Main: String
        String(value: 'BottomRight of A: ') + BottomRightOfA.String
        + String(value: '. Width and Height of B: ') + WidthOfB.String
        + String(value: ', ') + HeightOfB.String

#> BottomRight of A: 30, 15. Width and Height of B: 5, 10
```

### Class 'Inheritance'

Instead of having classical OOP inheritance, Compost mimics inheritance by letting you define the traits on one class on other classes.
This works the exact same way as classes define traits from classless modules as shown in the section above.

This allows a lot of flexibility. It allows full polymorphism since any class instance can be substituted by the instance of another class
as long as it defines all of its traits. It also allows 'inheriting' from multiple classes, since a class can define an unlimited amount of traits.

```
mod Rectangle
    class(x: Int, y: Int, width: Int, height: Int)
    traits(X: Int, Y: Int, Width: Int, Height: Int, Area: Int)
    defs
        Rectangle\X: x
        Rectangle\Y: y
        Rectangle\Width: width
        Rectangle\Height: height
        Rectangle\Area: .Rectangle\Width * .Rectangle\Height

# Square 'inherits' from Rectangle by defining some of its traits.
# The remaining traits are 'inherited' from the Rectangle class.
# The Square implements the Rectangle type because if defines all of its traits.
mod Square
    class(x: Int, y: Int, size: Int)
    defs
        Rectangle\X: x
        Rectangle\Y: y
        # We 'override' the Width and Height from Rectangle.
        Rectangle\Width: size
        Rectangle\Height: size
        # Rectangle\Area is automatically defined by the Rectangle module.

lets
    Main: Int
        Square
            x: 1
            y: 1
            size: 10
        .Rectangle\Area

#> 100
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

Ideally there will be modules to replace the runtime module which compile the code down into a binary file.

## The Future of Compost

There are many features of Compost that I have designed but haven't had the time to implement yet, such as:
- Functions and constants within modules.
- Automatically resolving `string` and `int` literals to `String` and `Int` structs, so we can do
  `MyPoint.String + '!'` instead of `MyPoint.String + String(value: '!')`.
- Shortened localized references to traits, functions and constants so we can do `Circumference: .Width * 2 + .Height * 2` 
  instead of `Rectangle\Circumference: .Rectangle\Width * 2 + .Rectangle\Height * 2`
- Operator precedence.
- More advanced types such as `Op/Add & Op/Sub`.
- Enum types.
- Array types.
- Control flow keywords such as `if` and `for`.
- Better compiler errors.
- Compiling to binary.
