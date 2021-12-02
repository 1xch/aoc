package main

import (
	"ext"
)

func run(input string) (interface{}, interface{}) {
	data := ext.ParseToIntList(input)
	part1 := isHigherCount(data)
	part2 := isHigherCountWindow(window(data))
	return part1, part2
}

func isHigherCount(d []int) int {
	cnt := 0
	l := len(d) - 1
	for i, _ := range d {
		next := i + 1
		if next > l {
			break
		}
		if isHigher(d[i], d[next]) {
			cnt = cnt + 1
		}
	}
	return cnt
}

func isHigher(a, b int) bool {
	return b > a
}

func window(d []int) [][3]int {
	l := len(d) - 1
	ret := [][3]int{}
	for i, _ := range d {
		secondth := i + 1
		thirdth := i + 2
		if secondth > l || thirdth > l {
			break
		}
		n := [3]int{d[i], d[secondth], d[thirdth]}
		ret = append(ret, n)
	}
	return ret
}

func isHigherCountWindow(d [][3]int) int {
	cnt := 0
	l := len(d) - 1
	for i, _ := range d {
		next := i + 1
		if next > l {
			break
		}
		if isHigherWindow(d[i], d[next]) {
			cnt = cnt + 1
		}
	}
	return cnt
}

func isHigherWindow(a, b [3]int) bool {
	sumA := a[0] + a[1] + a[2]
	sumB := b[0] + b[1] + b[2]
	return sumB > sumA
}

//func parse(s string) {
//	lines := strings.Split(s, "\n")
//	for _, line := range lines {
//		pkg.MustScanf(line, "")
//	}
//	return
//}

func main() {
	r := ext.NewRunner(run, nil)
	r.Run("input", true)
}
