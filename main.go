func main() {
	x := a != b && c <= d
	y := foo == bar || baz >= qux
	z := x := <-ch
	ok := a && b || c
	if err != nil {
		return fmt.Errorf("failed: %w", err)
	}
	result := a + b - c * d / e
	value := x -> y // not valid Go, but tests the glyph
	_ = func(a, b int) bool { return a == b }
}
