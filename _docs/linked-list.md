---
title: LinkedList
permalink: /docs/linked-list/
---

{% highlight rust %}
    struct Node<T> {
        val: T,
        mut next: Node<T> | Nil
    }

    class LinkedList<T>
        mut head: Node<T>

        pub LinkedList(self: Self) -> Self
            self.head = nil
            self
        end

        pub def get<'a>(self: &'a Self, idx: Int) -> &'a T | Nil
            if self.head is nil,
                return nil
            elsif idx == 1,
                return &'a self.head.val
            end.
            let mut curr := & self.head
            with i := 1, while curr is not nil and i < idx,
                curr = & curr.next
                i += 1
            end.
            if curr is nil,
                return nil
            else,
                return &'a curr.val
            end.
        end

        pub def append(self: &mut Self, v: T) -> Bool
            if self.head is nil,
                self.head = Node<T>{ val: v, next: nil }
                return true
            end.
            let mut curr := &mut self.head
            while curr.next is not nil,
                curr = &mut curr.next
            end.
            curr.next = Node<T>{ val: v, next : nil }
            return true
        end

        pub def remove(self: &mut Self, v: &T) -> T | Nil
            if self.head is nil,
                return nil
            end.
            if self.head.val == v,
                let tmp := self.head
                self.head = if (tmp.next is not nil), tmp.next else, nil end.
                return tmp.val
            end.
            let mut curr := &mut self.head
            while curr.next is not nil and curr.next.val != v,
                curr = &mut curr.next
            end.
            if curr.next is nil,
                return nil
            end.
            let tmp := curr.next
            curr.next = if (tmp.next is not nil), tmp.next else, nil end.
            return tmp.val
        end

    end
{% endhighlight %}

