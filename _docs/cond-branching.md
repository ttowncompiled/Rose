---
title: Conditional Branching
permalink: /docs/cond-branching/
---

Rose supports Ruby-like syntax for conditional branching.

{% highlight ruby %}
    if true begin
        @puts "True!"
    else begin
        @puts "False!"
    end
    #=> True!
{% endhighlight %}

The use of `begin` is optional.

{% highlight ruby %}
    if true
        @puts "True!"
    else
        @puts "False!"
    end
    #=> True!
{% endhighlight %}

There can be multiple branches.

{% highlight ruby %}
    if false
        @puts "Won't print!"
    elsif false
        @puts "Also won't print!"
    elsif false
        @puts "Also won't print!"
    else
        @puts "Will print!"
    end
    #=> Will print!
{% endhighlight %}

