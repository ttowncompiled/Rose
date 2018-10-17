---
title: Named Blocks
permalink: /docs/named-blocks/
---

Blocks can be named with a Symbol.

{% highlight julia %}
    begin :block
        let a := 1
        let b := 2
        a + b
    end
{% endhighlight %}

This allows breaking and jumping to go further than the current block scope.

{% highlight julia %}
    begin :fib
        let mut a := b := 1
        begin
            a, b = b, a+b
            if b > 10
                break :fib  # this is not an infinite loop
            end
        end
        next    # never executed
    end
{% endhighlight %}

Breaking and jumping cannot reach named blocks that are not within their scope.

{% highlight julia %}
    begin :hello
        @puts "Hello!"
    end
    begin
        next :hello # throws an error because block :hello is not within scope
    end
{% endhighlight %}

