package main

import (
	"database/sql"
	"fmt"
	"io"
	"log"

	"github.com/gin-gonic/gin"
	_ "github.com/lib/pq"
)

type Env struct {
	DB *sql.DB
}

type DBRecord struct {
	ID          int
	FirstName   string
	LastName    string
	Age         int
	Salary      float64
	BirthDate   string
	IsActive    bool
	Email       string
	PhoneNumber string
	Address     string
}

const (
	host     = "localhost"
	port     = 5432
	user     = "postgres"
	password = "postgres"
	dbname   = "postgres"
)

func ConnectDB() (*sql.DB, error) {
	connString := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname = %s sslmode=disable", host, port, user, password, dbname)
	db, err := sql.Open("postgres", connString)
	if err != nil {
		log.Printf("failed to connect to database: %v", err)
		return &sql.DB{}, err
	}
	return db, nil
}

func main() {
	env := new(Env)
	var err error
	env.DB, err = ConnectDB()
	if err != nil {
		log.Fatalf("failed to start the server: %v", err)
	}

	r := gin.Default()
	r.GET("/string/", func(c *gin.Context) {
		c.String(200, "Hello World!")
	})

	r.GET("/simple-json/", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"key1": "value1",
			"key2": "value2",
			"key3": "value3",
			"key_nest": gin.H{
				"kn1": "value_nest_1",
				"knn2": gin.H{
					"key": "value",
				},
			},
		})
	})

	r.GET("/query-params/", func(c *gin.Context) {
		param1 := c.Query("param1")
		param2 := c.Query("param2")

		c.String(200, fmt.Sprintf("%s, %s", param1, param2))
	})

	r.GET("/sql-select/", func(c *gin.Context) {
		rows, err := env.DB.Query("SELECT * FROM public.exampletable ORDER BY id ASC")
		if err != nil {
			log.Printf("failed to execute query: %v", err)
			c.String(500, "failed to execute query")
			return
		}
		defer rows.Close()

		var records []DBRecord
		for rows.Next() {
			var record DBRecord
			err := rows.Scan(&record.ID, &record.FirstName, &record.LastName, &record.Age, &record.Salary, &record.BirthDate, &record.IsActive, &record.Email, &record.PhoneNumber, &record.Address)
			if err != nil {
				log.Printf("failed to scan row: %v", err)
				c.String(500, "failed to scan row")
				return
			}
			records = append(records, record)
		}

		c.JSON(200, records)

	})

	r.POST("/file-upload/", func(c *gin.Context) {
		file, err := c.FormFile("file")
		if err != nil {
			log.Printf("failed to get file: %v", err)
			c.String(500, "failed to get file")
			return
		}

		fileHandle, err := file.Open()

		if err != nil {
			log.Printf("failed to open file: %v", err)
			c.String(500, "failed to open file")
			return
		}

		content, err := io.ReadAll(fileHandle)
		if err != nil {
			log.Printf("failed to read file: %v", err)
			c.String(500, "failed to read file")
			return
		}

		c.String(200, string(content))
	})
	r.Run("localhost:8080")
}
