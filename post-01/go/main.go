package main

import "fmt"

// The bug: a pointer into a slice's backing array outlives the array.
//
// `s` starts with capacity 3. append(s, 4) has no room, so it allocates a new,
// bigger backing array and copies the elements over. `p` still points into the
// OLD array — which nothing else references now. *p = 99 writes there and is
// silently lost.
//
// Run it:  go run main.go
// Expect:  [1 2 3 4]   (the 99 went nowhere)
//
// Then try the contrast: give s spare capacity so append does NOT reallocate,
// and watch the bug vanish:
//
//     s := make([]int, 3, 8)
//     copy(s, []int{1, 2, 3})
//
// Now *p = 99 writes into the array s still points at, and you'll see the 99.
func main() {
	s := []int{1, 2, 3}
	p := &s[0]
	s = append(s, 4)
	*p = 99
	fmt.Println(s)
}
