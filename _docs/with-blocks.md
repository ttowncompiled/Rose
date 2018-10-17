---
title: With Blocks
permalink: /docs/with-blocks/
---

When declaring a block, the `with` keyword can be used to declare variables for the block.

{% highlight julia %}
    with x, y := 1, 2, begin,
        x + y.
    end.
    #=> 3
    #=> x, y no longer exist
{% endhighlight %}

These declarations are only executed once when the block is first entered.

{% highlight julia %}
    with mut a := b := 1, begin :fib,
        a, b = b, a+b.
        if b > 10,
            break :fib.
        end.
        next.
    end.
    # b = 1, 2, 3, 5, 8, 13
    # a, b no longer exist
{% endhighlight %}

