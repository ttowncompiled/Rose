---
title: Assignment
permalink: /docs/assignment/
---

Variables can be assigned using the `let` keyword and the assignment operator `:=`.

{% highlight ruby %}
    let x := 1 .
    # x : Int32
{% endhighlight %}

`:=` is the inference assignment operator. It infers the type of the variable from the type of the right-hand-side expression.

Types can also be explicitly declared upon assignment.

{% highlight ruby %}
    let x : Int32 = 1 .
{% endhighlight %}

In Rose, variable bindings are immutable by default. To define a mutable variable binding, the `mut` keyword must be used.

{% highlight ruby %}
    let x := 1 .
    x += 1 .    # throws error
    let mut x := 1 .
    x += 1 .
    x.  #=> 2
{% endhighlight %}

Alternatively, bindings can be updated by redeclaring the binding.

{% highlight ruby %}
    let x := 1 .
    let x := x + 1 .
    x.  #=> 2
{% endhighlight %}

Redeclaring a binding can also be used to alter the type of the binding.

{% highlight ruby %}
    let x : Int32 = 1 .
    let x : Float64 = x + 1 .
    # x : Float64
    x.  #=> 2.0
{% endhighlight %}

Redeclaring a binding can also be used to make an immutable binding mutable and vice versa.

{% highlight ruby %}
    let x := 1 .
    let mut x := x.
    x += 1 .
    let x := x.
    x.  #=> 2
    x += 1 .    # throws an error
{% endhighlight %}

This redeclaring of variable bindings is called _shadowing_.

