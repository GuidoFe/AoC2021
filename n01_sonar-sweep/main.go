package main
import(
    "fmt"
    "os"
    "bufio"
    "strconv"
)
func main(){
    file, err := os.Open("input")
    if err != nil {
        panic(err)
    }
    scanner := bufio.NewScanner(file)
    scanner.Split(bufio.ScanLines)
    scanner.Scan()
    increased := 0
    prev, _ := strconv.Atoi(scanner.Text())
    for scanner.Scan() {
        i, _ := strconv.Atoi(scanner.Text())
        if i > prev {
            increased++
        }
        prev = i
    }
    fmt.Println(increased)
}
