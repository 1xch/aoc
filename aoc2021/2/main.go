package main

import (
	"ext"
	"strings"
)

type Dir int8

const (
	unknown Dir = 0
	up          = 1
	down        = 2
	forward     = 3
)

func stringToDir(d string) Dir {
	ret := unknown
	c := strings.ToLower(d)
	switch c {
	case "up":
		ret = up
	case "down":
		ret = down
	case "forward":
		ret = forward
	}
	return ret
}

func dirToString(d Dir) string {
	ret := "unknown"
	switch d {
	case up:
		ret = "up"
	case down:
		ret = "down"
	case forward:
		ret = "forward"
	}
	return ret
}

type Instruction struct {
	order     int
	direction Dir
	value     int
}

func parseInstructions(path string) []Instruction {
	ret := []Instruction{}
	slist := ext.ParseToStringList(path)
	for o, s := range slist {
		spl := strings.Split(s, " ")
		id := stringToDir(spl[0])
		vv := ext.MustAtoi(spl[1])
		ret = append(ret, Instruction{o, id, vv})
	}
	return ret
}

type positionFn func(accumulator, Instruction) accumulator

func basicPosition(a accumulator, i Instruction) accumulator {
	switch i.direction {
	case up:
		a[1] = a[1] + i.value
	case down:
		a[1] = a[1] - i.value
	case forward:
		a[0] = a[0] + i.value
	}
	return a
}

func aimPosition(a accumulator, i Instruction) accumulator {
	switch i.direction {
	case up: // decrease aim
		a[2] = a[2] - i.value
	case down: // increase aim
		a[2] = a[2] + i.value
	case forward: // change horizontal, (depth * aim)
		a[0] = a[0] + i.value
		a[1] = a[1] + (i.value * a[2])
	}
	return a
}

type accumulator [3]int

func position(ins []Instruction, fn positionFn) accumulator {
	accum := [3]int{0, 0, 0}
	for _, i := range ins {
		accum = fn(accum, i)
	}
	return accum
}

func positionCross(ins []Instruction, fn positionFn) int {
	accum := position(ins, fn)
	return accum[0] * accum[1]
}

func run(input string) (interface{}, interface{}) {
	ins := parseInstructions(input)
	part1 := positionCross(ins, basicPosition)
	part2 := positionCross(ins, aimPosition)
	return part1, part2
}

func main() {
	r := ext.NewRunner(run, nil)
	r.Run("input", true)
}
