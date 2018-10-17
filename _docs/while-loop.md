---
title: While Loop
permalink: /docs/while-loop/
---

There are a number of different approaches to creating loops in Rose. One approach is to use the classic `while` loop.

{% highlight ruby %}
    let x := 1 .
    while x < 10, begin,
        x += 1
    end.
    x.  #=> 10
{% endhighlight %}

The `begin` is optional.

{% highlight ruby %}
    let x := 1 .
    while x < 10,
        x += 1
    end.
    x.  #=> 10
{% endhighlight %}

`while` loops have the same scoping rules as basic blocks.

`while` loops can be named, and they can also be used in conjunction with the `with` keyword. This is useful for producing ranges, but not for incrementing variables.

{% highlight ruby %}
    with x := 1, while x < 10,
        x += 1
    end.
    # x no longer exists
{% endhighlight %}

`while` loops are expressive and have the same value as the value of the last expression that was evaluated in the loop before the condition was checked.

{% highlight ruby %}
    let z := with x := 1, while x < 10,
        x += 1,
        x.
    end.
    z.  #=> 10
{% endhighlight %}

`while` loops can be in-lined.

{% highlight ruby %}
    with x := 1, while x < 10, x += 1, end.
{% endhighlight %}

`while` loops can be broken out of and can be jumped to. However, if jumping is used, a `with` statement would not be reevaluated, but the condition of the loop would be rechecked.

{% highlight ruby %}
    with x := 1, while true, begin :loop,
        x += 1 .
        if x == 10,
            break :loop.
        end.
        next.
    end.
    # x = 1, 2, 3, ..., 10
{% endhighlight %}

Using a conditional branch to break out of loops and other blocks is so common that the syntactic sugar `::` is provided. This way, loops and blocks can be broken out of without requiring a name.

{% highlight ruby %}
    with x := 1, while true,
        x += 1 .
        if x == 10,
            break ::
        end.
        next.
    end.
    # x = 1, 2, 3, ..., 10
{% endhighlight %}

