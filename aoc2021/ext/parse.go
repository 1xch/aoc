package ext

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const sep = "\n"

func ParseToStringList(path string) []string {
	content, err := ioutil.ReadFile(path)
	Check(err)
	tcontent := strings.TrimSuffix(string(content), sep)
	return strings.Split(tcontent, sep)
}

func ParseToIntList(path string) []int {
	lines := ParseToStringList(path)
	list := make([]int, len(lines))
	for i, line := range lines {
		if line != "" {
			nb, err := strconv.Atoi(line)
			Check(err)
			list[i] = nb
		}
	}

	return list
}

//func ParseIntMap(s, sep string) map[int]int {
//	m := make(map[int]int)
//	for i, line := range strings.Split(s, sep) {
//		nb, err := strconv.Atoi(line)
//		Check(err)
//		m[i] = nb
//	}
//	return m
//}

func MustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	Check(err)
	return i
}

func MustScanf(line, format string, a ...interface{}) {
	n, err := fmt.Sscanf(line, format, a...) // don't forget to set parseCount
	Check(err)
	if n != len(a) {
		panic(fmt.Errorf("%d args expected in scanf, got %d", len(a), n))
	}
}

func Check(err error) {
	if err != nil {
		panic(err)
	}
}
