---
title: Punctuation
permalink: /docs/punctuation/
---

Semicolons are optional in Rose. A semicolon is equivalent to a newline character and can be used anytime a newline character can be used.

{% highlight ruby %}
    let x := 1
    let y := 2
{% endhighlight %}

is equivalent to

{% highlight ruby %}
    let x := 1 ; let y := 2
{% endhighlight %}

Commas are also optional in Rose. A comma is equivalent to a space character and be used anytime a space character can be used.

{% highlight ruby %}
    with x := 1 begin
        x
    end
{% endhighlight %}

is equivalent to

{% highlight ruby %}
    with x := 1, begin
        x
    end
{% endhighlight %}

It is recommend that semicolons and commas are used sparingly when it improvides readability to include them.
