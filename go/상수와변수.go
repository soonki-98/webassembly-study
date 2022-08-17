package main

func main() {
	var a int
	println(a)

	var f float32 = 11.
	println(f)

	var i, j, k int = 1, 2, 3
	println(i, j, k)

	const c int = 10
	const s string = "Hi"
	println(c, s)

	const (
		one   = 1
		two   = 2
		three = 3
	)
	println(one, two, three)

	const (
		ZERO = iota
		ONE
		TWO
	)
	println(ZERO, ONE, TWO)
}