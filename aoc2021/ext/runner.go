package ext

import (
	"fmt"
	"os"
	"time"
)

// Runner ...
type Runner struct {
	fn RunFn
	ts TestCases
}

// NewRunner ...
func NewRunner(fn RunFn, ts TestCases) *Runner {
	return &Runner{fn, ts}
}

// RunFn ...
type RunFn func(string) (interface{}, interface{})

// Run ...
func (r *Runner) Run(puzzle string, verbose bool) {
	if puzzle != "" && verbose {
		fmt.Printf("input file: %s\n", puzzle)
	}
	if r.ts != nil {
		fmt.Print("testing...")
		r.Probe(verbose)
	}
	if r.fn == nil {
		fmt.Print("no RunFn")
		return
	}
	start := time.Now()
	part1, part2 := r.fn(puzzle)
	elapsed := time.Since(start)
	fmt.Printf("PART1: %v\nPART2: %v\n", part1, part2)
	fmt.Printf("Program took %s\n", elapsed)
}

// Probe ...
func (r *Runner) Probe(verbose bool) {
	if r.ts != nil {
		r.ts.Probe(r.fn, !verbose)
	}
}

// TestCases ...
type TestCases []TestCase

// Probe ...
func (t TestCases) Probe(fn RunFn, hideInput bool) {
	for _, test := range t {
		test.Probe(fn, hideInput)
	}
}

// TestCase ...
type TestCase struct {
	Input                        string
	ExpectedPart1, ExpectedPart2 interface{}
}

// Probe ...
func (t TestCase) Probe(fn RunFn, hideInput bool) {
	part1I, part2I := fn(t.Input)
	part1, expectedPart1, part2, expectedPart2 := fmt.Sprint(part1I),
		fmt.Sprint(t.ExpectedPart1),
		fmt.Sprint(part2I),
		fmt.Sprint(t.ExpectedPart2)
	passedPart1 := part1 == expectedPart1 || expectedPart1 == ""
	passedPart2 := part2 == expectedPart2 || expectedPart2 == ""
	passed := passedPart1 && passedPart2

	if !passed && !hideInput {
		fmt.Println("Input ", t.Input)
	}
	if !passedPart1 {
		fmt.Println(" - PART1: ", part1, " but expected ", expectedPart1)
		os.Exit(1)
	}
	if !passedPart2 {
		fmt.Println(" - PART2: ", part2, " but expected ", expectedPart2)
		os.Exit(1)
	}
}
