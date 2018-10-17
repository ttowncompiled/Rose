---
title: Identifier Names
permalink: /docs/ident-names/
---

All identifiers which includes variables, types, procedures, fields, and symbols must use the following format.

They can begin with a lowercase or uppercase alpha character or an underscore. Followed by any number of lowercase/uppercase alpha characters, underscores, or numbers.

{% highlight ruby %}
    /[a-zA-Z_]+[a-zA-Z0-9_]*/
{% endhighlight %}

As such, any instance of a number prepended to an identifier will result in composition of the two objects.

{% highlight ruby %}
    4x == 4*x   #=> true
{% endhighlight %}

