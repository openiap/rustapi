package main

import (
	"bufio"
	"fmt"
	"log"
	"math/rand"
	"openiap"
	"os"
	"strings"
	"time"
)

type handler struct {
	ticker *time.Ticker
	done   chan bool
}

func startGauge(name string, setValue func()) *handler {
	h := &handler{
		ticker: time.NewTicker(30 * time.Second),
		done:   make(chan bool),
	}

	go func() {
		setValue() // Initial set
		for {
			select {
			case <-h.done:
				return
			case <-h.ticker.C:
				setValue()
			}
		}
	}()

	return h
}

func (h *handler) stop() {
	h.ticker.Stop()
	h.done <- true
}

func main() {
	// Create the client
	client, err := openiap.NewClient()
	if err != nil {
		log.Fatalf("Error creating client: %v", err)
	}
	defer client.Free()

	// Enable tracing
	openiap.EnableTracing("info", "")

	// Connect to the server
	err = client.Connect("")
	if err != nil {
		log.Fatalf("Connection failed: %v", err)
	}
	fmt.Println("Connected successfully")

	fmt.Println("? for help")
	reader := bufio.NewReader(os.Stdin)

	var f64Handler, u64Handler, i64Handler *handler

	for {
		fmt.Print("> ")
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Printf("Error reading input: %v\n", err)
			continue
		}

		command := strings.TrimSpace(input)

		switch command {
		case "quit", "exit":
			// Clean up handlers before exit
			if f64Handler != nil {
				openiap.DisableObservableGauge("test_f64")
				f64Handler.stop()
			}
			if u64Handler != nil {
				openiap.DisableObservableGauge("test_u64")
				u64Handler.stop()
			}
			if i64Handler != nil {
				openiap.DisableObservableGauge("test_i64")
				i64Handler.stop()
			}
			return
		case "?", "help":
			fmt.Println("Available commands:")
			fmt.Println("  q     - Query entities")
			fmt.Println("  i     - Insert document")
			fmt.Println("  w     - Watch collection")
			fmt.Println("  o     - Start/Stop observable gauge (float64)")
			fmt.Println("  o2    - Start/Stop observable gauge (uint64)")
			fmt.Println("  o3    - Start/Stop observable gauge (int64)")
			fmt.Println("  quit  - Exit program")
			fmt.Println("  ?     - Show this help")
		case "q":
			req := openiap.QueryRequest{
				Collection: "entities",
				Query:      "{}",
				Projection: "{\"name\": 1}",
				Top:        10,
			}
			result, err := client.Query(req)
			if err != nil {
				fmt.Printf("Query failed: %v\n", err)
			} else {
				fmt.Printf("Query result: %s\n", result)
			}
		case "i":
			req := openiap.InsertOneRequest{
				Collection: "entities",
				Item:       `{"name":"Test", "_type":"test"}`,
				W:          1,
			}
			result, err := client.InsertOne(req)
			if err != nil {
				fmt.Printf("Insert failed: %v\n", err)
			} else {
				fmt.Printf("Insert result: %s\n", result)
			}
		case "w":
			req := openiap.WatchRequest{
				Collection: "entities",
				Paths:      "",
			}
			watchID, err := client.Watch(req)
			if err != nil {
				fmt.Printf("Watch failed: %v\n", err)
				break
			}
			fmt.Printf("Watch created with ID: %s\n", watchID)

			// Start watching for events in a goroutine
			go func() {
				for {
					event, err := client.NextWatchEvent(watchID)
					if err != nil {
						fmt.Printf("Watch error: %v\n", err)
						return
					}
					if event != nil {
						fmt.Printf("Watch event: %s %s\n", event.Operation, event.Document)
					}
					time.Sleep(200 * time.Millisecond)
				}
			}()
		case "o":
			if f64Handler != nil {
				openiap.DisableObservableGauge("test_f64")
				f64Handler.stop()
				f64Handler = nil
				fmt.Println("Stopped test_f64")
				break
			}
			openiap.SetF64ObservableGauge("test_f64", 42.7, "test")
			fmt.Println("Started test_f64 to 42.7")
			f64Handler = startGauge("test_f64", func() {
				value := rand.Float64() * 50
				fmt.Printf("Setting test_f64 to %f\n", value)
				openiap.SetF64ObservableGauge("test_f64", value, "test")
			})
		case "o2":
			if u64Handler != nil {
				openiap.DisableObservableGauge("test_u64")
				u64Handler.stop()
				u64Handler = nil
				fmt.Println("Stopped test_u64")
				break
			}
			openiap.SetU64ObservableGauge("test_u64", 42, "test")
			fmt.Println("Started test_u64 to 42")
			u64Handler = startGauge("test_u64", func() {
				value := uint64(rand.Int63n(50))
				fmt.Printf("Setting test_u64 to %d\n", value)
				openiap.SetU64ObservableGauge("test_u64", value, "test")
			})
		case "o3":
			if i64Handler != nil {
				openiap.DisableObservableGauge("test_i64")
				i64Handler.stop()
				i64Handler = nil
				fmt.Println("Stopped test_i64")
				break
			}
			openiap.SetI64ObservableGauge("test_i64", 42, "test")
			fmt.Println("Started test_i64 to 42")
			i64Handler = startGauge("test_i64", func() {
				value := rand.Int63n(50)
				fmt.Printf("Setting test_i64 to %d\n", value)
				openiap.SetI64ObservableGauge("test_i64", value, "test")
			})
		default:
			fmt.Println("Unknown command. Type ? for help")
		}
	}
}
