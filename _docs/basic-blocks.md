---
title: Basic Blocks
permalink: /docs/basic-blocks/
---

Multiple statements can be grouped together in a block scope using the `begin` and `end` keywords.

{% highlight julia %}
    begin
        let x := 1
        let y := 2
        let z := x + y
        z
    end
    # x, y, z no longer exist
{% endhighlight %}

Blocks are expressive. The value of a block is the value of the last expression that was evaluated within the block.

{% highlight julia %}
    let c := begin
        let a := 1
        let b := 2
        a + b
    end
    # a, b no longer exist
    c #=> 3
{% endhighlight %}

Blocks can be broken out of with the `break` keyword.

{% highlight julia %}
    begin
        let a := 1
        let b := 2
        break
        a + b   # never executed
    end
{% endhighlight julia %}

The value of a broken block is the same as the last expression that was evaluated before the break.

{% highlight julia %}
    let c := begin
        let a := 1
        let b := 2
        a
        break
        a + b
    end
    c   #=> 1
{% endhighlight %}

Jumping can also occur within a block using the `next` keyword. This jumps to the top of the block without losing scope, so variables retain their value.

{% highlight julia %}
    let mut a := b := 1
    begin
        a, b = b, a+b   # b = 1, 2, 3, 5, 8, 13, ...
        next    # this is an infinite loop for the Fib sequence
    end
{% endhighlight %}

Breaking and jumping don't break or jump out further than the current block scope.

{% highlight julia %}
    begin
        let mut a := b := 1
        begin
            a, b = b, a+b
            if b > 10
                break   # only breaks out of the nested block
            end
        end
        next    # jumps to the top of the outer block, still an infinite loop
    end
{% endhighlight %}

