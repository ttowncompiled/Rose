---
title: Variables and Conditional Branching
permalink: /docs/var-cond-branching/
---

Like basic blocks, each conditional branch declares its own block scope.

{% highlight ruby %}
    if true
        let x := 1
    end
    # x no longer exists
{% endhighlight %}

Conditional branches are expressive. The value of a conditional branch is the last expression that was evaluated in the branch that was chosen.

{% highlight ruby %}
    let z := if false
        1
    else
        2
    end
    z   #=> 2
{% endhighlight %}

Conditional branches can be inlined.

{% highlight ruby %}
    let cmp := if 1 == 2 0 elsif 1 > 2 1 else -1 end
{% endhighlight %}

Parenthesis can help with readability.

{% highlight ruby %}
    let cmp := if (1 == 2) 0 elsif (1 > 2) 1 else -1 end
{% endhighlight %}

As can semicolons.

{% highlight ruby %}
    let cmp := if (1 == 2) 0 ; elsif (1 > 2) 1 ; else -1 end
{% endhighlight %}

Conditional branches have access to variables declared outside of their scope so long as those variables are reachable.

{% highlight ruby %}
    let x := 1
    if true
        let y := x + 1
        y   #=> 2
    end
    # y no longer exists
{% endhighlight %}

And same as basic blocks, conditional branches using shadowing so variable identifiers can be temporarily reused and variable bindings can't be mutated.

{% highlight ruby %}
    let x := 1
    if true
        let mut x := x
        x += 1
        x   #=> 2
    end
    x   #=> 1
{% endhighlight %}

Conditional branches can be named, but only if they include the `begin` keyword.

{% highlight ruby %}
    if true begin :fib
        let a, b := b+0, if (a+b+0 == 0) 1 else a+b end
        if b > 10
            break :fib
        end
        next
    end
    # b = 1, 1, 2, 3, 5, 8, 13
{% endhighlight %}

The `with` keyword can be used for conditional branches.

{% highlight ruby %}
    with ok, val_or_err := SomeErrFunction() if ok
        # do stuff
    else
        @panic val_or_err
    end
{% endhighlight %}

Each conditional branch can use the variables that were declared using `with`.

{% highlight ruby %}
    with x := 1, if x < 0
        # do stuff
    elsif x == 1
        @puts "x == 1"
    end
    #=> x == 1
{% endhighlight %}

But only those that were declared above the conditional branch, not below it.

{% highlight ruby %}
    with x := 1, if false
        # do stuff
    with y := 2, elsif false
        y + z   # throws error
    with z := 3, else
        x + y   # this wouldn't throw an error
    end
{% endhighlight %}

