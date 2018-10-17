---
title: Variables and Blocks
permalink: /docs/var-blocks/
---

Variables declared outside of block scope are still accessible within the block.

{% highlight julia %}
    let x := 1
    let y := begin
        x
    end
    y   #=> 1
{% endhighlight %}

New variable bindings for the same identifier can be created within block scope without changing the original binding. This is a temporary instance of shadowing.

{% highlight julia %}
    let x := 1
    begin
        let x := 2
    end
    x   #=> 1
{% endhighlight %}

This also prevents immutable bindings from being mutated within a block.

{% highlight julia %}
    let x := 1
    begin
        let mut x := x
        x += 1
    end
    x   #=> 1
{% endhighlight %}

But mutable bindings can be mutated within block scope.

{% highlight julia %}
    let mut x := 1
    begin :for
        x += 1
        if x == 10
            break :for
        end
    end
    x   #=> 10
{% endhighlight %}
