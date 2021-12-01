package main
import(
    "fmt"
    "os"
    "bufio"
    "strconv"
)
func addNums(nums ...int) int {
    sum := 0
    for _, num := range nums {
        sum += num
    }
    return sum
}

func main(){
    file, err := os.Open("input")
    increased := 0
    if err != nil {
        panic(err)
    }
    var firstRow []int
    scanner := bufio.NewScanner(file)
    scanner.Split(bufio.ScanLines)
    for i:=0; i < 3; i++ {
        scanner.Scan()
        value, _ := strconv.Atoi(scanner.Text())
        firstRow = append(firstRow, value)
    }
    secondRow := firstRow[1:]
    for scanner.Scan() {
        i, _ := strconv.Atoi(scanner.Text())
        secondRow = append(secondRow, i)
        if addNums(secondRow...) > addNums(firstRow...) {
            increased++
        }
        firstRow = secondRow
        secondRow = firstRow[1:]
    }
    fmt.Println(increased)
}

