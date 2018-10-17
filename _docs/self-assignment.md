---
title: Self-assignment
permalink: /docs/self-assignment/
---

There is a special type of assignment which occurs when the same variable identifier is used on both the left- and right-hand sides of the same assignment. In such cases, if the variable has not yet been defined, then, rather than throwing an error, the value of the variable on the right-hand side is treated as `()` (which is `nil`).

{% highlight ruby %}
    let a := a + 1  # doesn't throw an error
    a   #=> 1
{% endhighlight %}

This will, however, generate a warning by the compiler which can be ignored or suppressed when this behavior is intended.

This does not apply if the variable on the right is not the same as the variable on the left.

{% highlight ruby %}
    let a := b + 1  # throws an error
{% endhighlight %}

The same rules apply for multi-assignment.

{% highlight ruby %}
    let a, b := a+1, b+2
    a + b   #=> 3
{% endhighlight %}

All variables on the left are considered.

{% highlight ruby %}
    let a, b := b+1, a+2
    a + b   #=> 3
{% endhighlight %}

