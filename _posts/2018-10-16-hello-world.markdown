---
layout: post
title:  "Hello, World!"
date:   2018-10-16 19:30:00 -0500
categories: jekyll update
---
The classic "Hello, World!" program can be written in a single line.

{% highlight ruby %}
    @puts "Hello, World!"   #=> Hello, World!
{% endhighlight %}

Rose, like Ruby and Julia, can be used as a scripting language. No main method is necessary as the program itself is treated as the main routine.

The symbol `@` followed by an identifier is a macro. Rose is a compiled language that supports macros. Some macros are provided by the std library and some are user defined. Others, like `@puts` are actually keyed into the compiler to emit a specific intermediate representation (IR).

`"Hello, World!"` is a string literal. String literals can be declared with `''`, `""`, or ``` `` ```.

