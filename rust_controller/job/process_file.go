package main
import (
	"fmt"
	"log"

	"github.com/xitongsys/parquet-go-source/local"
	"github.com/xitongsys/parquet-go/reader"
)
type Weather struct {
	// MaxTemp float64 `parquet:"name=MaxTemp, type=DOUBLE"`
	// MinTemp  float64  `parquet:"name=MinTemp, type=DOUBLE"`
	// WindSpeed3pm int64 `parquet:"name=WindSpeed3pm, type=INT64"`
	WindGustDir string `parquet:"name=WindGustDir, type=BYTE_ARRAY"`
}

func main() {
	fr, err := local.NewLocalFileReader("../data/weather.parquet")
	if err != nil {
		log.Fatal(err)
	}
	defer fr.Close()

	pr, err := reader.NewParquetReader(fr, new(Weather), 4)
	if err != nil {
		log.Fatal(err)
	}
	defer pr.ReadStop()

	num := int(pr.GetNumRows())
	fmt.Println("Total rows:", num)

	weather := make([]Weather, num)
	if err = pr.Read(&weather); err != nil {
		log.Fatal(err)
	}

	for _, w := range weather {
		fmt.Println(w.WindGustDir)
	}
}