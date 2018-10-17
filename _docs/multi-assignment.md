---
title: Multiple Assignment
permalink: /docs/multi-assignment/
---

There are a few ways to declare multiple variables on the same line.

The `:=` operator can be used to assign the same value to multiple variables.

{% highlight ruby %}
    let x := y := z = 1
    x + y + z   #=> 3
{% endhighlight %}

The `:=` operator can also _splat_ the right-hand-side and assign different values to a `,` separated list of variables.

{% highlight ruby %}
    let x, y, z := ( 1, 2, 3 )
    x + y + z   #=> 6
    let x, y, z := 4, 5, 6  # comma separated values can also be splatted
    x + y + z   #=> 15
{% endhighlight %}

Multiple assignment expressions can be broken up on the same line using the `;` separator. This is useful when some variables need to have mutable bindings.

{% highlight ruby %}
    let x := y := 1 ; let mut z := 2
    x + y + z   #=> 4
{% endhighlight %}

Once variables have been defined, their values can be easily swapped like so.

{% highlight ruby %}
    let x := 1 ; let y := 2
    let x, y := y, x
    x   #=> 2
    y   #=> 1
{% endhighlight %}

