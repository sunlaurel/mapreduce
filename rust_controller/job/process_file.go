// Use this file to process a parquet file
package main

import (
	"fmt"
	"os"

	"github.com/segmentio/parquet-go"
)

type RowType struct {
	MinTemp		float64		`parquet:"MinTemp"`
	MaxTemp		float64		`parquet:"MixTemp"`
}

func main() {
	fmt.Println("Hello world!")
	file, err := os.Open("../data/weather.parquet")
	
	if err != nil {
		fmt.Println(err.Error())
	}

	defer file.Close()

	reader := parquet.NewReader(file)

	var rows []RowType
	err = reader.Read(&rows)
	if err != nil {
		fmt.Println(err.Error())
	}

	for _, row := range rows {
		fmt.Printf("max temp: %f, min temp: %f", row.MaxTemp, row.MinTemp)
	}
}