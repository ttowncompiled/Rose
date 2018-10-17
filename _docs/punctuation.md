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

Periods can also be used to provide punctuation. Periods are equivalent to a newline character, but they must be followed by a newline character or a space.

Periods can be appended to the `end` keyword.

{% highlight ruby %}
    begin
        let x := 1
    end.
{% endhighlight %}

Periods can also be appended to the end of a `let` declaration.

{% highlight ruby %}
    let x := 1 .
{% endhighlight %}

It should also be taken into consideration that periods can cause confusion to developers from other languages where `1.` is a `Float`. In Rose, `1.` is an `Int` and `1.0` is a `Float`.

{% highlight ruby %}
    let x := 1.
    # x : Int32
    let y := 1.0.
    # y : Float64
{% endhighlight %}

Lastly, periods can be used at the end of an expression.

{% highlight ruby %}
    add(1, 2).  #=> 3
{% endhighlight %}

The `.` symbol is often used a dot accessor, so care should be taken not to cause confusion in cases when punctuation might look like dot access.

It is recommend that semicolons, commas, and periods are only used when it improvides readability to include them.

The following documentation will attempt to make use of punctuation as it can be used without causing confusion.

